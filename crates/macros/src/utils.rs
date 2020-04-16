/// Get the types of all fields of a struct
pub(crate) fn struct_field_types(s: &syn::DataStruct) -> Vec<&syn::Type> {
    match &s.fields {
        syn::Fields::Named(n) => n.named.iter().map(|f| &f.ty).collect(),
        syn::Fields::Unnamed(n) => n.unnamed.iter().map(|f| &f.ty).collect(),
        syn::Fields::Unit => vec![],
    }
}
