#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    
    // These are methods because they use self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This is also a method
    fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //This is an associated function because it doesn't use self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 60,
    };

    let rect4 = Rectangle {
        width: 70,
        height: 50,
    };

    //Associated function is called using `::`
    println!("The square is\n{:?}",Rectangle::square(20));

    println!("The area of rectangle is {}", rect1.area());
    println!("Rectangle:\n{:?}",rect1);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}