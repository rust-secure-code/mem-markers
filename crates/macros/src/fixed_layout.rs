use quote::quote;
use syn::DeriveInput;

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let type_name = &input.ident;
    let attrs = &input.attrs;
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "FixedLayout on types with generics is not currently supported",
        ));
    }
    let ensure = match &input.data {
        syn::Data::Struct(s) => ensure_struct_has_stable_layout(type_name, s, attrs)?,
        syn::Data::Enum(e) => ensure_enum_has_stable_layout(type_name, e, attrs)?,
        syn::Data::Union(u) => ensure_union_has_stable_layout(type_name, u, attrs)?,
    };

    let stream = quote! {
        #ensure
        unsafe impl ::mem_markers::FixedLayout for #type_name {}
    };
    Ok(stream)
}

fn ensure_struct_has_stable_layout(
    type_name: &syn::Ident,
    s: &syn::DataStruct,
    attrs: &Vec<syn::Attribute>,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_types = crate::utils::struct_field_types(s);
    if field_types.is_empty() {
        return Ok(quote! {});
    }

    if !has_stable_repr(attrs) {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "FixedLayout trait requires #[repr(C)] or #[repr(transparent)] on structs with fields",
        ));
    }

    let ensure = quote! {
        #(
            ::mem_markers::fixed_layout::ensure::<#field_types>();
        )*
    };
    let ensure_method_name = quote::format_ident!("__ensure_fixed_layout_for_{}", type_name);

    let stream = quote! {
        #[allow(missing_docs)]
        #[allow(non_snake_case)]
        fn #ensure_method_name() {
            #ensure
        }
    };
    Ok(stream)
}

fn ensure_enum_has_stable_layout(
    _type_name: &syn::Ident,
    _e: &syn::DataEnum,
    _attrs: &Vec<syn::Attribute>,
) -> syn::Result<proc_macro2::TokenStream> {
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "FixedLayout on enums is not currently supported",
    ));
}

fn ensure_union_has_stable_layout(
    _type_name: &syn::Ident,
    _u: &syn::DataUnion,
    _attrs: &Vec<syn::Attribute>,
) -> syn::Result<proc_macro2::TokenStream> {
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "FixedLayout on unions is not currently supported",
    ));
}

fn has_stable_repr(attrs: &Vec<syn::Attribute>) -> bool {
    for attr in attrs {
        if let Ok(syn::Meta::List(meta)) = attr.parse_meta() {
            if meta.path.is_ident("repr") && meta.nested.len() == 1 {
                if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = &meta.nested[0] {
                    if path.is_ident("C") || path.is_ident("transparent") {
                        return true;
                    }
                }
            }
        }
    }
    false
}
