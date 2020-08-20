static language: &str = "Rust";
const threshold: i32 = 10;

fn is_big(n: i32)-> bool {
    n>threshold
}

fn main() {

    let body = reqwest::get("https://www.rust-lang.org")?
    .text()?;

    println!("body = {:?}", body);

    let n = 16;
    println!("This is {}",language);
    println!("The threshold is {}",threshold);
    println!("{} is {}", n,if is_big(n) {"big"} else {"small"});
    let a_binding;
    {
        let x=2;
        a_binding= x*x;
    }
    println!("a_binding: {}", a_binding);
    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
}