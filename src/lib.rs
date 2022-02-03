#![warn(clippy::all)]
#![warn(rustdoc::all)]

//! A library to generate "microtypes" (A.K.A. "newtypes").
//!
//! A microtype is a thin wrapper around a more primitive type (e.g. `String`), that helps prevent 
//! logic and security bugs at compile time.
//!
//! For example, consider the following code from an imaginary e-commerce web backend:
//! ```
//! # #[macro_use]
//! # extern crate microtype;
//! # fn retrieve_user_id() -> String { "".into() }
//! # fn retrieve_order_id() -> String { "".into() }
//! fn handle_order(user_id: String, order_id: String) {
//!    // ...
//! }
//!
//! fn main() {
//!   let user_id = retrieve_user_id();
//!   let order_id = retrieve_order_id();
//!
//!   handle_order(order_id, user_id);
//! }
//! ```
//!
//! This code compiles, but has a bug: the `order_id` and `user_id` are used in the wrong order.
//! This example is fairly trivial and easy to spot, but the larger a project gets, the harder it
//! becomes to spot these issues. This becomes especially troublesome if you want to refactor. For
//! example, if you wanted to swap the order of the arguments, you'd have to make sure you visited
//! all the calls to this function and swapped the arguments manually. What if we could make the
//! compiler do this?
//!
//! Microtypes solve this problem. They wrap some inner type, and allow the compiler to distinguish
//! between different uses of the same underlying type. For example, we could rewrite the earlier
//! example as:
//!
//! ```compile_fail
//! # #[macro_use]
//! # extern crate microtype;
//! # fn retrieve_user_id() -> String { "".into() }
//! # fn retrieve_order_id() -> String { "".into() }
//! // Generate wrappers around String called UserId and OrderId
//! microtype! {
//!     String {
//!         UserId,
//!         OrderId,
//!     }
//! }
//!
//! fn handle_order(user_id: UserId, order_id: OrderId) {
//!     // ...
//! }
//!
//! fn main() {
//!     let user_id: OrderId = retrieve_user_id();
//!     let order_id: UserId = retrieve_order_id();
//!
//!     handle_order(order_id, user_id);  // Error: incompatible types
//! }
//! ```
//! Excellent, a run-time error has been turned into a compile time error.
//!
//! ## Basic usage
//!
//! To declare microtypes, use the ['microtype::microtype'] macro:
//! ```
//! # #[macro_use]
//! # extern crate microtype;
//! # use microtype::Microtype;
//! microtype! {
//!     // these attributes apply to all microtypes defined in this block
//!     #[derive(Debug, Clone)]
//!     String {
//!         #[derive(PartialEq)]
//!         UserId,    // implements Debug, Clone and PartialEq
//!
//!         Username,  // only implements Debug and Clone
//!     }
//!
//!     // multiple inner types can be used in a single macro
//!     i64 {
//!         Timestamp
//!     }
//!
//!     // use `secret` to mark a type as secret
//!     secret String {
//!         Password
//!     }
//!
//!     // use `out secret` to make a secret type implement serde::Serialize
//!     out secret String {
//!         SessionToken
//!     }
//! }
//!
//! fn main() {
//!     let user_id = UserId::new("id".into());  // create new UserId
//!     let string = user_id.into_inner();       // consume UserId, return inner String
//!     let username = Username::new(string);    // create new Username
//!
//!     // sometimes you need to explicitly change the type of a value:
//!     let user_id: UserId = username.transmute();
//!
//!     // microtypes also implement Deref
//!     let length = user_id.len();
//!     assert_eq!(length, 2);
//! }
//! ```
//!
//! If built with the `serde_impls` feature, microtypes will be "transparent" when interacting with
//! `serde`. For example:
//! ```
//! # use microtype::microtype;
//! # use serde::{Serialize, };
//! ```
//!
//! ## Secrets
//!
//! Some types may be considered "sensitive" (for example: passwords, session tokens, maybe even
//! email addresses depending on your definition). For this purpose, microtypes can be marked as
//! `secret`:
//! ```
//! # use microtype::microtype;
//! # use microtype::SecretMicrotype;
//! # use microtype::secrecy::ExposeSecret;
//! microtype! {
//!   secret String {
//!     Password
//!   }
//! }
//!
//! fn main() {
//!     let password = Password::new("secret password".to_string());
//!     assert_eq!(password.expose_secret(), "secret password");
//! }
//! ```
//! Secret microtypes don't implement [`Microtype`], instead they implement
//! [`SecretMicrotype`], which has a much more restrictive API:
//!  - Mutable and owned access to the inner data is not possible, it is only possible to get a
//!  shared reference to the inner data via [`secrecy::ExposeSecret::expose_secret`].
//!  - They `#[derive(Debug, Clone)]` (and optionally `Serialize` and `Deserialize`) but do not support adding extra derive macros.
//!
//! Internally, they wrap the contained data in [`secrecy::Secret`], which provides some nice
//! safety features. In particular:
//!  - The debug representation is redacted. This is can prevent against accidentally leaking
//!  data to logs, without forgoing a `Debug` implementation (this means you can still
//!  `#[derive(Debug)]` on structs which contain secret data)
//!  - Data is zeroized after use, meaning the underlying data is overwritten with 0s, which
//!  ensures sensitive data exists in memory only for as long as is needed. (Caveat: not all types
//!  have perfect zeroization implementations, notably `Vec`, etc. will not be able to zeroize
//!  previous allocations)
//!  - when using `serde`, secret microtypes do not implement `Serialize`. For example, in
//!  a web server, using a secret microtype for `Password`s would prevent a password from being
//!  sent in a web response, checked at compile time.
//!
//! ## Serializable Secrets
//!
//! That final point can be overly restrictive at times. For example, session tokens might
//! reasonably be considered "sensitive", but you are likely going to want to serialize them at
//! some point.
//!
//! For this purpose, there is an escape hatch. By declaring a microtype as an `out secret`, a `Serialize`
//! implementation will be generated for your microtype:
//!
//! ```
//! # use serde::Serialize;
//! # use microtype::microtype;
//! microtype! {
//!     out secret String {
//!         SessionToken
//!     }
//! }
//!
//! #[derive(Serialize)]
//! struct LoginResponse {
//!     token: SessionToken
//! }
//! ```
//!
//! ## Feature Flags
//! - `serde_impls` - automatically generate `serde` implementations for secret microtypes
//! - `test_impls` - generate `PartialEq` implementations in tests

pub use microtype_core::secrecy;
pub use microtype_core::{Microtype, SecretMicrotype};
pub use microtype_macro::microtype;

#[cfg(test)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
