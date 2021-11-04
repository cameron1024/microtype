
/// A trait implemented by microtypes
pub trait Microtype {
    /// The type of the value wrapped by this microtype
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
#[macro_export]
macro_rules! microtype {
    ($inner:ty => $name:ident) => {
        microtype!($inner => $name, Debug, Clone, Eq, PartialEq);
    };
    ($inner:ty => $name:ident, $($traits:tt),*) => {
        #[repr(transparent)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        #[cfg_attr(feature = "serde", serde(transparent))]
        #[derive($($traits),*)]
        
        struct $name {
            inner: $inner,
        }

        impl Microtype for $name {
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

    #[cfg(serde)]
    #[derive(serde::Deserialize)]
    struct Example {
        email: Email,
        username: Username,
    }

    #[test]
    #[cfg(serde)]
    fn serde_transparent() {
        let json = r#"{"email": "1234", "username": "2345"}"#;
        let example: Example = serde_json::from_str(json).unwrap();
        assert_eq!(example.email, "1234");
        assert_eq!(example.username, "2345");

    }
}
