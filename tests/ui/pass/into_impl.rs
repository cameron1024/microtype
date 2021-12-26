microtype_macro::microtype! {
    String {
        Email
    }
}

fn main() {
    let _: Email = "hello".to_string().into();
}
