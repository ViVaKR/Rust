use std::{collections::HashMap, io::Write};

/// A + B -7
pub fn bj_11021() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");

    let number = input.trim().parse::<usize>().unwrap();

    for i in 1..=number {
        let mut temp = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut temp).expect("fail to read");

        let array: Vec<usize> = temp
            .trim()
            .split('\u{0020}')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let a = array[0];
        let b = array[1];
        println!("Case #{}: {}", i, a + b);
    }
}

// 주사위 세개
pub fn bj_2480() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");

    let array: Vec<isize> = input
        .trim()
        .split('\u{0020}')
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    let mut groups: HashMap<isize, Vec<isize>> = HashMap::new();

    array.into_iter().for_each(|x| {
        let group = groups.entry(x).or_insert(vec![]);
        group.push(x);
    });

    let mut price: isize = 0;
    let mut max_value: isize = 0;
    let mut max_count = 0;
    let mut current_value = 0;

    for (id, g) in groups {
        if max_value < id {
            max_value = id;
        }
        if max_count < g.len() {
            max_count = g.len();
            current_value = id;
        }

        price = match max_count {
            1 => max_value * 100,
            2 => 1_000 + (current_value * 100),
            3 => 10_000 + (id * 1_000),
            _ => 0,
        };
    }
    println!("{}", price);
}

/// 사분면
pub fn bj_14681() {
    let mut array = vec![0; 2];
    for i in 0..2 {
        let mut input = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("fail to read");
        let temp = input.trim().parse::<isize>().unwrap();
        array[i] = temp;
    }

    let x = array[0];
    let y = array[1];
    let quadrat = match (x, y) {
        a if a.0 > 0 && a.1 > 0 => 1,
        a if a.0 > 0 && a.1 < 0 => 4,
        a if a.0 < 0 && a.1 > 0 => 2,
        a if a.0 < 0 && a.1 < 0 => 3,
        _ => 1,
    };
    println!("{var}", var = quadrat);
}

/// 알람시계
pub fn bj_2884() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");

    let array: Vec<isize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    let hour = if array[0] == 0 { 0 } else { array[0] * 60 };
    let minute = array[1];

    let temp: isize = hour + minute - 45;
    let total: isize = match temp {
        t if t < 0 => (24 * 60) + t,
        t if t == 0 => 0,
        t if t > 0 => t,
        _ => 0,
    };

    let h = total / 60;
    let m = total % 60;

    print!("{} {}\n", h, m);
}

/// 등록
pub fn bj_7287() {
    println!("{}", 35);
    println!("{}", "kimbumjun");
}

/// 가짜 평균
pub fn bj_1546() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");

    let len = input.trim().parse::<usize>().unwrap();

    let mut numbers = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut numbers)
        .expect("fail to read");

    let mut array: Vec<usize> = numbers
        .trim()
        .split(' ')
        .take(len)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    array.sort();
    let max = array[len - 1];

    let fmax = max as f64;

    let mut new_sum = 0.0;

    for i in 0..len {
        let val = array[i];
        let fv = val as f64;
        new_sum += fv / fmax;
    }

    let new_avg = new_sum / len as f64;
    let new_score = new_avg * 100.0f64;

    let prec = 6;

    println!("{:8.1$}", new_score, prec);
}

/// 숫자의 합
pub fn bj_11720() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");

    let len = input.trim().parse::<usize>().unwrap();

    let mut numbers = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut numbers)
        .expect("fail to read");

    let mut sum = 0;
    for letter in numbers.chars().take(len).into_iter() {
        let number = letter.to_digit(10).unwrap();
        sum += number;
    }

    println!("{}", sum);
}

/// 단어의 개수
pub fn bj_1152() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");
    let words = input.trim().split_whitespace().into_iter().count();
    println!("{}", words);
}

