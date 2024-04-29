pub fn iter_repeat() {
    // demonstration
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for item in v1_iter {
        println!("\u{26EC} Got: {}", item);
    }

    let v2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum();
    println!("\n\u{26EC} Sum: {}\n", total);
}

pub fn iter_map() {
    println!("\n\u{269E} 다른 반복자를 생성하는 메서드 map \u{269F}");
    let v1: Vec<i32> = vec![2, 4, 6, 8];
    let v2: Vec<_> = v1.iter().map(|x| x * 3).collect();

    for item in v2 {
        println!("\u{26EC} {} ", item);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size).collect()
}

pub fn iter_filter() {
    println!("\n\u{269E} 환경을 캡처하는 클로저 filter \u{269F}");
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 17,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    let vec_key: Vec<_> = in_my_size.into_iter().map(|x| x.style).collect();

    for item in vec_key {
        println!("\u{26EC} Style -> {}, Size -> {}", item, 10);
    }
}
