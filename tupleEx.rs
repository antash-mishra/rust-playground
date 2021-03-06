use std::fmt;

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    } 
}

fn reverse(pair: (i32,bool)) -> (bool,i32) {
    let (integer,boolean)=pair;
    (boolean,integer)
}

fn transpose(matrix_eq: (f32,f32,f32,f32))-> (f32,f32,f32,f32) {
    let (x11,x12,x21,x22)=matrix_eq;
    (x11,x21,x12,x22)
}

fn main() {
    let long_tuples = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);
    
    println!("long tuple first value: {}", long_tuples.0);
    println!("long tuple second value: {}", long_tuples.1);

    let tuple_of_tuple =((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuple: {:?}", tuple_of_tuple);

    let pair =(1,true);
    println!("pair is: {:?}",pair);
    println!("reversed pair is: {:?}",reverse(pair));

    println!("one element tuple: {:?}",(5u32,));
    println!("Just an integer: {:?}",(5u32));

    let tuple = (1,"hello", 4.5, true);
    let (a,b,c,d)=tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("matrix:\n{}", matrix);
    let matrix_eq= (1.1,1.2,2.1,2.2);
    println!("Transpose:\n({} {})\n({} {})", transpose(matrix_eq).0, transpose(matrix_eq).1, transpose(matrix_eq).2, transpose(matrix_eq).3);
    

}