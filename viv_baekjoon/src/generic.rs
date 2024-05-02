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
