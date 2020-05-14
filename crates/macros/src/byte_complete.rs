use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let type_name = &input.ident;
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "ByteComplete on types with generics is not currently supported",
        ));
    }
    let ensure = match &input.data {
        syn::Data::Struct(s) => ensure_struct_is_byte_complete(type_name, s)?,
        syn::Data::Enum(e) => ensure_enum_is_byte_complete(type_name, e)?,
        syn::Data::Union(u) => ensure_union_is_byte_complete(type_name, u)?,
    };

    let stream = quote! {
        #ensure
        unsafe impl ::mem_markers::ByteComplete for #type_name {}
    };
    Ok(stream)
}

fn ensure_struct_is_byte_complete(
    type_name: &syn::Ident,
    s: &syn::DataStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_types = crate::utils::struct_field_types(s);
    if field_types.is_empty() {
        return Ok(quote! {});
    }
    let stream = crate::utils::ensure_field_types(
        field_types,
        type_name,
        &quote::format_ident!("ByteComplete"),
        None,
    );

    Ok(stream.into())
}

fn ensure_enum_is_byte_complete(
    _type_name: &syn::Ident,
    _e: &syn::DataEnum,
) -> syn::Result<proc_macro2::TokenStream> {
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "ByteComplete on enums is not currently supported",
    ));
}

fn ensure_union_is_byte_complete(
    _type_name: &syn::Ident,
    _u: &syn::DataUnion,
) -> syn::Result<proc_macro2::TokenStream> {
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "ByteComplete on unions is not currently supported",
    ));
}
