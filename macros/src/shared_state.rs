use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

/// Field names registered by `#[signer_seeds]`, keyed by struct name.
static SIGNER_SEEDS: LazyLock<Mutex<HashMap<String, Vec<String>>>> =
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
            "frame struct `{}` not found — ensure `#[frame]` is defined above `constant_group!`",
            frame_name,
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
            "type `{}` (field `{}`) is not annotated with `#[signer_seeds]`",
            type_name, parent_field,
        )
    })
}
