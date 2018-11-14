extern crate askama;
extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quote;
extern crate proc_macro2;

use proc_macro2::{Ident, Span, TokenStream};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

static PYTHON_PATH: &str = "venv/bin/python";
static BOKEH_SRC: &str = "bokeh-src/scripts/spec.py";

fn main() {
    env_logger::init();

    // Handle template recompilation
    askama::rerun_if_templates_changed();

    build_models_from_definitions();
}

fn build_models_from_definitions() {}

fn generate_models(description: Value) {
    let output_file = Path::new(&env::var("OUT_DIR").unwrap()).join("models.rs");
    let mut outfile = File::create(output_file).unwrap();
    let d = description.as_object().unwrap();

    for (class_name, body) in d.iter() {
        let tokens = generate_model(class_name, body);
        let token_str = tokens.to_string();
        writeln!(outfile, "{}", token_str);
    }
}

fn generate_model(class_name: &str, body: &Value) -> TokenStream {
    let class_name = Ident::new(class_name, Span::call_site());

    let tokens = quote! {
        struct #class_name;
    };
    tokens
}
