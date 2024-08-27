fn main() {
    slices_1();
    slices_2();
    slices_3();
    slices_4();
    slices_5();
}

fn slices_1() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
}

fn slices_2() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
    println!("{}{}", hello, world);
}

fn slices_3() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    let slice2 = &s[..2];

    let len = s.len();

    let slice3 = &s[3..len];
    let slice4 = &s[3..];

    let slice5 = &s[0..len];
    let slice6 = &s[..];

    println!("{} {} {} {} {} {}", slice1, slice2, slice3, slice4, slice5, slice6);
}

fn slices_4() {
    let my_string = String::from("hello world");

    // first_word가 `String`의 슬라이스로 동작합니다.
    let word1 = fixed_first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word가 스트링 리터럴의 슬라이스로 동작합니다.
    let word2 = fixed_first_word(&my_string_literal[..]);

    // 스트링 리터럴은 *또한* 스트링 슬라이스이기 때문에,
    // 아래 코드도 슬라이스 문법 없이 동작합니다!
    let word3 = fixed_first_word(my_string_literal);

    println!("{} {} {}", word1, word2, word3);
}

fn slices_5() {
    let mut s = String::from("hello world");
    let word = return_first_word(&s);
    println!("the first word is: {}", word);
}

fn fixed_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn return_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}