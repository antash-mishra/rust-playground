fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut v1 = vec![1,2,3,4,5,8];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}",v);
    println!("{:?}",v1);

    let third: &i32 = &v1[2];
    println!("the third element is {}", third);
    
    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("No third element."),
    }

    let does_not_exist = v.get(2);
    println!("{:?}",does_not_exist);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v1 {
        *i += 50;
    }
    println!("{:?}",v1);
// use of enum
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f32),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(36.2636),
    SpreadsheetCell::Text(String::from("blue")),
];

println!("{:?}",row);

//initialization using new() and changing into string using to_string()
let mut s = String::new();
let data = "initial contents";
let s = data.to_string();
println!("{}",s);

let s = "initial contents".to_string();
println!("{}",s);

let s = String::from("Initial contents");
println!("{}",s);




}