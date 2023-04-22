use std::io;

pub fn loop_() {
    println!("***** loop *****");
    let mut n = 0;
    loop {
        n += 1;
        println!("n = {}", n);

        if n > 10 {
            break;
        };
    }
}
