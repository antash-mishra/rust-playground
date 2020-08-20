//Errors are catogarizied into two types: 
//1. Recovarable  2. Unrecoverable

use std::fs::File;
use std::io;
use std::io::{Read, ErrorKind};
use std::fs;
fn read_username_from_file()-> Result<String, io::Error> {
    let mut f = File::open("txt5.txt").expect("Not found");
    let mut s = String::new();    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}

//use of `?` operator for error-handling
fn read_username__from_file2 () -> Result<String, io::Error> {
    let mut open_file = File::open("cargo.toml")?;
    let mut read_string = String::new();
    open_file.read_to_string(&mut read_string)?;
    println!("{}", read_string);
    Ok(read_string)
}

fn read_username__from_file3 () -> Result<String,io::Error> {
    let mut s =String::new();
    File::open("txt5.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

//use of `std::fs`
fn read_username__from_file4 () -> Result<String, io::Error> {
    fs::read_to_string("txt5.txt")
}

fn main() {

    //let v = vec![1,2,3];
    //v[99];

    //let f = File::open("text5.txt");

    //error handling using `match` expression
    let x = File::open("txt5.txt");
    let x = match x {
        Ok(file) => file,
        Err(error) => panic!("File cannot be opened {:?}", error),
    };

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Couldn't create file: {:?}", error),
            },
            other_error => {
                panic!("Problem opening file: {:?}", other_error)
            }
        },
    };

    //More precise way
    let fx = File::open("hell.txt").unwrap_or_else(|error| {
        if error.kind()==ErrorKind::NotFound {
            File::create("hell.txt").unwrap_or_else(|error| {
                panic!("Couldn't create: {:?}", error);
            })
        } else {
            panic!("problem opening the file: {:?}", error);
        }
    });

    //Use of `unwrap` for error handling
    let file_open = File::open("text").unwrap();

    //use of `expect` for error handling
    let file_open2 = File::open("display").expect("Not found");

    //call `read_username__from_file` fn 
    read_username_from_file();

    //use of `?` operator for error handling
    read_username__from_file2();

    //More shorter/precise
    read_username__from_file3();

    //most precise way/ use of `fs::read_to_string()`
    read_username__from_file4(); // It doesn't explain error handling



}


/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
T => It returns success case within `Ok` variant
E => It returns failure case within `Err` variant
*/

//`?` operator is used in function that returns Result