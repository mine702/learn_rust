fn main() {
    listening_6_1();
    listening_6_2();
    listening_6_4();
    listening_6_5();
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn listening_6_1() {
    let coin = Coin::Penny;
    let value = value_in_cents(coin);
    println!("The value of the coin is: {}", value);
}

fn listening_6_2() {
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("The value of the coin is: {}", value);
}

fn listening_6_4() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);
}

fn listening_6_5() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
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
            println!("State quarter from: {:?}!", state);
            25
        }
    }
}