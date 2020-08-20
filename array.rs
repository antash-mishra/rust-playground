use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of slice: {}",slice[0]);
    println!("slice has elements: {}",slice.len());
}

fn main() {
    //Fixed array size 
    let xs: [i32;5]=[1,2,3,4,5];

    //initializing the entire list by 1 
    let ys: [i32;500]=[1;500];
   
    //indexing starts at 0
    println!("First element of list xs: {}",xs[0]);
    println!("second element of list xs: {}",xs[1]);
    
    //len() shows size of array
    println!("Size of array: {}",xs.len());

    println!("Array occupies {} bytes",mem::size_of_val(&xs));
    
    println!("Borrow section of array as slice.");
    analyze_slice(&ys[1 .. 4]);
}

