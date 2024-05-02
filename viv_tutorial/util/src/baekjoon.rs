#[allow(dead_code)]
#[allow(unused)]
use std::io::Write;

pub fn baekjoon_run() {
    println!("\u{26EC} 나눗셈:\t1008");
    println!("\u{26EC} 최댓값:\t2565");
    println!("\u{26EC} 사칙연산:\t10869");
    println!("\u{26EC} 놀 람:\t10926");
    println!("\u{26EC} 곱 셈:\t10998");
    println!("\u{26EC} 불기연도:\t18108");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let choice = match input.trim().parse::<usize>() {
        Ok(val) => val,
        Err(_) => todo!(),
    };

    match choice {
        10869 => baekjoon_10869(),
        1008 => baekjoon_1008(),
        2562 => baekjoon_2562(),
        10926 => baekjoon_10926(),
        10998 => baekjoon_10998(),
        18108 => baekjoon_18108(),
        _ => println!("\n\u{269E} - \u{269F}"),
    }
}

fn baekjoon_18108() {
    let buddha_year = 543;
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let year: i32 = input.trim().parse().expect("Wanted buddha year");
    println!("{}", year - buddha_year);

    /*
        2024년 부처님 오신 날
        ● 몇주년 : 불기 2568년
        ● 날짜 : 양력 2024년 5월 15일 수요일
        ● 날짜 : 음력 2024년 4월 8일 수요일
    */
}

fn baekjoon_10926() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let len = input.len();
    input.truncate(len - 1);
    println!("{}??!", input);
}

fn baekjoon_10869() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let digits: Vec<&str> = input.split(' ').into_iter().collect();
    let a: i32 = digits[0].trim().parse().expect("Wanted a number");
    let b: i32 = digits[1].trim().parse().expect("Wanted a number");

    println!("{}\n{}\n{}\n{}\n{}", a + b, a - b, a * b, a / b, a % b)
}

fn baekjoon_1008() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let digits: Vec<_> = input.split(' ').into_iter().collect();
    let a: f64 = digits[0].trim().parse().expect("Wanted a number");
    let b: f64 = digits[1].trim().parse().expect("Wanted a number");
    println!("{:16}", a / b);
}
/// 곱셈
fn baekjoon_10998() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let digits: Vec<_> = input.split(' ').into_iter().collect();
    let a: i32 = digits[0].trim().parse().expect("Wanted a number");
    let b: i32 = digits[1].trim().parse().expect("Wanted a number");
    println!("{}", a * b);
}

/// max value and order number
fn baekjoon_2562() {
    println!("\n\u{269E} BAEKJOON 2565 \u{269F}");
    let mut array: Vec<usize> = Vec::new();
    let mut index: usize = 0;
    let mut max = usize::MIN;
    let mut max_order: usize = 0;

    loop {
        if index == 9 {
            break;
        }
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();

        let value = input.trim().parse::<usize>();
        match value {
            Ok(val) => {
                array.push(val);
                if max < array[index] {
                    max = array[index];
                    max_order = index + 1;
                }
                index += 1;
            }
            Err(_) => continue,
        }
    }
    println!("{}\n{}", max, max_order);
}
