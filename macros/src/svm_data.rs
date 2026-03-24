use quote::quote;

/// Expand `#[svm_data]` on a struct into the struct with `#[repr(C, packed)]`.
pub fn expand(input: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let attrs = &input.attrs;
    let vis = &input.vis;
    let ident = &input.ident;
    let generics = &input.generics;
    let fields = &input.fields;
    let semi = &input.semi_token;

    quote! {
        #(#attrs)*
        #[repr(C, packed)]
        #vis struct #ident #generics #fields #semi
    }
}
