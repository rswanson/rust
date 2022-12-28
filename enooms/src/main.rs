
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Write(s) => {
                println!("String to write {:?}", s);
            }
            Message::Quit => {}
            Message::ChangeColor(r, g, b) => {
                println!("Red: {}, Green: {}, Blue: {}", r, g, b);
            }
            Message::Move{x,y} => {
                println!("X Coordinate: {}, Y Coordinate: {}", x, y);
            }
        }
    }
}
fn main() {
    let ip_v4 = IpAddrKind::V4(10, 0, 0, 1);
    let ip_v6 = IpAddrKind::V6(String::from("::1"));

    print_ip(ip_v4);
    print_ip(ip_v6);

    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::ChangeColor(0, 0, 0);
    m.call();
    let m = Message::Quit;
    m.call();
    let m = Message::Move{x: 0, y: 0};
    m.call();
}

fn print_ip(ip: IpAddrKind) {
    match ip {
        IpAddrKind::V4(n1, n2, n3, n4) => {
            println!("{}.{}.{}.{}",n1,n2,n3,n4);
        }
        IpAddrKind::V6(s) => {
            println!("{}",s);
        }
    }
}
