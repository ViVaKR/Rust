use std::{
    env::current_dir,
    fs::File,
    io::{self, ErrorKind, Read},
};
pub fn panic_run() {
    let f = File::open("demo.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref err) if err.kind() == ErrorKind::NotFound => match File::create("demo.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(err) => {
            panic!("There was a problem opening the file: {:?}", err)
        }
    };

    println!("\u{26EC} {:?}", f);

    // let f2 = File::open("hello.txt").expect("hello.txt not exist");
    // println!("\u{26EC} hello.txt = {:?}", f2);
}

pub fn result_run() -> Result<String, io::Error> {
    println!("\u{26EC} Path: {:?}", current_dir());
    let f = File::open("run.sh");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub struct Guess {
    pub value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}
