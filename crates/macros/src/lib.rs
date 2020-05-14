extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod as_bytes;
mod byte_complete;
mod fixed_layout;
mod from_bytes;
mod no_uninit;
mod utils;
mod zeroable;

macro_rules! define_derive {
    ($item:meta => $fun:ident => $expand:path) => {
        #[proc_macro_derive($item)]
        pub fn $fun(item: TokenStream) -> TokenStream {
            let input = parse_macro_input!(item as DeriveInput);
            $expand(input)
                .unwrap_or_else(|err| err.to_compile_error())
                .into()
        }
    };
}

define_derive!(FixedLayout => fixed_layout => fixed_layout::expand);
define_derive!(ByteComplete => byte_complete => byte_complete::expand);
define_derive!(FromBytes => from_bytes => from_bytes::expand);
define_derive!(NoUninit => no_uninit => no_uninit::expand);
define_derive!(AsBytes => as_bytes => as_bytes::expand);
define_derive!(Zeroable => zeroable => zeroable::expand);
