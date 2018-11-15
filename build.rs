extern crate askama;
extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate quote;
extern crate proc_macro2;
extern crate semver;

use proc_macro2::{Ident, Span, TokenStream};
use semver::Version;
use serde_json::Value;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

static MODELS_SRC_DIR: &str = "model_descriptions";

fn main() {
    env_logger::init();

    let mut envar_file = File::create("ENVLOG").unwrap();
    for (key, value) in env::vars() {
        writeln!(envar_file, "{} {}", key, value);
    }

    // Handle template recompilation
    askama::rerun_if_templates_changed();

    build_models_from_definitions();
}

fn build_models_from_definitions() {
    let model_files = fs::read_dir(MODELS_SRC_DIR).expect("fetching_files");
    for entry in model_files {
        let entry = entry.unwrap();
        let path = entry.path();
        let entry_str = path.to_str().unwrap();
        let version_str = entry_str.split("/").last().unwrap().replace(".json", "");
        if let Ok(version) = Version::parse(&version_str) {
            if version.is_prerelease() {
                continue;
            }

            build_model(&path);
        }
    }
}

fn build_model(path: &PathBuf) {
    let content = fs::read_to_string(path).unwrap();
    let json_content = serde_json::from_str(&content).unwrap();
    generate_models(json_content);
}

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

fn generate_model(class_name: &str, _body: &Value) -> TokenStream {
    let class_name = Ident::new(class_name, Span::call_site());

    let tokens = quote! {
        struct #class_name;
    };
    tokens
}
