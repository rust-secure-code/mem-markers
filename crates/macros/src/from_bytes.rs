use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let type_name = &input.ident;
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "FromBytes on types with generics is not currently supported",
        ));
    }

    let stream = quote! {
        unsafe impl ::mem_markers::FromBytes for #type_name {}
    };
    Ok(stream)
}
