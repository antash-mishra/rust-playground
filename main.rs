    #[derive(Debug)]
    struct Structure(i32);
    
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    #[derive(Debug)]
    struct Deep(Structure);

    fn main() {
        let name = "Peter";
        let age = 27;
        let peter = Person {name , age};

        println! {"{:?}", peter};
        println! ("{:?} months in a year", 12);
        println! ("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christan", actor="actor's");

        //`structure` is printable!
        println!("Now {:?} will print!", Structure(3));
        println!("Now {:?} will print!", Deep(Structure(7)));
    }