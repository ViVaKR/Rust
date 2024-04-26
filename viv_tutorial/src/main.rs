use snippet::example::{array_ex, devide_by, std_fmt, Operator};
use std::{io, process, time::Instant};
use util::start;

extern crate communicator;
extern crate snippet;
extern crate util;

fn main() {
    clear_screen();
    let now = Instant::now();
    loop {
        start::display_menu();
        let choice: i32 = start::choice_menu();

        clear_screen();

        match choice {
            -1 => {
                println!("잘못된 메뉴 선택입니다.");
                continue;
            } /* [ -1. not a number ] */

            0 => {
                process::exit(0x0100);
            } /* [ 0. Exit ] */

            1 => {
                //
            } /* [ 1. Random ] */

            2 => {
                array_ex();
            } /* [ 2. Array ] */

            3 => {
                std_fmt();
            } /* [ 3. Format ] */

            4 => {
                let a: f64 = 3.14;
                let b: f64 = 0.54;

                let value = devide_by(a, b, Operator::Add);

                match value {
                    Some(rs) => {
                        println!("\u{26EC} {} + {} = {:.3}", a, b, rs);
                    }
                    None => println!("\u{26EC} 0",),
                }
            }

            _ => {
                continue;
            }
        }
        let elapse_time = now.elapsed();
        let elapse = elapse_time.as_secs();
        pause_screen(choice, elapse);
    }
}

fn pause_screen(choice: i32, elapse: u64) {
    let mut pause = String::new();
    println!(
        "\n\n\u{2728} ({} / {}초) 완료되었습니다. \u{2728}",
        choice, elapse
    );
    io::stdin().read_line(&mut pause).expect("Complete!");
    clear_screen();
}

fn clear_screen() {
    // println!("\x1b[2J\x1b[1;1H"); // clear screen & set cursor 1,1
    clearscreen::clear().unwrap();
}
