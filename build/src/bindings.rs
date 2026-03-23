use std::path::Path;

/// Base URL template for raw Agave headers on GitHub.
const AGAVE_RAW_BASE: &str = "https://raw.githubusercontent.com/anza-xyz/agave";
const AGAVE_RAW_PATH: &str = "platform-tools-sdk/sbf/c/inc/sol";

/// Headers to fetch, in dependency order.
const HEADERS: &[&str] = &[
    "types.h",
    "constants.h",
    "pubkey.h",
    "entrypoint.h",
    "cpi.h",
];

/// Structs to keep in the generated output.
const ALLOWLIST: &[&str] = &[
    "SolInstruction",
    "SolAccountMeta",
    "SolAccountInfo",
    "SolSignerSeed",
    "SolSignerSeeds",
];

/// Fetch Agave C headers from GitHub, run bindgen, and write Rust FFI
/// bindings to `output`. All `SolPubkey` references are replaced with
/// `pinocchio::Address`.
///
/// Reads `AGAVE_REV` from the environment (set via Makefile).
pub fn generate_bindings(output: &Path) {
    let rev = std::env::var("AGAVE_REV").expect("AGAVE_REV env var not set");

    let dir = std::env::temp_dir()
        .join("dropset-agave-headers")
        .join("sol");
    std::fs::create_dir_all(&dir).expect("failed to create temp header dir");

    // Fetch each header.
    for name in HEADERS {
        let url = format!("{AGAVE_RAW_BASE}/{rev}/{AGAVE_RAW_PATH}/{name}");
        let body: String = ureq::get(&url)
            .call()
            .expect(&url)
            .into_body()
            .read_to_string()
            .expect(&url);
        std::fs::write(dir.join(name), body).expect("failed to write header");
    }

    let inc_dir = dir.parent().unwrap();
    let wrapper = "#include <stdbool.h>\n#include <sol/cpi.h>\n";

    let mut builder = bindgen::Builder::default()
        .header_contents("wrapper.h", wrapper)
        .clang_arg(format!("-I{}", inc_dir.display()))
        .clang_arg("-std=c23")
        .use_core()
        .layout_tests(false)
        .generate_comments(false);

    for ty in ALLOWLIST {
        builder = builder.allowlist_type(ty);
    }

    let bindings = builder.generate().expect("bindgen failed").to_string();

    // Strip the SolPubkey struct and its attrs, replace remaining references
    // with Address. Uses a single buffered pass so orphaned attrs never appear.
    let mut out = String::from("#![allow(clippy::all)]\n\nuse pinocchio::Address;\n\n");
    let mut attr_buf: Vec<String> = Vec::new();
    let mut skip_body = false;

    for line in bindings.lines() {
        // Skip the bindgen version comment.
        if line.starts_with("/* automatically generated") {
            continue;
        }

        if skip_body {
            if line == "}" {
                skip_body = false;
            }
            continue;
        }

        let trimmed = line.trim();
        if trimmed.starts_with("#[repr") || trimmed.starts_with("#[derive") {
            attr_buf.push(line.replace("SolPubkey", "Address"));
            continue;
        }

        if trimmed.contains("pub struct SolPubkey") {
            attr_buf.clear();
            skip_body = true;
            continue;
        }

        // Flush any buffered attrs — they belong to this (non-SolPubkey) item.
        for attr in attr_buf.drain(..) {
            out.push_str(&attr);
            out.push('\n');
        }
        out.push_str(&line.replace("SolPubkey", "Address"));
        out.push('\n');
    }

    std::fs::write(output, &out)
        .unwrap_or_else(|e| panic!("failed to write {}: {}", output.display(), e));

    // Run rustfmt on the generated file.
    let status = std::process::Command::new("rustfmt")
        .arg(output)
        .status()
        .expect("failed to run rustfmt");
    assert!(status.success(), "rustfmt failed on {}", output.display());
}
