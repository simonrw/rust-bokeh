extern crate askama;
extern crate buildlib;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    // Handle template recompilation
    askama::rerun_if_templates_changed();

    // Build bindings
    buildlib::generate_bindings()
}
