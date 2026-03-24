mod immediate;
mod offset;
mod signer_seeds;

use super::ConstantKind;
use super::parse::ConstantGroupInput;
use crate::codegen;

/// Expand a parsed `ConstantGroupInput` into a module with constants and a GROUP.
pub fn expand(input: &ConstantGroupInput) -> proc_macro2::TokenStream {
    let mut const_defs = Vec::new();
    let mut meta_idents = Vec::new();

    for c in &input.constants {
        let doc = &c.doc;
        let base_name = &c.name;

        let asm_name = match &input.prefix {
            Some(p) => format!("{}_{}", p, base_name),
            None => base_name.to_string(),
        };

        match &c.kind {
            ConstantKind::Offset { negate, expr } => {
                let (def, meta) = offset::expand_offset(base_name, &asm_name, doc, *negate, expr);
                const_defs.push(def);
                meta_idents.push(meta);
            }
            ConstantKind::FrameOffset { fields } => {
                let frame_ty = input
                    .frame_type
                    .as_ref()
                    .expect("frame_type must be set for FrameOffset");
                let (def, meta) =
                    offset::expand_frame_offset(base_name, &asm_name, doc, frame_ty, fields);
                const_defs.push(def);
                meta_idents.push(meta);
            }
            ConstantKind::SignerSeeds {
                parent_field,
                seeds,
            } => {
                let frame_ty = input
                    .frame_type
                    .as_ref()
                    .expect("frame_type must be set for SignerSeeds");
                signer_seeds::expand_signer_seeds(
                    &asm_name,
                    frame_ty,
                    parent_field,
                    seeds,
                    &mut const_defs,
                    &mut meta_idents,
                );
            }
            ConstantKind::Immediate { expr } => {
                let (def, meta) = immediate::expand_immediate(base_name, &asm_name, doc, expr);
                const_defs.push(def);
                meta_idents.push(meta);
            }
        };
    }

    codegen::group_module(
        &input.mod_name,
        &input.target,
        &input.doc,
        &const_defs,
        &meta_idents,
    )
}
