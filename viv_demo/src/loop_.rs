use std::io;

pub fn loop_() {
    println!("***** loop *****");
    let mut n = 0;
    loop {
        if n > 30 {
            break;
        };
        n += 1;
        if n == 7 {
            continue;
        }
        println!("n = {}", n);
    }
}
