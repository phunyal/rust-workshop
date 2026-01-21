enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... other states can be added here
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {

    println!("{:?}", Coin::Dime);
    println!("Value of a dime: {} cents", value_in_cents(Coin::Dime));
    println!("Value of a quarter: {} cents", value_in_cents(Coin::Quarter));
    println!("Value of a nickel: {} cents", value_in_cents(Coin::Nickel));
    println!("Value of a penny: {} cents", value_in_cents(Coin::Penny));
    Coin::Quarter(String::from("Nepal"));
}