use std::io::Write;

/// 소수 찾기
pub fn bj_1978() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("fail to read");

    let length = input.trim().parse::<usize>().unwrap();

    let mut arr = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut arr).expect("fail to read");

    let array: Vec<u32> = arr
        .trim()
        .split('\u{0020}')
        .take(length)
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut prime_count = 0;
    for i in 0..length {
        let temp = array[i];
        let rs = is_prime(temp);
        if rs == true {
            prime_count += 1;
        }
    }
    println!("{}", prime_count);
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// 정렬
pub fn bj_10989() {
    let mut input = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Fail to read");
    let length: usize = input.trim().parse().expect("Not a number");
    let mut array = vec![0; 10001];

    for _ in 0..length {
        let mut input_a = String::new();
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input_a)
            .expect("Fail to read");
        let n: usize = input_a.trim().parse().expect("Not a number");
        array[n] += 1;
    }

    // let mut msg = String::new();

    for item in 0..10001 {
        if array[item] == 0 {
            continue;
        };
        for _ in 0..array[item] {
            println!("{}", &item);
        }
    }
}
