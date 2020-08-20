use std::iter::Iterator;

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.into_iter();

    let total: i32 = v1_iter.clone().sum();
    
    let maps: Vec<_> = v1_iter.map(|x| x+1).collect();
    //for val in v1_iter.clone() {
    //    println!("Got: {}", &val);
    //}

    let mut counter = Counter::new();

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a*b)
        .filter(|x| x%3 == 0)
        .sum();


    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
        
    }

    println!("{:?}", maps);
    println!("{:?}", &counter.next());
    println!("{}", sum);
    
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (0, y) => println!("x is 0 and y is {:?}", y),
        (x, 0) => println!("x is {:?} and y is 0", x),
        _      => println!("It doesn't matter what they are"),
    }

    let color = Color::RGB(122, 17, 40);
    println!("Which color is it?");

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("THe color is Green!"),
        Color::RGB(r, g, b) => 
            println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) => 
            println!("Hue: {}, saturation: {}, value: {}", h, s, v),
        Color::CMYK(c, m, y, k) => 
            println!("Cyan: {}, magenta: {}, yellow: {}, key(black): {}", c, m, y, k),
    }

}