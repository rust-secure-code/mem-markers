extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive a Fixed layout for a type
///
/// ```
/// #[repr(C)]
/// struct Foo {
///     bar: u8
/// }
/// ```
#[proc_macro_derive(FixedLayout)]
pub fn derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let type_name = input.ident;
    let can_derive = input.attrs.iter().any(|attr| {
        let meta = attr.parse_meta().unwrap();
        match meta {
            syn::Meta::List(list) => {
                if !list.path.is_ident("repr") {
                    return false;
                }
                let first = list.nested.first().unwrap();
                match first {
                    syn::NestedMeta::Meta(syn::Meta::Path(p)) if p.is_ident("C") => {
                        return true;
                    }
                    _ => return false,
                }
            }
            _ => false,
        }
    });

    if !can_derive {
        panic!("NO!")
    }

    (quote! {
        unsafe impl mem_markers::FixedLayout for #type_name {}
    })
    .into()
}
