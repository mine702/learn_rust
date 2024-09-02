fn main() {
    listening_5_7();
    listening_5_8();
    listening_5_9();
    listening_5_10();
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn listening_5_7() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(length1, width1)
    );
}

fn listening_5_8() {
    let rect1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn listening_5_9() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );
}

fn listening_5_10() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("rect1 is {:?}", rect1);
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area1(length: u32, width: u32) -> u32 {
    length * width
}