struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = build_user(String::from("some@test.com"), String::from("swanny"));
    debug_print(&user1);
    user1.email = String::from("another@example.com");
    debug_print(&user1);
    let user2 = User {
        email: String::from("third@example.com"),
        ..user1
    };
    debug_print(&user2);

    let black = Color(0, 0, 0);
    let origin = Point(0,0,0);
    println!("Color: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);
    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
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