/// 별 찍기 - 7
pub fn bj_2444() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");
    let count = input.trim().parse::<usize>().unwrap();
    let star = '*';
    for i in 0..count {
        print!("{:>c$}", star, c = count - i);
        for _ in 0..i {
            print!("{}", star);
        }
        for _ in 0..i {
            print!("{}", star);
        }
        println!();
    }

    for i in (1..count).rev() {
        print!("{:>c$}", '\u{0020}', c = count - i);
        for _ in 0..i {
            print!("{}", star);
        }
        for _ in 1..i {
            print!("{}", star);
        }
        println!();
    }
}

/// 별찍기 - 1
pub fn bj_2438() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let count = input.trim().parse::<usize>().unwrap();
    let star: char = '*';
    for i in 1..=count {
        for _ in 0..i {
            print!("{}", star);
        }
        println!();
    }
}

/// 별찍기 - 2
pub fn bj_2439() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let count = input.trim().parse::<usize>().unwrap();
    let star: char = '*';
    for i in 0..count {
        print!("{:>1$}", star, count - i);
        for _ in 0..i {
            print!("{}", star);
        }
        println!();
    }
}

/// A + B - 5
pub fn bj_10952() {
    loop {
        let mut nums = String::new();
        std::io::stdout().flush().unwrap();

        let lines = std::io::stdin()
            .read_line(&mut nums)
            .expect("Failed to read");

        if lines != 4 {
            break;
        }

        let v: Vec<i32> = nums
            .trim()
            .split(' ')
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let a = &v[0];
        let b = &v[1];

        if a == &0 && b == &0 {
            break;
        };

        let sum = a + b;

        println!("{}", sum);
    }
}

/// A + B - 4
pub fn bj_10951() {
    loop {
        let mut nums = String::new();
        std::io::stdout().flush().unwrap();

        let lines = std::io::stdin()
            .read_line(&mut nums)
            .expect("Failed to read");

        if lines != 4 {
            break;
        };

        let v: Vec<i32> = nums
            .trim()
            .split(' ')
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let a = &v[0];
        let b = &v[1];
        let sum = a + b;
        println!("{}", sum);
    }
}

/// A + B - c
pub fn bj_10950() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let len = input.trim().parse::<usize>().unwrap();

    for _ in 0..len {
        let mut nums = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut nums)
            .expect("Failed to read");

        let v: Vec<usize> = nums
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let a = &v[0];
        let b = &v[1];
        let sum = a + b;
        println!("{}", sum);
    }
}

/// X보다 작은 수
pub fn bj_10871() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let len_filter: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut input_arr = String::new();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input_arr)
        .expect("Failed to read");

    let arr: Vec<usize> = input_arr
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    for item in arr.into_iter().take(len_filter[0]) {
        if item < len_filter[1] {
            print!("{} ", item);
        }
    }
    println!();
}

/// 중복 갯수
pub fn bj_10807() {
    let mut input_count = String::new();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input_count)
        .expect("Failed to read");

    let len = input_count.trim().parse::<usize>().unwrap();

    let mut input_array = String::new();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input_array)
        .expect("read error");

    let array: Vec<i32> = input_array
        .trim()
        .split(' ')
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut input_filter = String::new();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input_filter)
        .expect("Failed to read");

    let filter: i32 = input_filter.trim().parse().unwrap();

    let mut count = 0;
    for i in array.into_iter().take(len) {
        if i.eq(&filter) {
            count += 1;
        }
    }
    println!("{}", count);
}
/// 최소, 최대값
pub fn bj_10818() {
    let mut input_cn = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input_cn)
        .expect("Failed to read");

    let size = input_cn.trim().parse::<u32>().unwrap();

    let mut arr: Vec<i32> = Vec::new();
    let mut input_nums = String::new();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input_nums)
        .expect("Failed to read");
    let numbers: Vec<&str> = input_nums.trim().split(' ').collect();
    let mut idx = 0;
    for _ in 0..size {
        let item = (&numbers.get(idx)).unwrap();
        let number = item.parse::<i32>().unwrap();
        arr.push(number);
        idx += 1;
    }
    arr.sort();

    let min = arr.iter().min().unwrap();
    let max = arr.iter().max().unwrap();

    println!("{} {}", min, max);
}

/// 시험성적
pub fn bj_9498() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let rs = match input.trim().parse::<u8>().unwrap() {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("{}", rs);
}

