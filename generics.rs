use std::cmp::PartialOrd;
//use of generics on struct(It is of single type `T`)
struct Point<T> { //T is the generic type ie the elements 
    //in struct are of type T
    x: T,
    y: T,
}
//use of generics on struct(It is of two type `T, E`)
struct Points<T,U> { //The struct is of type T and U
    x: T, //It is of type T
    y: U, //It is of type U
}

impl<T,U> Points<T,U> {
    fn mixup<V,W>(self, other: Points<V,W>) -> Points<T,W> {
        Points {
            x: self.x,
            y: other.y,
        }
    } 
}

//use of generics in enum(single type)
enum Option<T> {
    Some(T),
    None,
}

//use of generic in enum(two types)
enum Result<T, E> {
    Ok(T),
    Err(E),
}

//use of generics in method definition
struct Pointer<T> {
    x:T,
    y:T,
}

impl<T> Pointer<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Pointer<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2)) + (self.y.powi(2)).sqrt()
    }
}


fn largest_number<T:PartialOrd + Copy>(number_list: &Vec<T>) -> T {
    
    let mut largest= number_list[0];
    for &item in number_list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


    fn main() {
    let numbers_list = vec![65,87,42,97,21];
    //println!("{}", largest_number(&numbers_list));
    let p = Pointer {x: 6.0, y: 8.0};
    println!("p.x = {}", p.x);
    println!("distance = {}", p.distance_from_origin());

    let p1 = Points {x: 5, y: 10.0};
    let p2 = Points {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x= {}, p3.y= {}", p3.x,p3.y);
}