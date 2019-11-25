#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }

    fn square(size: u64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("{:#?}", rect1);
    println!("The area of rectangle is {} square pixels", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "Can rect1 hold square? {}",
        rect1.can_hold(&Rectangle::square(3))
    );
}

fn main() {
    run();
}
