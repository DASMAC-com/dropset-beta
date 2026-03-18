use std::path::Path;

use dropset_build::inject;
use dropset_interface::INJECTION_GROUPS;

const ASM_DIR: &str = "program/src/dropset";

fn main() {
    inject(Path::new(ASM_DIR), INJECTION_GROUPS);
    println!("cargo::rerun-if-changed=interface/src/lib.rs");
    println!("cargo::rerun-if-changed=macros/src");
    println!("cargo::rerun-if-changed=build/src");
    println!("cargo::rerun-if-changed={ASM_DIR}");
}
