struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("some@test.com"),
        username: String::from("swanny"),
        active: true,
        sign_in_count: 1,
    };
    debug_print(&user1);
    user1.email = String::from("another@example.com");
    debug_print(&user1);

}

fn debug_print(user1: &User) {
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    if user1.active {
        println!("User Status: active");
    } else {
        println!("User Status: inactive");
    }
    println!("Sign-in Count: {}", user1.sign_in_count);

}