extern crate askama;
extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro2::{Ident, Span, TokenStream};
use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    env_logger::init();

    // Handle template recompilation
    askama::rerun_if_templates_changed();

    // Build the bindings
    let output_file = Path::new(&env::var("OUT_DIR").unwrap()).join("models.rs");
    let mut outfile = File::create(output_file).unwrap();
    let model_definition_path = "model_descriptions/1.0.1.json";
    generate_model_bindings(model_definition_path, &mut outfile)
}

fn generate_model_bindings<P>(filename: P, output_file: &mut File) -> Result<()>
where
    P: AsRef<Path>,
{
    let content = fs::read_to_string(filename)?;
    let json_content: Value = serde_json::from_str(&content)?;
    let d = json_content.as_object().unwrap();

    for (class_name, body) in d.iter() {
        let model_definition = ModelDefinition::new(class_name, body);
        writeln!(output_file, "{}", model_definition.into_token_stream()?);
    }
    Ok(())
}

struct ModelDefinition {
    name: String,
    body: Value,
}

impl ModelDefinition {
    fn new(name: &str, body: &Value) -> ModelDefinition {
        ModelDefinition {
            name: name.into(),
            body: body.clone(),
        }
    }

    fn into_token_stream(self) -> Result<TokenStream> {
        let class_name = self.class_name();
        let props = self.props()?;

        // The final compilation of the model
        Ok(quote! {
            #[derive(Debug, Clone, Default)]
            pub struct #class_name {
                #(#props)*
            }

            impl #class_name {}
        })
    }

    fn class_name(&self) -> Ident {
        ident(&self.name)
    }

    fn props(&self) -> Result<Vec<TokenStream>> {
        let props = &self.body["props"];
        assert!(props.is_array());
        let props: &Vec<_> = props.as_array().unwrap();

        Ok(props
            .iter()
            .map(|p| {
                assert!(p.is_object());
                let p = p.as_object().unwrap();
                let name = ident(p["name"].as_str().unwrap());
                let typ = p["type"].as_str().unwrap();
                let converted_type = self.parse_type(typ);

                quote! {
                    #name: #converted_type,
                }
            }).collect())
    }

    fn parse_type(&self, typ: &str) -> Ident {
        if typ.starts_with("Dict") {
            ident("u32")
        } else {
            ident("String")
        }
    }
}

fn ident<S>(txt: S) -> Ident
where
    S: AsRef<str>,
{
    Ident::new(txt.as_ref(), Span::call_site())
}
