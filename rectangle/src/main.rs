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
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 20,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("The rect struct info: {:#?}", rect1);
    dbg!(&rect2);
    println!("This one uses an implementation: {}", rect3.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
