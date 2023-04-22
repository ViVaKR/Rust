#![allow(unused)]

pub fn tupple_() {

    println!("Hello Tupple");
    let my_tupple:(u8, String, f64) = (47, "Viv".to_string(), 50_000.34);
    println!("Age: {}, Name : {}, Etc : {}", my_tupple.0, my_tupple.1, my_tupple.2);

    let(age, name, etc) = my_tupple;
    println!("Age: {}, Name : {}, Etc : {}", age, name, etc);
}

