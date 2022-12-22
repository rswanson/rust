#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle::new(20,40);
    println!(
        "The area of the rectangle is {} square pixels", 
        rect1.area()
    );
    dbg!(&rect1);
    dbg!(&rect1.can_hold(&rect2));
    // debug_print(&rect1);
}

fn debug_print(rectangle: &Rectangle) {
    println!("{:#?}", rectangle);

}
