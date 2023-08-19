extern crate proc_macro2;
extern crate quote;
extern crate syn;

#[cfg(not(precompiled))]
extern crate proc_macro;
#[cfg(precompiled)]
extern crate proc_macro2 as proc_macro;

mod internals;

use proc_macro::TokenStream;
#[cfg(precompiled)]
use std::sync::atomic::AtomicBool;
#[cfg(not(precompiled))]
use syn::parse_macro_input;
use syn::DeriveInput;

#[macro_use]
mod bound;
#[macro_use]
mod fragment;

mod de;
mod dummy;
mod pretend;
mod ser;
mod this;

#[cfg(precompiled)]
macro_rules! parse_macro_input {
    ($tokenstream:ident as $ty:ty) => {
        match syn::parse2::<$ty>($tokenstream) {
            Ok(data) => data,
            Err(err) => return err.to_compile_error(),
        }
    };
}

#[cfg(precompiled)]
pub static DESERIALIZE_IN_PLACE: AtomicBool = AtomicBool::new(false);

#[cfg_attr(not(precompiled), proc_macro_derive(Serialize, attributes(serde)))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    ser::expand_derive_serialize(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg_attr(not(precompiled), proc_macro_derive(Deserialize, attributes(serde)))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    de::expand_derive_deserialize(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
