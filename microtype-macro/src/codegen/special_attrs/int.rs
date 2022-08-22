use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_str, Ident, Type};

use super::helpers::fmt_impl;

pub fn generate_int_impls(name: &Ident, inner: &Type) -> TokenStream {
    let display = fmt_impl(name, inner, &parse_str("::core::fmt::Display").unwrap());
    let octal = fmt_impl(name, inner, &parse_str("::core::fmt::Octal").unwrap());
    let lower_hex = fmt_impl(name, inner, &parse_str("::core::fmt::LowerHex").unwrap());
    let upper_hex = fmt_impl(name, inner, &parse_str("::core::fmt::UpperHex").unwrap());
    let binary = fmt_impl(name, inner, &parse_str("::core::fmt::Binary").unwrap());
    let lower_exp = fmt_impl(name, inner, &parse_str("::core::fmt::LowerExp").unwrap());
    let upper_exp = fmt_impl(name, inner, &parse_str("::core::fmt::UpperExp").unwrap());

    quote! {
        #display
        #octal
        #lower_hex
        #upper_hex
        #binary
        #lower_exp
        #upper_exp

        impl ::core::str::FromStr for #name {
            type Err = ::core::num::ParseIntError;

            fn from_str(s: &::core::primitive::str) -> Result<Self, Self::Err> {
                <#inner as ::core::str::FromStr>::from_str(s).map(Self)
            }
        }

        impl ::core::ops::Add for #name {
            type Output = #name;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl ::core::ops::Sub for #name {
            type Output = #name;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl ::core::ops::Mul for #name {
            type Output = #name;
            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }

        impl ::core::ops::Div for #name {
            type Output = #name;
            fn div(self, rhs: Self) -> Self::Output {
                Self(self.0 / rhs.0)
            }
        }

        impl ::core::ops::Rem for #name {
            type Output = #name;
            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }

        impl ::core::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl ::core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl ::core::ops::MulAssign for #name {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0
            }
        }

        impl ::core::ops::DivAssign for #name {
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0
            }
        }

        impl ::core::ops::RemAssign for #name {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0
            }
        }

    }
}