/// 오븐시계
pub fn bj_2525() {
    let mut input_a = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input_a)
        .expect("Failed to read");
    let hour_minute: Vec<u32> = input_a
        .trim()
        .split(' ')
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut input_b = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input_b)
        .expect("Failed to read");

    let minute: u32 = input_b.trim().parse().unwrap();

    let hour = (&hour_minute[0] + ((&hour_minute[1] + minute) / 60)) % 24;
    let min = (&hour_minute[1] + minute) % 60;
    println!("{} {}", hour, min);
}

/// 피타고라스 정리
pub fn bj_7510() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let times = input.trim().parse::<u32>().unwrap();
    for i in 1..=times {
        let mut temp = String::new();
        std::io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read");

        let mut arr: Vec<u32> = temp
            .trim()
            .split(' ')
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        arr.sort();

        let sum_ab = u32::pow(arr[0], 2) + u32::pow(arr[1], 2);
        let sum_c = u32::pow(arr[2], 2);

        let result = if sum_ab.eq(&sum_c) { "yes" } else { "no" };

        println!("Scenario #{}:\n{}", i, result);
        println!("");
    }
}

// fn temp_a() {
// let mut arr: Vec<u32> = Vec::new();

// arr.push(85);
// arr.push(36);
// arr.push(77);
// arr.sort();

// println!(
//     "\u{26EC} {} == {}",
//     u32::pow(36, 2) + u32::pow(77, 2),
//     u32::pow(85, 2)
// );

// temp_a();
//     let mut v = vec![4, 3, 7, 1, 9];
//     for i in &mut v {
//         *i += 10; // * : 역참조 연산자.
//         println!("\u{26EC} {}", i);
//     }
//     println!("\u{26EC} = {}", &v[2]);
//     let x = 5;
//     let y = Box::new(x);
//     println!("\u{26EC} {:p}, {:p}, {}, {}", y, &x, *&x, *y);
// }

/// Draw
pub fn bj_10172() {
    let l1 = "|\\_/|";
    let l2 = "|q p|   /}";
    let l3 = "( 0 )\"\"\"\\";
    let l4 = "|\"^\"`    |";
    let l5 = "||_/=\\\\__|";

    println!("{}", l1);
    println!("{}", l2);
    println!("{}", l3);
    println!("{}", l4);
    println!("{}", l5);

    //     let dog = r#"
    // |\_/|
    // |q p|   /}
    // ( 0 )"""\
    // |"^"`    |
    // ||_/=\\__|
    // "#;
}

pub fn bj_10171() {
    let line1 = "\\    /\\";
    let line2 = " )  ( ')";
    let line3 = "(  /  )";
    let line4 = " \\(__)|";
    println!("{}", line1);
    println!("{}", line2);
    println!("{}", line3);
    println!("{}", line4);
}
/// 곱셈
pub fn bj_2588() {
    let mut input_first = String::new();
    let mut input_second = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input_first)
        .expect("Failed to read");

    std::io::stdin()
        .read_line(&mut input_second)
        .expect("Failed to read");

    let first: i32 = input_first.trim().parse::<i32>().unwrap();
    let second_org: i32 = input_second.trim().parse::<i32>().unwrap();
    let mut second = second_org.clone();
    println!("{}", first * (second % 10));
    second = second / 10;
    println!("{}", first * (second % 10));
    second = second / 10;
    println!("{}", first * (second % 10));
    println!("{}", first * second_org);
}
/// 나머지
pub fn bj_10430() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let array: Vec<i32> = input
        .trim()
        .split(' ')
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let a = array[0];
    let b = array[1];
    let c = array[2];
    let rs1 = (a + b) % c;
    let rs2 = ((a % c) + (b % c)) % c;
    let rs3 = (a * b) % c;
    let rs4 = ((a % c) * (b % c)) % c;
    println!("{}", rs1);
    println!("{}", rs2);
    println!("{}", rs3);
    println!("{}", rs4);
}

