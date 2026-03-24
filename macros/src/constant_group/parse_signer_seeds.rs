use syn::{
    Ident,
    parse::ParseStream,
};

use super::ConstantKind;
use crate::shared_state;

/// Parse the inside of `signer_seeds!(parent_field)`.
///
/// The field list is resolved from shared state registered by `signer_seeds!`
/// (on the struct) and `#[frame]` (on the frame struct).
pub fn parse_signer_seeds(
    inner: ParseStream,
    frame_type: &Option<syn::Path>,
    span: proc_macro2::Span,
) -> syn::Result<ConstantKind> {
    let frame_path = frame_type
        .as_ref()
        .ok_or_else(|| syn::Error::new(span, "signer_seeds! requires #[frame(Type)] attribute"))?;

    let frame_name = frame_path
        .segments
        .last()
        .map(|s| s.ident.to_string())
        .unwrap_or_default();

    let parent_field: Ident = inner.parse()?;

    let field_names = shared_state::lookup_signer_seed_fields(
        &frame_name,
        &parent_field.to_string(),
    )
    .map_err(|e| syn::Error::new(parent_field.span(), e))?;

    let seeds: Vec<Ident> = field_names
        .iter()
        .map(|name| Ident::new(name, parent_field.span()))
        .collect();

    Ok(ConstantKind::SignerSeeds {
        parent_field,
        seeds,
    })
}
