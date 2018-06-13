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
use std::collections::{HashMap, HashSet as Set};
use std::sync::{Arc, Mutex};

use syn::DeriveInput;

/// Describes the "lifetime" of a field in an API struct
struct VersionSpan {
    start: String,
    stop: String,
}

static mut attributes: Option<Arc<Mutex<HashMap<String, VersionSpan>>>> = None;

// #[proc_macro_attribute]
// pub fn versioning(args: TokenStream, input: TokenStream) -> TokenStream {
//     println!("Running the main versioning thingy");
//     input
// }

#[proc_macro_attribute]
pub fn version(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("Running a version command");
    input
}
