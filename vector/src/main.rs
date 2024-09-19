fn main() {
    listing_8_1();
    listing_8_2();
    listing_8_3();
    listing_8_4();
    listing_8_5();
    listing_8_6();
    listing_8_7();
    listing_8_8();
    listing_8_9();
    listing_8_10();
    listing_8_11();
    listing_8_12();
    listing_8_13();
    listing_8_14();
}

fn listing_8_1() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
}

fn listing_8_2() {
    let v = vec![1, 2, 3];
    println!("{}", v.get(1).unwrap_or(&0));
}

fn listing_8_3() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn listing_8_4() {
    let v = vec![1, 2, 3, 4];

    // v를 가지고 뭔가 합니다
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
} // <- v가 스코프 밖으로 벗어났고, 여기서 해제됩니다

fn listing_8_5() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    //let third: Option<&i32> = v.get(2);
    println!("The third element is {}", third);
}

fn listing_8_6() {
    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    //let does_not_exist = v.get(100);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn listing_8_7() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
}

fn listing_8_8() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

fn listing_8_9() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn listing_8_10() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn listing_8_11() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
}

fn listing_8_12() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

fn listing_8_13() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);
}

fn listing_8_14() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}