//! `thunder.rs` a zero-boilerplate commandline argument parser ✨
#![feature(external_doc)]
#![feature(proc_macro, proc_macro_lib, iterator_flatten, attr_literals)]
#![allow(unused_imports, unused_variables)]


extern crate proc_macro;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use std::collections::HashSet as Set;
use std::str::FromStr;

use syn::DeriveInput;

#[proc_macro_derive(Versioning,attributes(version))]
pub fn versioning(input: TokenStream) -> TokenStream {
    println!("{:#?}", syn::parse::<DeriveInput>(input.clone()).unwrap());


    let tokens = quote! {};

    tokens.into()
}