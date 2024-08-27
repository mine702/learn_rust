fn main() {
    ownership_4_1();
    ownership_4_2();
    ownership_4_3();
    ownership_4_4();
    ownership_4_5();
    ownership_4_6();
    ownership_4_7();
}

fn ownership_4_1() {
    let mut s = "hello";
    println!("{}", s);
    s = "world";
    println!("{}", s);
}

fn ownership_4_2() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn ownership_4_3() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}", s2);

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);
}

fn ownership_4_4() {
    let s = String::from("hello");
    let s1 = "hello";
    takes_ownership(s);
    takes_ownership(s1.to_string());
    println!("{}", s1);

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn ownership_4_5() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}, s3 = {}", s1, s3);
}

fn ownership_4_6() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn ownership_4_7() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length_ref(&s1);
    println!("The length of '{}' is {}.", s2, len);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_ref(s: &String) -> (String, usize) {
    let length = s.len();
    (s.to_string(), length)
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s.to_string(), length)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}