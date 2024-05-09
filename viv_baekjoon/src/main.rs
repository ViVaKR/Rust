use std::env;

use baekjoon::{
    bj_1008, bj_10171, bj_10172, bj_10430, bj_10807, bj_10818, bj_10869, bj_10871, bj_10926,
    bj_10950, bj_10951, bj_10952, bj_10998, bj_11021, bj_11382, bj_1152, bj_11720, bj_1330,
    bj_1406, bj_14681, bj_1546, bj_18108, bj_2438, bj_2439, bj_2444, bj_2480, bj_2525, bj_2562,
    bj_2588, bj_2739, bj_2743, bj_2745, bj_2753, bj_2884, bj_7287, bj_7510, bj_9498, bj_fmt,
};
use baekjoon_b::{bj_11022, bj_11654, bj_27866, bj_8393};
use baekjoon_c::{bj_10989, bj_1978};
use playground::playground;
mod baekjoon;
mod baekjoon_b;
mod baekjoon_c;
pub mod generic_ex;
pub mod playground;
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
        1 => bj_fmt(),
        2 => playground(),
        1008 => bj_1008(),
        1152 => bj_1152(),
        1330 => bj_1330(),
        1406 => bj_1406(), // 메모장
        1546 => bj_1546(), // 평균
        1978 => bj_1978(), // 소수찾기
        2438 => bj_2438(),
        2439 => bj_2439(),
        2444 => bj_2444(),
        2480 => bj_2480(),
        2525 => bj_2525(),
        2562 => bj_2562(),
        2588 => bj_2588(),
        2739 => bj_2739(), // 구구단
        2743 => bj_2743(),
        2745 => bj_2745(), // 진법변환
        2753 => bj_2753(),
        2884 => bj_2884(), // 알람시계
        7287 => bj_7287(), // 등록
        7510 => bj_7510(), // 직각 삼각형
        8393 => bj_8393(),
        9498 => bj_9498(),
        10171 => bj_10171(), // 고양이
        10172 => bj_10172(),
        10430 => bj_10430(), // 나눗셈
        10807 => bj_10807(),
        10818 => bj_10818(), // 최소, 최대
        10869 => bj_10869(),
        10871 => bj_10871(),
        10926 => bj_10926(),
        10950 => bj_10950(), // A + B - 3
        10951 => bj_10951(), // A + B - 4
        10952 => bj_10952(), // A + B - 5
        10989 => bj_10989(), // 수 정렬하기 3
        10998 => bj_10998(), // 곱셈
        11021 => bj_11021(), // A + B - 7
        11022 => bj_11022(), // A + B - 8
        11382 => bj_11382(),
        11654 => bj_11654(), // ascii
        11720 => bj_11720(), // 숫자의 합
        14681 => bj_14681(), // 사분면 고르기
        18108 => bj_18108(), // 불기 연도
        27866 => bj_27866(), //
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
