//! # extern_attrib
//! Rust automatic ABI attribute macro: #[extern_auto]
//! 
//! The purpose of this library is to replace or insert the ABI name at compile time for functions.
//! This is useful for hiding the calling convention for FFI callbacks while avoiding code duplication.
//! 
//! - __Requires nightly rust__, depends on #[proc_macro_attribute].
//! - Currenly only handles target_os = "windows" / "macos" / "linux".
//! - See the tests/tests.rs for a usage example.
//! - Pull Requests Welcome!
//! 
//! TODO: Investigate how error handling should best be done in procedural macros.
#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::str::FromStr;
use syn::ItemKind;

#[proc_macro_attribute]
pub fn extern_auto(_: TokenStream, input: TokenStream) -> TokenStream {
    let src = input.to_string();
    let mut item = match syn::parse_item(&src) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Parse error in extern_auto: {}", e);
            return input // Let rustc handle syntax errors.
        }
    };
    { //! This scope drops abi_opt after mutation.
        let abi_opt = match item.node {
            ItemKind::Fn(_, _, _, ref mut opt, _, _) => opt,
            _ => return input
        };
        modify_abi(abi_opt);
    }
    let tokens = quote!(#item).to_string();
    // Uncomment for debug help:
    // println!("tokens: {}", tokens);
    TokenStream::from_str(&tokens).unwrap() // TODO: Handle error.
}

fn modify_abi(abi: &mut Option<syn::Abi>) {
    let name = get_platform_abi_name();
    *abi = Some(syn::Abi::Named(name.into()));
}
    
#[cfg(target_os = "windows")]
fn get_platform_abi_name() -> &'static str {
    //! TODO: Investigate if rustc interprets "stdcall" as "C" for x64.
    "stdcall"
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn get_platform_abi_name() -> &'static str {
    "C"
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn get_platform_abi_name() -> &'static str {
    panic!("Platform not supported.");
}
