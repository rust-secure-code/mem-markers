use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Get the types of all fields of a struct
pub(crate) fn struct_field_types(s: &syn::DataStruct) -> Vec<&syn::Type> {
    match &s.fields {
        syn::Fields::Named(n) => n.named.iter().map(|f| &f.ty).collect(),
        syn::Fields::Unnamed(n) => n.unnamed.iter().map(|f| &f.ty).collect(),
        syn::Fields::Unit => vec![],
    }
}

pub(crate) fn ensure_field_types(
    field_types: Vec<&syn::Type>,
    type_name: &Ident,
    trait_name: &Ident,
    extra: Option<TokenStream>,
) -> TokenStream {
    let ensure_method_name = quote::format_ident!("r#{}Impls{}", type_name, trait_name);
    let extra = extra.unwrap_or(quote! {});
    quote! {
        #[allow(missing_docs)]
        #[allow(non_snake_case)]
        fn #ensure_method_name<T: ::mem_markers::#trait_name>() {
            #(
                #ensure_method_name::<#field_types>();
            )*
            #extra
        }
    }
}
