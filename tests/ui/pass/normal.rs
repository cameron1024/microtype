use microtype_core::Microtype;

microtype_macro::microtype! {
    #[derive(Clone)]
    String {
        Email,
        Username,
    }
}


fn main() {
    let mut email = Email::new("hello".into());
    assert_eq!(email.inner(), "hello");
    assert_eq!(email.inner_mut(), "hello");
    assert_eq!(email.clone().into_inner(), "hello");

    let username: Username = email.transmute();
    assert_eq!(username.into_inner(), "hello");
}
