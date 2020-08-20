use std::string;
use std::collections::HashMap;

fn main() {
    
//creating new string
let mut s = String::new();
let data = "Initial contents";
let s = data.to_string();

//the method works on literal directly
let d = "initial contents".to_string();

//String::from to create string from string literals
let p = String::from("initial contents");

//Appending to a string with `push_str`
let mut x = String::from("foo");
x.push_str("bar");
println!("{}", x);
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s1);

//Appending to string with `push`   
let mut s3 = String::from("lo");
s3.push('l');
println!("{}", s3);

//Appwnding string with `+`
let s4 = String::from("Hello ");
let s5 = String::from("world!");
let s6 = s4 + &s5; //s4 has been moved so it can't be used and s5 has been borrowed
println!("{}", s6);

// for concatenating multiple strings we donot use `+`, it gets unwidely
let s7 = String::from("tic");
let s8 = String::from("tac");
let s9 = String::from("toe");
let s10 = s7 + "-" + &s8 + "-" + &s9;
let s11 =format!("{}-{}-{}",s5,s8,s9);
println!("{}", s11);
//It doesnot take the owernship so we should use `format!` instead of `+`
println!("{}", s10);

//accessing each element of string
for c in "नमस्ते".chars() { //`chars` iterates each element
    println!("{}", c);
}

for b in "नमस्ते".bytes() {
    println!("{}", b); // It returns each raw byte 
//It returns 18 bytes that make up this string
}

//HashMap<K, V>
//creating New Hash Map
//HashMap can be created by two ways
//1. using `insert` element
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
println!("Scores: {:?}", scores); // printing hash-Map

//2. using iterator and collect method that gathers data and `zip` 
// method to create vector tuple
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_values = vec![10, 50];
let mut scores2: HashMap<_, _> =
    teams.into_iter().zip(initial_values.into_iter()).collect();
println!("{:?}",scores2);

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);
println!("{:?}",map);

//for defining values in hashmap we use `insert`
//for accessing values in hashmap we use `get`
let mut scores1 = HashMap::new();
scores1.insert(String::from("Blue"),10);
scores1.insert(String::from("Yellow"),50);

let team_name = String::from("Blue");
let score = scores1.get(&team_name);
println!("{:?}", score); // `get` returns Option<&V> so result will be in `Some()`
//Note: If there was no Value than it would have returned `None`

//To iterate each value we use `for` loop
let mut scores3 = HashMap::new();
scores3.insert(String::from("blue"), 10);
scores3.insert(String::from("Yellow"), 50);
for (key, value) in scores3 {
    println!("{}: {}", key, value);
}

//updating value
//1. Overwriting values
let mut scores4 = HashMap::new();
scores4.insert(String::from("Blue"), 10);
scores4.insert(String::from("Blue"), 50);
println!("{:?}", scores4); //original value `10` has been overwritten by `50`

//2. Inserting key if they have no value using `entry`
let mut scores5 = HashMap::new();
scores5.insert(String::from("Blue"), 10);
scores5.entry(String::from("Yellow")).or_insert(50);
scores5.entry(String::from("Blue")).or_insert(50);
println!("{:?}", scores5); //`or_insert` returns mutable refrence
//Value is not gonna change for `Blue` 

//3. Updating value based on old value
let text = "Hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
println!("{:?}", map);




}
