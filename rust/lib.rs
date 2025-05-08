#![no_std]

extern crate proc_macro;
use core::str::FromStr;
use proc_macro::TokenStream;

#[proc_macro]
pub fn fam(_input: TokenStream) -> TokenStream {
    let stream = TokenStream::from_str("#![no_std]");
    stream.expect("parse token stream")
}
