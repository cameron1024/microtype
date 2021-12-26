microtype_macro::microtype! {
    out secret String {
        Token
    }
    secret String {
        Password
    }
}


fn main() {
    use microtype_core::SecretMicrotype;

    let token = Token::new("asdf".into());
    assert_serialize(token.clone());
    assert_deserialize(token);

    let password = Password::new("asdf".into());
    assert_deserialize(password);
}

fn assert_serialize<T: serde::Serialize>(_t: T) {}
fn assert_deserialize<'a, T: serde::Deserialize<'a>>(_t: T) {}
