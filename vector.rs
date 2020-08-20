use std::convert::From;

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
    let element = 5u8;
    let mut vec = Vec::new();
    let num = Number::from(30);
    println!("the number is {:?}",num);
    vec.push(element);
    println!("{:?}", vec);
    type NanoSeconds = u64;
    type Inch = u64;

    #[allow(non_camel_case_types)]
    type u64_t = u64;
    let nanoseconds: NanoSeconds = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    println!("{} Nanoseconds + {} Inches = {} unit?", nanoseconds,inches,nanoseconds+inches);

}