use secrecy::{ExposeSecret, Zeroize};

/// A trait implemented by secret microtypes
///
/// Due to their nature, secret microtypes are more restrictive than regular microtypes:
///  - `inner`, `inner_mut` and `into_inner` are removed, since they can allow accidental use of
///  the contained secret.
///  - `SecretMicrotype` requires `ExposeSecret<Self::Inner>`; to use the contained data, use
///  `.expose_secret()`
pub trait SecretMicrotype: ExposeSecret<Self::Inner> {
    /// The type of the wrapped value
    /// For example, the inner type of a `Passweord` could be a `String`
    type Inner: Zeroize;

    /// Create a microtype from the inner value
    /// 
    /// Note that it is not possible to retrieve the owned value, it can only be read via shared
    /// reference obtained via `expose_secret()`
    fn new(inner: Self::Inner) -> Self;
}

/// Create a secret microtypes
///
/// A secret microtype is intended for sensitive data (e.g. passwords, session tokens, API keys,
/// etc). It wraps type in `Secret` from the crate `secrecy`. This provides the following benefits:
///  - when dropped, the memory is first "zeroed" to avoid leaving secrets in the heap (this is not
///  a 100% guarantee, since some types may leave data on the heap when reallocated, for example
///  `Vec`)
///  - the `Debug` implementation is redacted, making it harder to accidentally log/print sensitive
///  data
///  - unless explicitly enabled, `serde::Serialize` is not implemented, preventing accidentally
///  writing to disk or sending over the network
///
///  For example:
///  ```
///  # #[macro_use]
///  # extern crate microtype;
///  # use microtype::*;
///  
///  secret_microtype!(String => [Password]);
///
///  fn main() {
///     let password = Password::new("asdf".into());
///     println!("{:?}", password);  // prints "[REDACTED]" with some type info
///  }
///  ```
///  If you want to serialize a microtype, use the following:
///  ```
///  # #[macro_use]
///  # extern crate microtype;
///  # use microtype::*;
///  # use serde::Serialize;
///  # fn main() {}
///  # struct Credentials;
///  secret_microtype!(ser String => [Jwt]);
///  
///  fn login(credentials: Credentials) -> impl Serialize {
///     // ...
///     Jwt::new("example".into())
///  }
///  ```
#[macro_export]
macro_rules! secret_microtype {
    (ser $inner:ty => [$name:ident, $($names:ident),*]) => {
        secret_microtype!(ser $inner => [$name, $($names),*], );
    };
    (ser $inner:ty => [$name:ident]) => {
        secret_microtype!(ser $inner => [$name],);
    };
    ($inner:ty => [$name:ident, $($names:ident),*]) => {
        secret_microtype!($inner => [$name, $($names),*],);
    };
    ($inner:ty => [$name:ident]) => {
        secret_microtype!($inner => [$name],);
    };





    (ser $inner:ty => [$name:ident, $($names:ident),*], $($derives:tt),*) => {
        secret_microtype!(ser $inner => [$name], $($derives),*);
        secret_microtype!(ser $inner => [$($names),*], $($derives),*);
    };
    (ser $inner:ty => [$name:ident], $($derives:tt),*) => {
        $crate::paste::paste! {
            #[repr(transparent)]
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
            #[derive($($derives),*)]
            #[serde(transparent)]
            pub struct $name(secrecy::Secret<[<$name Wrapper>]>);
            #[repr(transparent)]
            #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
            #[derive($($derives),*)]
            #[serde(transparent)]
            pub struct [<$name Wrapper>]($inner);

            impl $crate::secrecy::CloneableSecret for [<$name Wrapper>] {}
            impl $crate::secrecy::DebugSecret for [<$name Wrapper>] {}
            impl $crate::secrecy::SerializableSecret for [<$name Wrapper>] {}

            impl $crate::secrecy::Zeroize for [<$name Wrapper>] {
                fn zeroize(&mut self) {
                    self.0.zeroize()
                }
            }

            impl $crate::secrecy::ExposeSecret<$inner> for $name {
                fn expose_secret(&self) -> &String {
                    &self.0.expose_secret().0
                }
            }

            impl $crate::SecretMicrotype for $name {
                type Inner = $inner;

                fn new(inner: Self::Inner) -> Self {
                    Self($crate::secrecy::Secret::new([<$name Wrapper>](inner)))
                }
            }
        }
    };
    ($inner:ty => [$name:ident, $($names:ident),*], $($derives:tt),*) => {
        secret_microtype!($inner => [$name], $($derives),*);
        secret_microtype!($inner => [$($names),*], $($derives),*);
    };
    ($inner:ty => [$name:ident], $($derives:tt),*) => {
        $crate::paste::paste! {
            #[repr(transparent)]
            #[derive($($derives),*)]
            #[derive(serde::Deserialize, Debug, Clone)]
            #[serde(transparent)]
            pub struct $name($crate::secrecy::Secret<[<$name Wrapper>]>);
            #[repr(transparent)]
            #[derive($($derives),*)]
            #[derive($crate::serde::Deserialize, Debug, Clone)]
            #[serde(transparent)]
            pub struct [<$name Wrapper>]($inner);

            impl $crate::secrecy::CloneableSecret for [<$name Wrapper>] {}
            impl $crate::secrecy::DebugSecret for [<$name Wrapper>] {}

            impl $crate::secrecy::Zeroize for [<$name Wrapper>] {
                fn zeroize(&mut self) {
                    self.0.zeroize()
                }
            }

            impl $crate::secrecy::ExposeSecret<$inner> for $name {
                fn expose_secret(&self) -> &String {
                    &self.0.expose_secret().0
                }
            }

            impl SecretMicrotype for $name {
                type Inner = $inner;

                fn new(inner: Self::Inner) -> Self {
                    Self($crate::secrecy::Secret::new([<$name Wrapper>](inner)))
                }
            }
        }
    };
}



#[cfg(test)]
mod tests {
    use super::*;

    secret_microtype!(ser String => [Jwt]);
    secret_microtype!(String => [Password, Other]);

    #[test]
    fn example_non_serializable() {
        let password = Password::new("asdf".into());
        let debug = format!("{:?}", password);
        assert!(!debug.contains("asdf"));
        let _ = password.clone();
    }

    #[test]
    fn example_serializable() {
        let jwt = Jwt::new("jwt".into());
        let parsed: Jwt = serde_json::from_str(r#""jwt""#).unwrap();
        let serialized = serde_json::to_string(&jwt).unwrap();

        assert_eq!(parsed.expose_secret(), jwt.expose_secret());
        assert_eq!(serialized, r#""jwt""#);
    }

    mod can_customise_derives {
        secret_microtype!(ser String => [Asdf],);
    }
}
