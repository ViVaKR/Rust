use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn capture_types() {
    // 타입명시
    let expensive_closuer = |num: u32| -> u32 {
        println!("\u{26EC} Calculating Slowly...");
        thread::sleep(Duration::from_secs(1));
        num * 2
    };

    let rs = expensive_closuer(32);
    println!("\u{26EC} Expensive Closuer {}", rs);

    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 2 };
    let add_one_v3 = |y| y;

    println!(
        "\u{26EC} v1: {}, v2: {}, v3: {}",
        add_one_v1(12),
        add_one_v2(13),
        add_one_v3(15)
    );
    // --> 값 캡처 방식
    // 1. 불변으로 빌려오기
    println!("\n\u{269E} 불변으로 빌려오기 \u{269F}");
    let list = vec![1, 2, 3];
    println!("\u{26EC} Before defining closure: {:?}", list);
    let only_borrows = || println!("\u{26EC} From closure{:?}", list);
    println!("\u{26EC} Before calling closure: {:?}", list);
    only_borrows();
    println!("\u{26EC} After calling closure: {:?}", list);

    // 2. 가변으로 빌려오기
    println!("\n\u{269E} 가변으로 빌려오기 \u{269F}");
    let mut list = vec![4, 5, 6];
    println!("\u{26EC} Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("\u{26EC} After caling closure: {:?}", list);

    // 3. 소유권 이동
    println!("\n\u{269E} 소유권 이동 \u{269F}");
    let list = vec![9, 2, 3, 4];
    println!("\u{26EC} Before defining closure: {:?}", list);
    thread::spawn(move || println!("\u{26EC} From thread: {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn sort_list() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|x| x.width);
    println!("\u{26EC} {:#?}", list);
}
