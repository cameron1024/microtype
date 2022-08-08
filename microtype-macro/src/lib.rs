//! proc-macro crate for `microtype`

#![warn(clippy::all)]
#![deny(missing_docs)]

use codegen::codegen;
use parse::MicrotypeMacro;
use syn::parse_macro_input;

use crate::model::flatten;

extern crate proc_macro;

mod parse;
mod model;
mod codegen;


/// Macro to create microtype wrappers
/// 
/// See crate-level documentation for a more thorough explanation
///
/// Example usage:
/// ```
/// # use microtype::microtype;
/// microtype! {
///   #[derive(Debug, Clone)]  // attributes on the outer type apply to all types in this block
///   String {
///     #[derive(PartialEq)]  // attributes can also be applied to a single microtype
///     Email,
///
///     NotPartialEqString,
///   }
///
///   // secret microtypes have extra restrictions to prevent accidental misuse of sensitive data
///   secret String {
///     Password
///   }
///
///   // "out secret" microtypes have the same restrictions, except that they implement
///   // serde::Serialize
///   out secret String {
///     SessionToken
///   }
/// }
/// ```
#[proc_macro]
pub fn microtype(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let microtype = parse_macro_input!(tokens as MicrotypeMacro);
    let microtypes = flatten(microtype);
    codegen(microtypes).into()
}

