use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    print!("Input Your Messages");
    print!(">> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Input String");

    if let Ok(var) = input.trim().parse::<u32>() {
        run(var);
    } else {
        println!("Invalid Input..");
    }
}

fn run(menu: u32) {
    match menu {
        1 => {
            // loop - infinite loop
            let mut i = 1;
            loop {
                println!("i is {i}");
                if i > 100 {
                    break;
                }
                i *= 2;
            }
            assert_eq!(i, 128);

            // while - conditional loop
            let mut counter = 0;
            while counter < 10 {
                println!("{counter}");
                counter += 1;
            }
        }
        2 => {
            println!("Hello 2");
        }
        _ => {
            println!("default");
        }
    }
}
