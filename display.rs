use std::fmt;

#[derive(Debug)]
struct MinMax(i64,i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"({} {})", self.0, self.1)
    }
} 
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x:{}, y:{}",self.x,self.y)

    }
}

#[derive(Debug)]
struct ComplexNumber {
    real: f64,
    imaginary: f64,
}

impl fmt::Display for ComplexNumber {
    fn fmt(&self, f:&mut fmt::Formatter)-> fmt::Result {
        write!(f, "{} + {}i",self.real,self.imaginary)
    }
}

fn main() {
    let minmax =MinMax(0,14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("debug: {:?}", minmax);

    let big_range = MinMax(-300,300);
    let small_range = MinMax(-3,3);

    println!("The big range is {big} and small range is {small}", big=big_range, small=small_range);

    let point = Point2D {x: 3.3, y: 4.7};
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    
    let complexnumber = ComplexNumber {real:3.3, imaginary:7.2};
    println!("Print the complex number");
    println!("Display: {}",complexnumber);
    println!("Debug: {:?}",complexnumber);   
}