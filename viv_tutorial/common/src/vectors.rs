use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_vectors() {
    // 범위로 벡터 생성
    let mut vec: Vec<u32> = (1..=10).collect();

    // 평균
    let sum = vec.iter().sum::<u32>() as f32;
    let count = vec.len();
    let mean = match count {
        x if x > 0 => Some(sum / count as f32),
        _ => None,
    };

    // 무작위로 섞기
    vec.shuffle(&mut thread_rng());
    match mean {
        Some(val) => println!("\u{26EC} mean: {}", val),
        None => todo!(),
    }
    println!("\u{26EC} {:?}, sum {}", vec, sum);
}
