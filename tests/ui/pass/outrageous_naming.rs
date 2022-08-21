// for people who are really trying to screw with macro authors

#[allow(non_camel_case_types, dead_code)]
struct str;

#[allow(non_camel_case_types, dead_code)]
struct i32 {}

mod core {}

mod std {}



microtype::microtype! {
    #[string]
    String {
        Email
    }

    #[int]
    ::core::primitive::i32 {
        Num
    }
}

use microtype::Microtype;

fn main() {
    let email = Email::from("hello"); // relies on `From<&str>` from `#[string]`
    let _ = format!("{email}");
    is_as_ref_str(email);

    let num = Num::from(123);
    let _ = format!("{num}");
    let mut num = num + Num::from(123);
    num *= Num::from(2);

    assert_eq!(num.into_inner(), 123 * 4);
}

fn is_as_ref_str(_s: impl AsRef<::core::primitive::str>) {}
