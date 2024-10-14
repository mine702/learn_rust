fn main() {
    listening_10_4();
    listening_10_5();
    listening_10_6();
    listening_10_7();
    listening_10_8();
    listening_10_9();
    listening_10_10();
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn listening_10_4() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&numbers);
    println!("가장 큰 숫자: {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&chars);
    println!("가장 큰 문자: {}", result);
}

fn listening_10_5() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("가장 큰 숫자: {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("가장 큰 문자: {}", result);
}

fn listening_10_6() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // does not work
    // let wont_work = Point { x: 5, y: 4.0 };
}

fn listening_10_7() {
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
}

fn listening_10_8() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

fn listening_10_9() {
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn listening_10_10() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> T {
    // let mut largest = list[0];
    //
    // for &item in list {
    //     if item > largest {
    //         largest = item;
    //     }
    // }

    largest
}