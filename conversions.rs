#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item}
    }
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    let num = Number::from(30);
    println!("My number is {:?}", num);
    
    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x * x_squared;

        x_cube+x_squared+x 
    };

    let z = {
        2 * x * y
    };

    println!("x = {:?}", x);
    println!("y = {:?}", y);
    println!("z = {:?}", z);
    

}
