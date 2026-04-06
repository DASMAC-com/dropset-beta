use syn::{Ident, parse::ParseStream};

use super::super::ConstantKind;
use crate::common::shared_state;

/// Parse the inside of `cpi_accounts!(parent_field)`.
///
/// The field list is resolved from shared state registered by `cpi_accounts!`
/// (on the struct) and `#[frame]` (on the frame struct).
pub fn parse_cpi_accounts(
    inner: ParseStream,
    frame_type: &Option<syn::Path>,
    span: proc_macro2::Span,
) -> syn::Result<ConstantKind> {
    let frame_path = frame_type.as_ref().ok_or_else(|| {
        syn::Error::new(span, "cpi_accounts! requires #[frame(Context)] attribute")
    })?;

    let frame_name = frame_path
        .segments
        .last()
        .map(|s| s.ident.to_string())
        .unwrap_or_default();

    let parent_field: Ident = inner.parse()?;

    let field_names =
        shared_state::lookup_cpi_account_fields(&frame_name, &parent_field.to_string())
            .map_err(|e| syn::Error::new(parent_field.span(), e))?;

    let accounts: Vec<Ident> = field_names
        .iter()
        .map(|name| Ident::new(name, parent_field.span()))
        .collect();

    Ok(ConstantKind::CpiAccounts {
        parent_field,
        accounts,
    })
}
