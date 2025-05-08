#![no_std]

extern crate proc_macro;
use core::str::FromStr;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn fam(_input: TokenStream, _attrs: TokenStream) -> TokenStream {
    let stream = TokenStream::from_str("#![no_std]");
    stream.expect("parse token stream")
}
