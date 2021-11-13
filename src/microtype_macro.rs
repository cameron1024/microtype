/// A trait implemented by microtypes
///
/// Provides some useful common functions for working with microtypes
pub trait Microtype {
    /// The type of the wrapped value
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
    /// This exists as an alternative to `From`/`Into` implementations to make conversions explicit
    fn transmute<T: Microtype<Inner = Self::Inner>>(self) -> T;
}

/// Create a new microtype
///
/// For example, to create a microtype called `EmailAddress` that wraps a `String`, write:
/// ```
/// # #[macro_use]
/// # extern crate microtype;
/// # fn main() {}
/// # use microtype::Microtype;
/// microtype!(String => EmailAddress);
/// ```
/// To declare multiple microtypes at once:
/// ```
/// # #[macro_use]
/// # extern crate microtype;
/// # fn main() {}
/// # use microtype::Microtype;
/// microtype!(String => [Foo, Bar, Baz]);
/// ```
///
/// Microtypes by default will have the following:
///  - `repr(transparent)`
///  - `derive(Debug, Clone, Eq, PartialEq)`
///  - `derive(serde::Serialize, serde::Deserialize)` (if the `serde` feature is enabled)
///  - `serde(transparent)` (if the `serde` feature is enabled)
///
///  However, if you wish to customise the derived traits, put them in a comma separated list after
///  the name of the type:
/// ```
/// # #[macro_use]
/// # extern crate microtype;
/// # fn main() {}
/// # use microtype::Microtype;
/// microtype!(String => EmailAddress, Clone, Debug);  // doesn't impl PartialEq or Eq
/// ```
/// A trailing comma can be used to not derive *any* traits:
/// ```compile_fail
/// # #[macro_use]
/// # extern crate microtype;
/// # use microtype::Microtype;
/// microtype!(String => EmailAddress,);  // note the trailing comma
///
/// fn main() {
///   let email = EmailAddress::new("example@example.com".into());
///   let cloned = email.clone();  // error: EmailAddress is not Clone
/// }
/// ```
#[macro_export]
macro_rules! microtype {
    ($inner:ty => [$name:ident]) => {microtype!($inner => $name);};
    ($inner:ty => [$name:ident], $($traits:tt)*) => {microtype!($inner => $name, $($traits)*);};
    ($inner:ty => [$name:ident, $($names:ident),*]) => {
        microtype!($inner => $name);
        microtype!($inner => [$($names),*]);
    };
    ($inner:ty => [$name:ident, $($names:ident),*], $($traits:tt)*) => {
        microtype!($inner => $name, $($traits)*);
        microtype!($inner => [$($names),*], $($traits)*);
    };
    ($inner:ty => $name:ident) => {
        microtype!($inner => $name, Debug, Clone, Eq, PartialEq);
    };
    ($inner:ty => $name:ident, $($traits:tt),*) => {

        #[repr(transparent)]
        #[derive(serde::Deserialize, serde::Serialize)]
        #[serde(transparent)]
        #[derive($($traits),*)]
        pub struct $name {
            inner: $inner,
        }

        impl $crate::Microtype for $name {
            type Inner = $inner;

            fn new(inner: Self::Inner) -> Self {
                Self { inner }
            }

            fn into_inner(self) -> Self::Inner {
                self.inner
            }

            fn inner(&self) -> &Self::Inner {
                &self.inner
            }

            fn inner_mut(&mut self) -> &mut Self::Inner {
                &mut self.inner
            }

            fn transmute<T: Microtype<Inner = Self::Inner>>(self) -> T {
                T::new(self.into_inner())
            }
        }
    };
}

/// A utility macro to define microtypes that implement `Copy`
///
/// By default, the `microtype` macro derives `Debug`, `PartialEq`, `Eq` and `Clone`. This can be
/// overriden, but if you just want to *add* a `Copy` implementation, this macro can be used as a
/// shorthand
/// ```
/// # #[macro_use]
/// # extern crate microtype;
/// # use microtype::Microtype;
/// copy_microtype!(i32 => Age);
///
/// fn main() {
///   let age = Age::new(123);
///   drop(age);
///   drop(age);  // Age: Copy
/// }
/// ```
#[macro_export]
macro_rules! copy_microtype {
    ($inner:ty => $name:ident) => {
        microtype!($inner => $name, Debug, Clone, Eq, PartialEq, Copy);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // non Eq types can be created by specifying an empty list of traits
    microtype!(f64 => Coord, );

    microtype!(String => Email);
    microtype!(String => Username);

    #[test]
    fn email_example() {
        let email = "email".to_string();
        let mut email = Email::new(email);
        assert_eq!(email.inner(), "email");
        assert_eq!(email.inner_mut(), "email");
        assert_eq!(email.into_inner(), "email");
    }

    #[test]
    fn email_clone() {
        let email = Email::new("email".into());
        let cloned = email.clone();
        assert_eq!(email, cloned);
    }

    #[test]
    fn can_transmute() {
        let email = Email::new("user@example.com".to_string());
        let cloned = email.clone();
        let username = email.transmute::<Username>();
        assert_eq!(cloned.into_inner(), username.into_inner());
    }

    microtype!(String => [Name, Address]);

    #[test]
    fn multiple_declarations() {
        let name = Name::new("name".into());
        let address = Address::new("example road".into());
        assert_eq!(name.into_inner(), "name");
        assert_eq!(address.into_inner(), "example road");
    }

    microtype!(f64 => [X, Y, Z], Clone, Copy, PartialEq, Debug);

    #[test]
    fn multiple_declarations_with_traits() {
        let x = X::new(1.0);
        let y = Y::new(2.0);
        let z = Z::new(3.0);

        assert_eq!(x.into_inner(), 1.0);
        assert_eq!(y.into_inner(), 2.0);
        assert_eq!(z.into_inner(), 3.0);
    }

    #[derive(serde::Deserialize)]
    struct Example {
        email: Email,
        username: Username,
    }

    #[test]
    fn serde_transparent() {
        let json = r#"{"email": "1234", "username": "2345"}"#;
        let example: Example = serde_json::from_str(json).unwrap();
        assert_eq!(example.email.inner(), "1234");
        assert_eq!(example.username.inner(), "2345");
    }
}
