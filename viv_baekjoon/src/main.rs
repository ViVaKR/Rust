use std::env;

use baekjoon::{
    bj_1008, bj_10171, bj_10172, bj_10430, bj_10869, bj_10926, bj_10998, bj_11382, bj_1330,
    bj_1406, bj_18108, bj_2562, bj_2588, bj_2739, bj_2743, bj_2745, bj_2753,
};

mod baekjoon;
pub mod generic_ex;
pub mod trait_ex;
// use trait_ex::Book;
// use crate::{generic_ex::print_pro, trait_ex::Printable};
fn main() {
    // $ ./run.sh <baekjoon number>
    let args: Vec<String> = env::args().collect();
    let arg_1 = &args[1];
    let mut choice = 0;
    if let Ok(val) = arg_1.trim().parse::<i32>() {
        choice = val;
    }

    match choice {
        1008 => bj_1008(),
        1406 => bj_1406(), // 메모장
        1330 => bj_1330(),
        2562 => bj_2562(),
        2588 => bj_2588(),
        2739 => bj_2739(), // 구구단
        2743 => bj_2743(),
        2745 => bj_2745(), // 진법변환
        2753 => bj_2753(),
        10171 => bj_10171(), // 고양이
        10172 => bj_10172(),
        10430 => bj_10430(), // 나눗셈
        10869 => bj_10869(),
        10926 => bj_10926(),
        10998 => bj_10998(), // 곱셈
        11382 => bj_11382(),
        18108 => bj_18108(), // 불기 연도
        _ => std::process::exit(0x0100),
    }
}

/*
    --> trait : 특성, 여러구조에 걸쳐 표준 행동(방법) 을 구현, 인터페이스와 동일함.
*/

/*
    let x = 0b10101010u8;
    let y = !x;

    println!("x: {:0>8b}", x);
    println!("y: {:0>8b}", y);

    generic_ex::generic_run();

    let book = Book {
        id: 1001,
        name: "Rust in Action".to_owned(),
    };
    book.print();

    print_pro(10 as u8);
    print_pro("Hi Everyone");
*/
