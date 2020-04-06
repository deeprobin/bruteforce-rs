#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;

use proc_macro::TokenStream;

use std::prelude::v1::*; // needed for std-compat
use std::collections::HashSet;

use quote::quote;

/// Converts an string into a slice
#[doc(hidden)]
#[proc_macro]
pub fn charset_string(item: TokenStream) -> TokenStream {
    assert!(!item.is_empty(), "Cannot parse empty string as charset");
    let chars: HashSet<char> = item.to_string().chars().collect(); // hashset, because duplicates are not permitted
    let iter = chars.into_iter();
    TokenStream::from(quote! {
        let slice = &[#(#iter),*];
    })
}