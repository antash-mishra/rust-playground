use std::ops::Deref;
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSMartPointer {}", self.data);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; //define associated type for Deref trait

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::new(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&b));
    {
        let c = Rc::new(4, Rc::new(&b));
        println!("count after creating c = {}", Rc::strong_count(&c));
    }

    println!("count after c goes out of scope {}", Rc::strong_count(&a));
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
}