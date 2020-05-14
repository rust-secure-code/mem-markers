use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let type_name = &input.ident;
    let attrs = &input.attrs;
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Zeroable on types with generics is not currently supported",
        ));
    }
    let ensure = match &input.data {
        syn::Data::Struct(s) => ensure_struct_has_stable_layout(type_name, s)?,
        syn::Data::Enum(e) => ensure_enum_has_stable_layout(type_name, e, attrs)?,
        syn::Data::Union(u) => ensure_union_has_stable_layout(type_name, u, attrs)?,
    };

    let stream = quote! {
        #ensure
        unsafe impl ::mem_markers::Zeroable for #type_name {}
    };
    Ok(stream)
}

fn ensure_struct_has_stable_layout(
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
        &quote::format_ident!("Zeroable"),
        None,
    );
    Ok(stream)
}

fn ensure_enum_has_stable_layout(
    _type_name: &syn::Ident,
    _e: &syn::DataEnum,
    _attrs: &Vec<syn::Attribute>,
) -> syn::Result<proc_macro2::TokenStream> {
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "Zeroable on enums is not currently supported",
    ));
}

fn ensure_union_has_stable_layout(
    _type_name: &syn::Ident,
    _u: &syn::DataUnion,
    _attrs: &Vec<syn::Attribute>,
) -> syn::Result<proc_macro2::TokenStream> {
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "Zeroable on unions is not currently supported",
    ));
}
