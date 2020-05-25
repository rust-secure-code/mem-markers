extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod utils;

macro_rules! define_derive {
    ($item:meta => $mod:ident) => {
        mod $mod;
        #[proc_macro_derive($item)]
        pub fn $mod(item: TokenStream) -> TokenStream {
            let input = parse_macro_input!(item as DeriveInput);
            $mod::expand(input)
                .unwrap_or_else(|err| err.to_compile_error())
                .into()
        }
    };
}

define_derive!(FixedLayout => fixed_layout);
define_derive!(ByteComplete => byte_complete);
define_derive!(FromBytes => from_bytes);
define_derive!(NoUninit => no_uninit);
define_derive!(AsBytes => as_bytes);
define_derive!(Zeroable => zeroable);
define_derive!(InvariantFree => invariant_free);
