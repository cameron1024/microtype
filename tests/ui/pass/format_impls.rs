microtype::microtype! {
    #[string]
    String {
        Email
    }

    #[int]
    i32 {
        Num
    }
}

fn main() {
    let email = Email::from("email");
    let num = Num::from(123);

    // strings should impl display
    let _ = format!("{email}");

    // numbers should impl more
    let _ = format!("{num:o}, {num:x}, {num:X}, {num:b}, {num:e}, {num:E}");
}
