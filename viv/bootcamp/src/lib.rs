use closure::{Inventory, ShirtColor};
use core::panic;
use interface::Print;
use std::{fs, path::Path, thread};
use structs::Data;
use usererror::print_result;

use crate::{result::pulling_results, usererror::double_first};
pub mod closure;
pub mod interface;
pub mod result;
pub mod structs;
pub mod usererror;

pub fn closure_run() {
    //
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveway1 = store.giveway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveway1
    );

    let user_pref2 = None;
    let giveway2 = store.giveway(user_pref2);
    println!(
        "Ther user with preference {:?} gets {:?}",
        user_pref2, giveway2
    );
}

/// trait
pub fn print_data(data: Data) {
    data.print();
}

/// # text reader
pub fn read_text(file_path: &str) {
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Display` able structure
    let _display = path.display();

    // `join` merges a path with a byte container using the OS specific
    // separator, and returns a `PathBuf`
    let mut new_path = path.join("assets");

    // `push` extends the `PathBuf` with a `&Path`
    new_path.push("text");
    new_path.push(file_path);

    // `set_file_name` update the file name of the `PathBuf`
    new_path.set_file_name(file_path);

    // Convert the `PathBuf` into a string slice
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => {
            let content = fs::read_to_string(s).expect("not found");
            println!("{} With Test:\n{}", new_path.display(), content);

            let result = search("Add", &content);

            println!("result = {}", result.len());
        }
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn array_check() {
    let numbers = vec!["42", "93", "18"];
    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", pulling_results(numbers));
    println!("The first doubled is {:?}", pulling_results(strings));
}

pub fn user_error() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print_result(double_first(numbers));
    print_result(double_first(empty));
    print_result(double_first(strings));
}

pub fn vector_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for i in v1_iter {
        println!("Get: {}", i);
    }
}

/// # Iterator
pub trait Iterator {
    // 트레이트에 대한 연관 타입(associated type)
    //
    // next 메서드의 반환 타입
    // Item 타입은 반복자로 부터 반환되는 타입이 됨.
    type Item;

    // Iterator 트레이트는 구현하려는 이에게 딱 하나의 메서드 정의를 요구함.
    fn next(&mut self) -> Option<Self::Item>;
}

#[test]
/// # iterator_demonstration
/// $ cargo test -p bootcamp iterator_demonstration
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    //
    let v1_iter = v1.iter(); // 가변으로 만들어야 함.

    // next 는 반복자를 소비함.
    // next : 불변 참조자.
    // into_next : 가변 참조자.
    // => 같은 맥락으로 가변 참조자에 대한 반복작가 필요하면 inter 대신 iter_mut 를 사용할 수 있음.
    // for loop 를 사용할 때는 v1_iter 를 가변으로 만ㄷ르 필요가 없음 :
    // =>  루프가 v1_iter 의 소유권을 갖고 내부적으로 가변으로 만들기 때문.
    let total: i32 = v1_iter.sum(); // sum 은 반복자를 소비함으로 호출한 후 v1_iter 의 사용이 허용되지 않음.
    assert_eq!(total, 6);

    let mut v1_iter2 = v1.iter();
    assert_eq!(v1_iter2.next(), Some(&1));
    assert_eq!(v1_iter2.next(), Some(&2));
    assert_eq!(v1_iter2.next(), Some(&3));
    assert_eq!(v1_iter2.next(), None);

    let v2: Vec<i32> = vec![1, 3, 5, 7, 9];

    // collect 메서드 : 반복자를 소비하고 결과값을 모아서 컬렉션 데이터 타입으로 만들어줌.
    let v2_map: Vec<_> = v2.iter().map(|x| x + 1).collect();
    assert_eq!(v2_map, vec![2, 4, 6, 8, 10]);
}

// --> 환경을 캡처하는 클로저
// 1. 많은 반복자 어댑터는 클로저를 인수로 사용함.
// 2. 보통 반복자 어댑터의 인수에 명시되는 클로저는 자신의 환경을 캡처하는 클로저임.
// --> filter : 반복자로 부터 아이템을 받아서 bool 을 반환, 만일 클로저가 true 를 반환하면, 그 값을 필터에 의해 생성된 반복자에 포함 시킴.

/// shoe_size 를 캡처하는 클로저의 필터를 사용하여 Shoe 구조체 인스턴스의 컬렉션을 순회함.
/// 이는 지정된 크기의 신발만 반환해 줌.
#[derive(PartialEq, Debug)]
pub struct Shoe {
    size: u32,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sanal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            }
        ]
    )
}

/// 파일로 부터 특정 문자 검색
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|x| x.contains(query)).collect()
}

#[cfg(target_os = "linux")]
pub fn are_you_on_linux() {
    println!("==> Yes. It's definitely linux");
}

#[cfg(not(target_os = "linux"))]
pub fn are_you_on_linux() {
    println!("==> Yes. It's definitely *not* linux");
}
pub fn cfg_attr() {
    are_you_on_linux();
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux");
    } else {
        println!("Yes. It's definitely *not* linux");
    }
}

const NTHREADS: u32 = 10;

pub fn thread_run() {
    let mut children = vec![];
    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }

    // This is our data to process.
    // We will calculate the sum of all digits via a threaded map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    //
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];

    let chunked_data = data.split_whitespace();
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);
        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();

            println!("Processed segment {}, result = {}", i, result);
            result
        }));
    }

    let final_result = children.into_iter().map(|x| x.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//         assert_eq!(3, add_one(2));
//     }
// }
