#![warn(clippy::all)]
#![warn(rustdoc::all)]
#![no_std]

//! A library to generate "microtypes" (A.K.A. "newtypes"). Opinionated in favour of ergonomics
//! over maximum flexibility.
//!
//! A microtype is a thin wrapper around an underlying type, that helps disambiguate similar uses
//! of the same type
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
//! becomes to detect these issues. This becomes especially troublesome if you want to refactor. For
//! example, if you wanted to swap the order of the arguments, you'd have to make sure you visited
//! all the calls to this function and swapped the arguments manually. Luckily, we can get the
//! compiler to help with this.
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
//!     // use the `#[secret]` attribute to mark a type as "secret"
//!     #[secret]
//!     String {
//!         Password
//!     }
//!
//!     // use `#[secret(serialize)]` to make a secret type implement serde::Serialize
//!     String {
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
//!     let user_id: UserId = username.convert();
//!
//!     // microtypes also optionally implement Deref
//!     let length = user_id.len();
//!     assert_eq!(length, 2);
//! }
//! ```
//!
//!
//! ## Secrets
//!
//! Some types may be considered "sensitive" (for example: passwords, session tokens, etc).
//! For this purpose, microtypes can be marked as `#[secret]`:
//! ```
//! # use microtype::microtype;
//! # use microtype::SecretMicrotype;
//! # use microtype::secrecy::ExposeSecret;
//! microtype! {
//!   #[secret]
//!   String {
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
//!  shared reference to the inner data via [`secrecy::ExposeSecret::expose_secret`], which makes
//!  accesses easier to audit.
//!  - They `#[derive(Debug, Clone)]` (and optionally `Serialize` and `Deserialize`) but do not support adding extra derive macros.
//!
//! Internally, they wrap the contained data in [`secrecy::Secret`], which provides some nice
//! safety features. In particular:
//!  - The debug representation is redacted. This is can prevent against accidentally leaking
//!  data to logs, but it still *has* a `Debug` implementation (so you can still
//!  `#[derive(Debug)]` on structs which contain secret data)
//!  - Data is zeroized after use, meaning the underlying data is overwritten with 0s, which
//!  ensures sensitive data exists in memory only for as long as is needed. (Caveat: not all types
//!  have perfect zeroize implementations. Notably `Vec` (and `String`) will not be able to zeroize
//!  previous allocations)
//!  - when using `serde`, secret microtypes do not implement `Serialize`, to avoid accidentally
//!  leaking secret data
//!
//! ## Serializable Secrets
//!
//! The fact that secret microtypes do not implement `Serialize` can be overly restrictive
//! sometimes. There are many types (e.g. session tokens) which are sensitive enough to warrant
//! redacting their debug implementation, but also need to be serialized. For types like this, you
//! can use `#[secret(serialize)]` to make the type implement `Serialize`.
//!
//! ```ignore
//! # use serde::Serialize;
//! # use microtype::microtype;
//! microtype! {
//!     #[secret(serialize)]
//!     String {
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
//! ## Common cases
//!
//! Since wrapping a string is so common, the `#[string]` attribute can be added to a type to
//! provide a few extra implementations (e.g. `FromStr`, `From<str>`, `Display`)
//!
//! ## Feature flags
//!
//! The following feature flags are provided, to help customize the behaviour of the types creates:
//!  - `serde` - when enabled, any type created will derive `Serialize` and `Deserialize`, and will
//!  be `#[serde(transparent)]`
//!  - `deref_impls` - some people argue that implementing `Deref` and `DerefMut` on a non-pointer container is
//!  unidiomatic. Others prefer the ergonomics of being able to call associated functions more
//!  easily. If `deref_impls` is enabled, microtypes will deref to their inner types
//!  - `test_impls` - makes secret microtypes easier to work with in test environments by:
//!    - making their `Debug` implmentation print their actual value instead of `"REDACTED"`
//!    - making them derive `PartialEq`
//!  - `secret` - enables secret microtypes, discussed below:

/* TRAIT DEFS */

/// A trait implemented by microtypes
///
/// Provides some useful common functions for working with microtypes
pub trait Microtype {
    /// The type of the wrapped value
    ///
    /// For example, the inner type of an `EmailAddress` could be a `String`
    type Inner;

    /// Create a microtype from the inner value
    fn new(inner: Self::Inner) -> Self;

    /// Consume this microtype and return the value it contains
    fn into_inner(self) -> Self::Inner;

    /// Get a shared reference to the inner value
    fn inner(&self) -> &Self::Inner;

    /// Get a mutable reference to the inner value
    fn inner_mut(&mut self) -> &mut Self::Inner;

    /// Explicitly convert from one microtype to another.
    ///
    /// This exists as an alternative to `From`/`Into` implementations between different
    /// microtypes to make conversions explicit
    fn convert<T: Microtype<Inner = Self::Inner>>(self) -> T;
}

/// A trait implemented by secret microtypes
///
/// Due to their nature, secret microtypes are more restrictive than regular microtypes:
///  - `inner`, `inner_mut` and `into_inner` are removed, since they can allow accidental use of
///  the contained secret.
///  - `SecretMicrotype` requires `ExposeSecret<Self::Inner>`; to use the contained data, use
///  `.expose_secret()`
///
///  The wrapped type must also implement [`secrecy::Zeroize`]
#[cfg(feature = "secret")]
pub trait SecretMicrotype: secrecy::ExposeSecret<Self::Inner> {
    /// The type of the wrapped value
    /// For example, the inner type of a `Password` could be a `String`
    type Inner: secrecy::Zeroize;

    /// Create a microtype from the inner value
    ///
    /// Note that it is not possible to retrieve the owned value, it can only be read via shared
    /// reference obtained via `expose_secret()`
    fn new(inner: Self::Inner) -> Self;
}

pub use microtype_macro::microtype;
#[cfg(feature = "secret")]
pub use secrecy;

#[cfg(test)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");

    #[cfg(feature = "serde")]
    t.pass("tests/ui/pass/serde/*.rs");
    #[cfg(feature = "serde")]
    t.compile_fail("tests/ui/fail/serde/*.rs");
}