/// 메모장 만들기
pub fn bj_1406() {
    let mut input_str = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read");

    // commands
    let mut input_len = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input_len)
        .expect("Failed to read");

    let mut length: i32 = input_len.trim().parse().expect("Not a number");
    let mut left: Vec<char> = Vec::new();
    let mut right: Vec<char> = Vec::new();

    for c in input_str.trim().chars().into_iter() {
        left.push(c);
    }

    loop {
        if length == 0 {
            break;
        }

        let mut temp = String::new();

        std::io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read");

        let first_letter: char = match temp.chars().next() {
            Some(letter) => letter,
            None => continue,
        };

        let mut last_letter = first_letter.clone();
        if first_letter.eq(&'P') {
            let t: Vec<_> = temp.trim().split(' ').collect();
            last_letter = t[t.len() - 1].chars().last().unwrap();
        }

        match first_letter {
            'L' => {
                if left.last().is_some() {
                    let v = left.pop().unwrap();
                    right.push(v);
                }
            }
            'D' => {
                if right.last().is_some() {
                    let v = right.pop().unwrap();
                    left.push(v);
                }
            }
            'B' => {
                if left.last().is_some() {
                    left.pop().unwrap();
                }
            }
            'P' => {
                left.push(last_letter);
            }
            _ => continue,
        }
        length -= 1;
    }

    for _ in 0..right.len() {
        let r = right.pop().unwrap();
        left.push(r);
    }

    for item in left.into_iter() {
        print!("{}", item);
    }
    println!("");
}

/// 진법 변환 (2745)
pub fn bj_2745() {
    let mut str = String::new();
    std::io::stdin()
        .read_line(&mut str)
        .expect("Failed to read");

    let array: Vec<_> = str.trim().split(' ').collect();
    let number: String = array[0].trim().parse().expect("Wanted string");
    let base: i32 = array[1].trim().parse().expect("Wanted a number");

    let mut sum: i32 = 0;
    for (i, c) in number.trim().chars().rev().enumerate() {
        let m: i32 = if c.is_numeric() {
            c as i32 - '0' as i32
        } else {
            c as i32 - 'A' as i32 + 10
        };
        sum += m * base.pow(i as u32);
    }
    println!("{}", sum);
}
/// 윤년, Leap Year
pub fn bj_2753() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let year: usize = input.trim().parse().expect("Not a number");

    let result = match year {
        temp if (temp % 4 == 0 && temp % 100 != 0) => 1,
        temp if (temp % 4 == 0 && temp % 400 == 0) => 1,
        temp if (temp % 100 == 0 && temp % 400 != 0) => 0,
        temp if (temp % 400 == 0) => 1,
        _ => 0,
    };
    println!("{}", result);
}

/// 11382 : 데이터 타입 주의
pub fn bj_11382() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let array: Vec<_> = input.trim().split(' ').into_iter().collect();

    let a: i64 = array[0].trim().parse().expect("Wanted a number");
    let b: i64 = array[1].trim().parse().expect("Wanted a number");
    let c: i64 = array[2].trim().parse().expect("Wanted a number");
    let sum = a + b + c;
    println!("{}", sum);
}

/// 2743
pub fn bj_2743() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("{}", input.trim().len());
}

/// 1330
pub fn bj_1330() {
    let mut input = String::new();

    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let numbers: Vec<_> = input.trim().split(' ').into_iter().collect();

    let a: i32 = numbers[0].trim().parse().expect("Wanted a number");
    let b: i32 = numbers[1].trim().parse().expect("Wanted a number");

    let result = match a.cmp(&b) {
        std::cmp::Ordering::Less => "<",
        std::cmp::Ordering::Equal => "==",
        std::cmp::Ordering::Greater => ">",
    };

    println!("{}", result);
}

/// 2739. 구구단
pub fn bj_2739() {
    println!("\n\u{269E} 단을 입력하세요. \u{269F}");
    let mut input = String::new();
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let grade: u8 = input.trim().parse().expect("Not a number");

    for item in (1..=9).into_iter() {
        println!("{} * {} = {}", grade, item, grade * item);
    }
}

