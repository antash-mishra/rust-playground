#[derive(Debug)]
enum UsState {
    NY,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Sttae quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::NY));
    fn plus_one(x:Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}",five);
    println!("{:?}", six);
    println!("{:?}", none);

}