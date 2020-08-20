use std::io;
use text_io::read;
use std::env;

fn main() {
    let mut subreddit: Vec<String> = Vec::new();
    let mut counter = 0;
    loop {
        let mut input = String::new();
        println!("Add number");
        io::stdin().read_line(&mut input)
            .expect("Error reading input");
        subreddit.push(input);
        counter += 1;
        if counter == 3 {
            break;
        }
        }
        println! ("{}", subreddit[0]);
}
