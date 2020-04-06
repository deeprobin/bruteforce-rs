#![cfg_attr(not(feature = "std"), no_std)]

extern crate proc_macro;

#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;

use proc_macro::TokenStream;

use std::prelude::v1::*; // needed for std-compat
use std::collections::HashSet;

/// Converts an string into a slice
#[doc(hidden)]
#[proc_macro]
pub fn charset_string(item: TokenStream) -> TokenStream {
    assert!(!item.is_empty(), "Cannot parse empty string as charset");
    let chars: HashSet<char> = item.to_string().chars().collect(); // hashset, because duplicates are not permitted
    let mut out_string = String::default();
    let len: usize = chars.len();
    out_string.push_str("&[");
    for (i, ch) in chars.into_iter().enumerate() {
        out_string.push(ch);
        if i != len {
            out_string.push_str(", ");
        }
    }
    out_string.push(']');

    out_string.parse().unwrap()
}