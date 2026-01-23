#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let penny = value_in_cents(Coin::Penny);
    println!("{}", penny);

    let nickel = value_in_cents(Coin::Nickel);
    println!("{}", nickel);

    let dime = value_in_cents(Coin::Dime);
    println!("{}", dime);

    let quarter = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", quarter);
}