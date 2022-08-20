use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate_int_impls(name: &Ident) -> TokenStream {
    quote! {

        impl ::std::ops::Add for #name {
            type Output = Num;
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl ::std::ops::Sub for #name {
            type Output = Num;
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl ::std::ops::Mul for #name {
            type Output = Num;
            fn mul(self, rhs: Self) -> Self::Output {
                Self(self.0 * rhs.0)
            }
        }

        impl ::std::ops::Div for #name {
            type Output = Num;
            fn div(self, rhs: Self) -> Self::Output {
                Self(self.0 / rhs.0)
            }
        }

        impl ::std::ops::Rem for #name {
            type Output = Num;
            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }

        impl ::std::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }

        impl ::std::ops::SubAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }

        impl ::std::ops::MulAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0
            }
        }

        impl ::std::ops::DivAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0
            }
        }

        impl ::std::ops::RemAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0
            }
        }

    }
}
