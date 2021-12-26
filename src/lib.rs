//! A library to ease creation of so-called "microtypes".
//!
//! A microtype is a thin wrapper around a more primitive type (e.g. `String`), that can help to provide
//! safety against logic bugs at compile-time.
//!
//! For example, consider the following code from an imaginary e-commerce app:
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
//! becomes to spot these issues. Not only that, when refactoring, wouldn't it be nice if the
//! compiler could highlight all the places you needed to change for you?
//!
//! For this, we use microtypes. Microtypes are thin wrappers around other types, and tell the
//! compiler about different uses for the same underlying data. For example, if we rewrite our
//! earlier code to use microtypes, it might look like this:
//!
//! ```compile_fail
//! # #[macro_use]
//! # extern crate microtype;
//! # fn retrieve_user_id() -> String { "".into() }
//! # fn retrieve_order_id() -> String { "".into() }
//! // microtype wrappers around String
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
//! To create a microtype, use the `microtype` macro. Microtypes also implement the `Microtype`
//! trait which defines some common behaviours:
//! ```
//! # #[macro_use]
//! # extern crate microtype;
//! # use microtype::Microtype;
//! microtype! {
//!     #[derive(Debug, Clone)]
//!     String {
//!         #[derive(PartialEq)]
//!         UserId,    // implements Debug, Clone and PartialEq
//!         Username,  // only implements Debug and Clone
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
//! Secret microtypes don't implement [`microtype::Microtype`], instead they implement
//! [`microtype::SecretMicrotype`], which has a much more restrictive API:
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
//!  - when using `serde`, secret microtypes do not implement [`serde::Serialize`]. For example, in
//!  a web server, using a secret microtype for `Password`s would prevent a password from being
//!  sent in a web response, at compile time.
//!
//! ## Serializable Secrets
//!
//! That final point can be overly restrictive at times. For example, session tokens might
//! reasonably be considered "sensitive", but you are likely going to want to serialize them at
//! some point.
//!
//! For this purpose, there is an escape hatch. By declaring an `out secret`, a [`serde::Serialize`] 
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



pub use microtype_macro::microtype;
pub use microtype_core::*;
pub use secrecy;

#[cfg(test)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
