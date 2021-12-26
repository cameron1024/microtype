
use microtype_core::Microtype;

microtype_macro::microtype! {
    #[derive(Debug)]
    String {
        #[derive(Clone)]
        Email
    }
}


fn main() {
    let email = Email::new("hello".into());
    assert_eq!(email.inner(), "hello");

    let _ = format!("{:?}", email);
    let _ = email.clone();
}
