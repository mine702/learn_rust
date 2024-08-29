use std::fmt::{Display, Formatter};

fn main() {
    let mut user1 = listing_5_1();
    println!("{}", user1);
    user1.email = String::from("user1@example.com");
    println!("{}", user1);

    let user2 = listing_5_3(String::from("user2@example.com"), String::from("some username123"));
    println!("{}", user2);

    let user3 = User {
        email: String::from("user3@exmaple.com"),
        username: String::from("another username567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user3);

    let user4 = User {
        email: String::from("user4@exmaple.com"),
        ..user3
    };
    println!("{}", user4);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{} {} {} {} {} {}", black.0, black.1, black.2, origin.0, origin.1, origin.2);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct User2 {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}, Email: {}, Sign-in count: {}, Active: {}",
               self.username, self.email, self.sign_in_count, self.active)
    }
}

fn listing_5_1() -> User {
    User {
        email: String::from("someone@example.com"),
        username: String::from("some username123"),
        active: true,
        sign_in_count: 1,
    }
}

fn listing_5_3(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}