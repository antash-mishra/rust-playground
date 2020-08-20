#[derive(Debug)]
struct Person<'a> {   // Here 'a defines a life time
    name: &'a str,
    age: u8,
}

//unit struct
struct Unit;

//a tuple struct
struct Pair(f32,f32);

//a struct with two field
struct Point {
    x: f32,
    y: f32,
}

//struct used as a field of another structs
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user (username:String, email:String) -> User {
    User {
        email,
        username,
        active:true,
        sign_in_count: 1,
    }
}

fn rect_area(area: (f32,f32))-> f32 {
    let (length,breadth) = area;
    let rectangular_area = length * breadth;
    rectangular_area    
}  


fn main() {
    let name = "Antash";
    let age = 21;
    let peter = Person {name,age};
    // print the struct
    println!("{:?}", peter);

    let point: Point = Point {x: 10.3, y: 20.6};
    println!("THe co-ordinates are: {} {}", point.x, point.y);

    let bottom_right=  Point{x: 5.2, ..point};
    println!("second point: ({}, {})",bottom_right.x, bottom_right.y);

    let Point {x: top_edge, y:left_edge}=point;
    
    // Initiating 
    let _rectangle = Rectangle {
        top_left: Point {x:left_edge, y:top_edge},
        bottom_right: bottom_right,
    };
    //instiating a unit structure
    let _unit = Unit;

    //instiating a tuple structure
    let pair = Pair(0.1, 0.1);

    //Access the field of tuple struct 
    println!("The pair is {:?} {:?}",pair.0, pair.1);

    //Destruct the tuple structure
    let Pair(integer,decimal) = pair;

   println!("pair contains {:?} and {:?}", integer, decimal);
   
   let area = (2.0,3.0);
   println!("Area= {:?}", rect_area(area));
}

