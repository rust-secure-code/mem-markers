use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let type_name = &input.ident;
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "NoUninit on types with generics is not currently supported",
        ));
    }
    let ensure = match &input.data {
        syn::Data::Struct(s) => ensure_struct_has_no_uninit(type_name, s)?,
        syn::Data::Enum(_e) => todo!(),
        syn::Data::Union(_u) => todo!(),
    };

    let stream = quote! {
        #ensure
        unsafe impl ::mem_markers::NoUninit for #type_name {}
    };
    Ok(stream)
}

fn ensure_struct_has_no_uninit(
    type_name: &syn::Ident,
    s: &syn::DataStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_types = crate::utils::struct_field_types(s);
    if field_types.is_empty() {
        return Ok(quote! {});
    }
    let ensure_no_padding = quote! {
        const HAS_PADDING: bool = core::mem::size_of::<#type_name>() != #(core::mem::size_of::<#field_types>())+*;
        let _: [(); 1/(1 - HAS_PADDING as usize)];
    };
    let ensure_fields_no_uninit = quote! {
        #(
            ::mem_markers::no_uninit::ensure::<#field_types>();
        )*
    };
    let ensure_method_name = quote::format_ident!("__ensure_no_uninit_for_{}", type_name);

    let stream = quote! {
        #[allow(missing_docs)]
        #[allow(non_snake_case)]
        fn #ensure_method_name() {
            #ensure_no_padding
            #ensure_fields_no_uninit
        }
    };
    Ok(stream)
}
