#![allow(unused)]
#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

// looping or iteration
// loop
// while

fn ifstatement() {
    let a = 1;
    let b = 2;
    let c = 3;

    if a > 9 {
        println!("very small number");
    } else {
        println!("Bin number");
    }
}
// Immutable by default, but can be mutable
// Immutable : cannot be changed
// Mutable : can be changed

// let c1 = 'a'; // 4byte
// let i= 125; // 4byte
// let d = 3.14; // 8byte
// let kor = '글'; // 4byte
// assert_eq!(size_of_val(&c1), 4);
// println!("{} {} {} {}", size_of_val(&c1), size_of_val(&i),size_of_val(&d), size_of_val(&kor));

// let x: u32 = 5;
// assert_eq!("u32".to_string(), type_of(&x));
// assert_eq!(i8::MAX, 127);
// assert_eq!(u8::MAX, 255);

// --
// let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
// assert_eq!(v, 1597);

// 0.1_f32, 0.1 as f32, 0.1f32
// assert_eq!(0.1f32 + 0.2f32, 0.3_f32); // 0.1 + 0.2 = 0.3000000000000001

// assert_eq, print lowercase char with i8 format
// let mut sum: i32 = 0;
// for i in -3..2 {
//     sum += i;
// }
// assert_eq!(sum, -5, "we are testing sum value {} equal -3", sum);
// for c in 'a'..='z' {
//     println!("{}\t{}", c, c as i8);
// }

// Range, RangeInclusive
// assert_eq!(1..5, Range { start: 1, end: 5 });
// assert_eq!(1..=5, RangeInclusive::new(1, 5));
// println!("\nSuccess!");

// Integer addition
// assert_eq!(1u32 + 2u32, 3u32);
// assert_eq!(1i32 - 2i32, -1i32);
// assert_eq!(1i8-2i8, -1);
// assert_eq!(3 * 50, 150);
// assert_eq!(9.6f32 / 3.2f32, 3.0f32);
// assert_eq!(24 % 5, 4);
// assert!(true && false ==true);
// assert!(true || false == true);
