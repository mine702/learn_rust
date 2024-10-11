use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::net::IpAddr;

fn main() {
    // error_1();
    // error_2();
    // error_3();
    // error_4();
    read_username_from_file();
}

fn error_1() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}

fn error_2() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("파일을 열 수 없습니다: {:?}", error)
        }
    };
}

fn error_3() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일을 생성할 수 없습니다: {:?}", e),
            }
        }
        Err(error) => {
            panic!("파일을 열 수 없습니다: {:?}", error)
        }
    };
}

fn error_4() {
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("파일을 열 수 없습니다.");
}

fn error_5() {
    let home = "127.0.0.1".parse::<IpAddr>().unwrap();
}

fn error_6() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("숫자를 1과 100 사이로 입력해주세요.");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}