#![deny(missing_docs)]

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
//! microtype!(Stirng => UserId);  
//! microtype!(Stirng => OrderId);
//!
//! fn handle_order(user_id: UserId, order_id: OrderId) {
//!    // ...
//! }
//!
//! fn main() {
//!   let user_id: OrderId = retrieve_user_id();
//!   let order_id: UserId = retrieve_order_id();
//!
//!   handle_order(order_id, user_id);  // Error: incompatible types
//! }
//! ```
//!
//! To create a microtype, use the `microtype` macro. Microtypes also implement the `Microtype`
//! trait which defines some common behaviours:
//! ```
//! # #[macro_use]
//! # extern crate microtype;
//! # use microtype::Microtype;
//! microtype!(String => UserId);
//! microtype!(String => Username);
//!
//! fn main() {
//!   let user_id = UserId::new("id".into());  // create new UserId
//!   let string = user_id.into_inner();       // consume UserId, return inner String
//!   let username = Username::new(string);    // create new Username
//!
//!   // sometimes you need to explicitly change the type of a value:
//!   let user_id: UserId = username.transmute();
//! }
//!
//! ```
//! By default, `Debug`, `Clone`, `Eq` and `PartialEq` are derived for a microtype. However, this
//! can be customised by providing a list of traits to derive:
//! ```compile_fail
//! # fn main() {}
//! microtype!(f64 => Coord);  // Error: f64 doesn't implement Eq
//! ```
//! Instead:
//! ```
//! # #[macro_use]
//! # extern crate microtype;
//! # use microtype::Microtype;
//! # fn main() {}
//! microtype!(f64 => Coord, Clone, Debug, PartialEq);  // works
//! ```
//! If you just want to add `Copy` to the derived traits, you can use the `copy_microtype` macro
//! instead:
//! ```
//! # #[macro_use]
//! # extern crate microtype;
//! # use microtype::Microtype;
//! # fn main() {}
//! microtype!(i32 => Foo, Clone, Debug, Eq, PartialEq, Copy);
//! // behaves the same as:
//! copy_microtype!(i32 => Bar);
//! ```
//! Microtypes are also `repr(transparent)`.
//!
//!

mod microtype_macro;
mod secret_microtype;

pub use secret_microtype::SecretMicrotype;
pub use microtype_macro::Microtype;
pub use secrecy;
pub use serde;
pub use paste;
