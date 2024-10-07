fn main() {
    error_1();
}

fn error_1() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99];
}
