// #![allow(unused)]
// #[allow(dead_code)]

mod libs {
    pub mod demo;
}

mod func;

use crate::func::array_;
use crate::func::enum_;
use crate::func::func_;
use crate::func::if_;
use crate::func::loop_;
use crate::func::match_;
use crate::func::print_;
use crate::func::tupple_;
use crate::func::var_;
use crate::func::while_;
use crate::func::refs_;
use crate::func::struct_;
use crate::libs::demo::demo_;

use std::io;
use std::io::Write;



fn main() {
    let mut input: String = String::new();
    while input != "100" {
        println!();
        println!("***** 메뉴선택 (숫자) *****");
        println!("{:03}. Print", 1);
        println!("{:03}. 변수 1", 2);
        println!("{:03}. Match Statement", 3);
        println!("{:03}. Array", 4);
        println!("{:03}. Tupple", 5);
        println!("{:03}. Loop", 6);
        println!("{:03}. While Loop", 7);
        println!("{:03}. Enum", 8);
        println!("{:03}. Demo", 9);
        println!("{:03}. 변수 2", 10);
        println!("{:03}. 함수", 11);
        println!("{:03}. 참조", 12);
        println!("{:03}. struct", 13);

        println!("{:03}. Exit", 100);
        println!("**************************");
        print!(">> ");

        std::io::stdout().flush().unwrap();

        input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        // 화면정리
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("\n*****************************************\n");

        match input.trim_end() {
            "1" => print_(),
            "2" => var_(),
            "3" => if_(),
            "4" => match_(),
            "5" => array_(),
            "6" => tupple_(),
            "7" => loop_(),
            "8" => while_(),
            "9" => enum_(),
            "10" => demo_(),
            "11" => func_(),
            "12" => refs_(),
            "13" => struct_(),

            "100" => {
                println!("************* 프로그램 종료 *************");
                break;
            }
            _ => {
                println!("잘못된 입력입니다! 숫자만 선택하세요");
                break;
            }
        };
    }
}
