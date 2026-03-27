use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

/// Field names registered by `#[signer_seeds]`, keyed by struct name.
static SIGNER_SEEDS: LazyLock<Mutex<HashMap<String, Vec<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Field names registered by `cpi_accounts!`, keyed by struct name.
static CPI_ACCOUNTS: LazyLock<Mutex<HashMap<String, Vec<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Metadata registered by `#[frame]`, keyed by struct name.
struct FrameInfo {
    /// Field name → type name mappings (e.g. `("pda_seeds", "PdaSignerSeeds")`).
    fields: Vec<(String, String)>,
    /// Doc comment from the frame struct, used as the default group header.
    doc: String,
}

static FRAME_INFO: LazyLock<Mutex<HashMap<String, FrameInfo>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Store the field names of a `#[signer_seeds]` struct.
pub fn register_signer_seeds(struct_name: &str, fields: Vec<String>) {
    SIGNER_SEEDS
        .lock()
        .unwrap()
        .insert(struct_name.to_string(), fields);
}

/// Store the field names of a `cpi_accounts!` struct.
pub fn register_cpi_accounts(struct_name: &str, fields: Vec<String>) {
    CPI_ACCOUNTS
        .lock()
        .unwrap()
        .insert(struct_name.to_string(), fields);
}

/// Store the metadata of a `#[frame]` struct.
pub fn register_frame(struct_name: &str, fields: Vec<(String, String)>, doc: String) {
    FRAME_INFO
        .lock()
        .unwrap()
        .insert(struct_name.to_string(), FrameInfo { fields, doc });
}

/// Look up the doc comment registered by `#[frame]`.
pub fn lookup_frame_doc(frame_name: &str) -> Option<String> {
    let info = FRAME_INFO.lock().unwrap();
    info.get(frame_name)
        .map(|i| i.doc.clone())
        .filter(|d| !d.is_empty())
}

/// Look up the signer seed field names for a parent field on a frame struct.
///
/// Resolves `frame_name.parent_field` → type name → signer seed fields.
pub fn lookup_signer_seed_fields(
    frame_name: &str,
    parent_field: &str,
) -> Result<Vec<String>, String> {
    let frame_info = FRAME_INFO.lock().unwrap();
    let info = frame_info.get(frame_name).ok_or_else(|| {
        format!(
            "frame struct `{f}` not found. `#[frame]` structs must be defined \
             before the `constant_group!` that references them (proc macros \
             execute in source order within a file, and in dependency order \
             across crates).",
            f = frame_name,
        )
    })?;

    let (_, type_name) = info
        .fields
        .iter()
        .find(|(name, _)| name == parent_field)
        .ok_or_else(|| {
            format!(
                "field `{}` not found on frame struct `{}`",
                parent_field, frame_name,
            )
        })?;

    let signer_seeds = SIGNER_SEEDS.lock().unwrap();
    signer_seeds.get(type_name).cloned().ok_or_else(|| {
        format!(
            "type `{t}` (field `{f}`) is not annotated with `signer_seeds!`. \
             The `signer_seeds!` invocation must appear before the \
             `constant_group!` that references it.",
            t = type_name,
            f = parent_field,
        )
    })
}

/// Look up the CPI account field names for a parent field on a frame struct.
///
/// Resolves `frame_name.parent_field` → type name → CPI account fields.
pub fn lookup_cpi_account_fields(
    frame_name: &str,
    parent_field: &str,
) -> Result<Vec<String>, String> {
    let frame_info = FRAME_INFO.lock().unwrap();
    let info = frame_info.get(frame_name).ok_or_else(|| {
        format!(
            "frame struct `{f}` not found. `#[frame]` structs must be defined \
             before the `constant_group!` that references them (proc macros \
             execute in source order within a file, and in dependency order \
             across crates).",
            f = frame_name,
        )
    })?;

    let (_, type_name) = info
        .fields
        .iter()
        .find(|(name, _)| name == parent_field)
        .ok_or_else(|| {
            format!(
                "field `{}` not found on frame struct `{}`",
                parent_field, frame_name,
            )
        })?;

    let cpi_accounts = CPI_ACCOUNTS.lock().unwrap();
    cpi_accounts.get(type_name).cloned().ok_or_else(|| {
        format!(
            "type `{t}` (field `{f}`) is not annotated with `cpi_accounts!`. \
             The `cpi_accounts!` invocation must appear before the \
             `constant_group!` that references it.",
            t = type_name,
            f = parent_field,
        )
    })
}
