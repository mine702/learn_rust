fn main() {
    listing_8_1();
    listing_8_2();
    listing_8_3();
    listing_8_4();
    listing_8_5();
}

fn listing_8_1() {
    let mut s = String::new();
}

fn listing_8_2() {
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
}

fn listing_8_3() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn listing_8_4() {
    let mut s = String::from("foo");
    s.push_str("bar");
}

fn listing_8_5() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);
    println!("s2 is {}", s2);
}

fn listing_8_6() {
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
}

fn listing_8_7() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);
}