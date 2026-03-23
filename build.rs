use std::path::Path;

use dropset_build::{generate_bindings, inject};
use dropset_interface::INJECTION_GROUPS;

const ASM_DIR: &str = "program/src/dropset";
const CPI_BINDINGS: &str = "interface/src/cpi_bindings.rs";

fn main() {
    if std::env::var("AGAVE_REV").is_ok() {
        generate_bindings(Path::new(CPI_BINDINGS));
    }

    inject(Path::new(ASM_DIR), INJECTION_GROUPS);
    println!("cargo::rerun-if-changed=interface/src/lib.rs");
    println!("cargo::rerun-if-changed=macros/src");
    println!("cargo::rerun-if-changed=build/src");
}
