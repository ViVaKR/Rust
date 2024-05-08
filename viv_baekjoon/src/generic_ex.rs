use std::fmt::Display;

pub struct Size<T, U> {
    pub x: T,
    pub y: U,
}

pub fn make_point() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.3, y: 3.14 };

    let size_a = Size { x: 5, y: 3.14 };
}

pub fn generic_run() {
    let data: Data<i32> = Data { value: 123 };
    let data_str: Data<String> = Data {
        value: String::from("Hello, World"),
    };
    println!("Data<i32>: {}, Data<String> {}", data.value, data_str.value);
}

pub struct Data<T> {
    pub value: T,
}

pub fn print_pro<T: Display>(t: T) {
    // println 에 의해 값을 인쇄할 수 있도록
    // Display 특성을 구현.

    println!("\u{26EC} Inside print_pro generic function:");
    println!("\u{26EC} {}", t);
}
