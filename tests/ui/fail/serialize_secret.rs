microtype_macro::microtype! {
    secret String {
        Password
    }
}


fn main() {
    use microtype::SecretMicrotype;
    let password = Password::new("asdf".into());
    assert_serialize(password);
}

fn assert_serialize<T: serde::Serialize>(_t: T) {}
