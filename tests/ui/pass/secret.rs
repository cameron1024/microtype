microtype_macro::microtype! {
    secret String {
        Password
    }
}

fn main() {
    use microtype_core::SecretMicrotype;
    use microtype_core::secrecy::ExposeSecret;
    let password = Password::new("string".into());
    assert_eq!(password.expose_secret(), "string"); 

    // secret microtype is clone and debug by default
    let _ = password.clone();
    let password = format!("{:?}", password);
    assert!(password.contains("REDACTED"))
}
