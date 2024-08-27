fn main() {
    referrals_1();
    referrals_2();
    referrals_3();
    referrals_4();
    referrals_5();
    referrals_6();
}

fn referrals_1() {
    let s1 = String::from("So how do I say goodbye to someone who's been with me for my whole damn life?");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn referrals_2() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn referrals_3() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    //let r2 = &mut s;
    println!("{}", r1);
}

fn referrals_4() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);
}

fn referrals_5() {
    let mut s = String::from("referrals_5");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

fn referrals_6() {
    let s = no_dangle();
    println!("{}", s);
}

/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

 */
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/