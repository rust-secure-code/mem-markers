extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod byte_complete;
mod fixed_layout;
mod utils;

/// Derive `FixedLayout` for a type
#[proc_macro_derive(FixedLayout)]
pub fn fixed_layout_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    fixed_layout::expand(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derive `ByteComplete` for a type
#[proc_macro_derive(ByteComplete)]
pub fn byte_complete_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    byte_complete::expand(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