pub fn bj_18108() {
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

pub fn bj_10926() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let len = input.len();
    input.truncate(len - 1);
    println!("{}??!", input);
}

pub fn bj_10869() {
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

pub fn bj_1008() {
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
pub fn bj_10998() {
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
pub fn bj_2562() {
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

/// fmt
pub fn bj_fmt() {
    // fmt
    let temp = (3, 4);
    let fm = format!("{:?}, {}, {}", (3, 4), temp.0, temp.1);
    println!("\u{26EC} {}", fm);
    println!(
        "\u{26EC} {value} {zero}, {two}",
        value = 123,
        zero = 1,
        two = 2
    );

    let people = "Rustanceans";
    println!("\u{26EC} {people}");

    println!("\u{26EC} {:010}", 42);

    println!("\u{26EC} {:#?}", (100, 200.to_string()));
    println!("\u{26EC} {1} {} {0} {}", 1, 2);

    // identifier '=' expression
    println!("\u{26EC} {name} {}", 1, name = 2);

    let s = make_string(927, "label");
    println!("\u{26EC} {}", s);

    // width
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 10);
    println!("Hello {1:0$}!", 15, "x");
    let width = 25;
    println!("Hello {:width$}!", "x");

    // fill / alignment
    println!("Hello {:<17}!", "x"); // Hello x                !
    println!("Hello {:>width$}!", "x"); // Hello                         x!
    println!("Hello {:^width$}", "x"); // Hello             x
    println!("Hello {:-^width$}", "Y"); // Hello ------------Y------------
    println!("Hello {:-^width$.8}", 0.9); // Hello -------0.90000000--------

    // Sign
    println!("Hello {:+}, {:+}!", 5, -5);
    println!("Hello {:#x}, {:#b}, {:#o}!", 27, 27, 27); // Hello 0x1b, 0b11011, 0o33!
    println!("Hello {:#016x}!", 27);
    println!("27 {:#032b}", 27);

    println!("{:#032x}", 27);

    // 0x00000000000000000000000000001b
    // #?, #x, #X, #b, #0
    // ? => Debug
    // x? => Debug with lower-case hexadecimal intergers
    // X? => Debug with upper-case hexadecimal integers
    // o => Octal
    // x => LowerHex
    // X => UpperHex
    // p => Pointer
    // b => Binary
    // e => LowerExp
    // E => UpperExp

    // precision
    // 1. An integer .N:
    // 2. An integer or name followed by dollar sign .N$:
    // 3. An asterisk .*:

    println!("Hello {0} is {1:.16}", "x", 0.01);
    println!("Hello {1} is {2:.0$}", 16, "x", 3.141592);
    println!("Hello {} is {:.*}", "x", 5, 0.034);
    println!("Hello {1} is {2:.*}", 5, "x", 0.123);
    println!("Hello {} is {number:.prec$}", "x", prec = 16, number = 3.14);
    println!(
        "{}, `{name:>8.*}` has 3 right-aligned chararcters",
        "Hello",
        3,
        name = "1234.56"
    );

    //[[fill]align][sign]['#']['0'][width]

    // 소수점 포맷 (채우기 정렬방향(>^<) 부호(+) 채우는내용(0) 전체넓이(width$) .소수점아래길이(prec$))
    let wp = format!("{:>+0width$.prec$}", 23.3434, width = 32, prec = 1);
    println!("{}", wp);

    println!("{:->+width$.prec$}", 3.141592, width = 16, prec = 6);
    //포맷 (채울기호 정렬방향(>^<) 부호(+) '#' 채우는내용(0) 전체넓이(width$) .소수점아래길이(prec$) 'x, b, o')
    println!("{:>#0width$.prec$b}", 27, width = 32, prec = 2);

    //포맷 (채울기호 정렬방향(>^<) 부호(+) 채우는내용(0) 전체넓이(width$) .소수점아래길이(prec$))
    println!("{:->#width$.prec$b}", -1, width = 32, prec = 2);

    println!("{:#8X}", -1);
}

fn make_string(a: u32, b: &str) -> String {
    format!("{b} {a}")
}
