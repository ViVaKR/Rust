/*
    - 구문 : 어떤 동작을 수행하고 값을 반환하지 않는 명령.
    - 표현식 : 결과값을 평가함.
*/
pub fn function() {
    let y = five(false); // 구문
    let z = {
        let x = 'A';
        x
    }; // 표현식

    another_function(y, z);
}

pub fn another_function(x: i32, label: char) {
    // -->
    let mut rs = five(true);
    if rs & 2 == 0 {
        rs = rs + 45;
    }

    println!("The value of x is: {x} {label} - {rs}");
}

fn five(tf: bool) -> i32 {
    let number = if tf { 11 } else { 100 };
    plus_one(number)
}

fn plus_one(x: i32) -> i32 {
    x + 100
}

pub fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic
// pub fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
