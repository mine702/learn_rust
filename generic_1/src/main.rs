fn main() {
    listing_10_1();
    listing_10_2();
    listing_10_3();
}

fn listing_10_1() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("가장 큰 숫자: {}", largest);
}

fn listing_10_2() {
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("가장 큰 숫자: {}", largest);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("가장 큰 숫자: {}", largest);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn listing_10_3() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("가장 큰 숫자: {}", result);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&numbers);
    println!("가장 큰 숫자: {}", result);
}