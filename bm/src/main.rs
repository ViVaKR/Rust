use rand::Rng;
use std::io;

fn main() {
    println!("\u{26EC} Guess the number{}", "!");
    // 비밀번호 생성하기, 1~100 사이의 랜덤 숫자 생성
    // rand 라이브러리 (crate)를 사용하기 위해 Cargo.toml 에 추가해야함.
    // $ cargo add rand
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("\u{26EC} The secret number is : {}", secret_number);

    loop {
        println!("\u{26EC} Please input your guess{}", "!");

        // String : growable UTF-8 encoded text
        // ::new() : associated function of String, 연관함수, 비어있는 새 문자열을 생성.
        // new 는 어떤 새로운 값을 만드는 함수 이름으로 흔히 사용됨.
        let mut guess = String::new();

        io::stdin()
            // `&` 는 기본적으로 불변 참조자를 만듬. (reference) 코드의 여러부분에 같은 값을 여러번 읽을 때 유용. 안전성과 편의성 제공
            // `&mut` 는 가변 참조자를 만듬. (mutable reference) 여러 부분에서 같은 값을 여러번 쓸 때 유용.
            .read_line(&mut guess)
            // 잠재적 에러를 처리하기 위해 Result 타입을 반환함.
            // Result 타입은 열거형(enum) 이며, Ok 또는 Err 두 가지의 variant 가 있음.
            // Result 인스턴스가 Err 이면 expect 함수는 프로그램을 크래시하고 Err 에 담긴 메시지를 출력함.
            .expect("Failed to read line");

        // {} : placeholder, 문자열에 값이 들어갈 자리를 의미함.
        // shadow (guess) : 같은 이름의 변수를 여러번 정의할 수 있음.
        // trim() : 문자열의 앞뒤 공백을 제거함. (whitespace), 숫자형 데이터만 저장할 수 있는 u32 와 문자열을 비교할 수 있게 하기 위해 필수.
        // ex : read_line 을 끝내기 위해 enter 키를 반드시 눌러야 하고 이것이 개행 문자를 문자열에 추가시킴.
        //      5를 입력하면 "5\n" 가 되고, 이것은 숫자가 아니기 때문에 parse 함수가 실패함.
        //      trim 메서드는 \n 또는 \r\n (윈도우) 을 제거하고 5만 남도록 처리함.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\u{26EC} Please input valied number!");
                continue;
            }
        };
        println!("\u{26EC} You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("\u{26EC} Too small!"),
            std::cmp::Ordering::Greater => println!("\u{26EC} Too big!"),
            std::cmp::Ordering::Equal => {
                println!("\u{26EC} You win!");
                break;
            }
        }
    }
}
