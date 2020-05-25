use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let type_name = &input.ident;
    let attrs = &input.attrs;
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "InvariantFree on types with generics is not currently supported",
        ));
    }
    let ensure = match &input.data {
        syn::Data::Struct(s) => ensure_struct_invariant_free(type_name, s)?,
        syn::Data::Enum(e) => ensure_enum_invariant_free(type_name, e, attrs)?,
        syn::Data::Union(u) => ensure_union_invariant_free(type_name, u, attrs)?,
    };

    let stream = quote! {
        #ensure
        unsafe impl ::mem_markers::InvariantFree for #type_name {}
    };
    Ok(stream)
}

fn ensure_struct_invariant_free(
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
        &quote::format_ident!("InvariantFree"),
        None,
    );
    Ok(stream)
}

fn ensure_enum_invariant_free(
    _type_name: &syn::Ident,
    _e: &syn::DataEnum,
    _attrs: &Vec<syn::Attribute>,
) -> syn::Result<proc_macro2::TokenStream> {
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "InvariantFree on enums is not currently supported",
    ));
}

fn ensure_union_invariant_free(
    _type_name: &syn::Ident,
    _u: &syn::DataUnion,
    _attrs: &Vec<syn::Attribute>,
) -> syn::Result<proc_macro2::TokenStream> {
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "InvariantFree on unions is not currently supported",
    ));
}
