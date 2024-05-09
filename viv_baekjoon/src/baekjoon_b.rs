use std::io::{Read, Write};
/// A + B - 8
pub fn bj_11022() {
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
        println!("Case #{}: {} + {} = {}", i, a, b, a + b);
    }
}

/// 문자와 문자열
pub fn bj_27866() {
    //
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");

    let mut read_num = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut read_num)
        .expect("fail to read");

    let index = read_num.trim().parse::<usize>().unwrap();

    let c = input.chars().nth(index - 1).unwrap();
    println!("{}", c);
}

// Ascii
pub fn bj_11654() {
    let c = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|x| x as u8)
        .unwrap();
    println!("{}", c);
}

/// 합
pub fn bj_8393() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");

    let number = input.trim().parse::<usize>().unwrap();

    let mut sum = 0;

    for i in 1..=number {
        sum += i;
    }
    println!("{}", sum);
}
