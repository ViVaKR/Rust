pub fn make_string() {
    // 빈 스트링
    let mut s1 = String::new();

    // Display trait

    // literal
    let data = "contents ...";

    // Make String (to_string() == String::from("..."))
    // the method also works on a literal directly
    let mut s2 = data.to_string();
    let data = String::from(data);

    // Add string : push_str, push
    s1.push_str("foo ");
    s1.push_str("bar ");
    let l2 = "Hi";

    s2.push_str(&l2); // String Slice
    s2.push('E'); // push char

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // --> `+` 시스니처 -> fn add(self, s: &str) -> String {}
    // self 는 s1 의 소유권을 가져간후 &s2 강제 역참조를 실행함.
    let s3 = s1 + &s2;

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toc = String::from("toc");
    let tic_tac_toc = format!("{}-{}-{}", tic, tac, toc);
    println!(
        "\u{26EC}\ndata: {}, s3: {}\ntic tac toc: {}",
        data, s3, tic_tac_toc
    );

    // --> 러스트는 스트링 인덱싱을 지원하지 않음. -> (x) &lene[0]
    // String 은 Vec<u8> 로 wrapper 한 것임.
    let lene = String::from("Hola").len();

    // UTF-8 로 인코딩 되어 u8 값들로 Vec 으로 저장됨.
    // 유니코드 스칼라 값이 저장소의 3바이트를 차지함.
    let kor = "안녕하세요!";
    let lenk = String::from(kor).len();
    println!(
        "\u{26EC} Eng = {}, kor = {} ({}), &kor[0..6] = ({:#?})",
        lene,
        kor,
        lenk,
        &kor[0..6]
    );

    // --> 개별접근 방법
    for item in kor.chars() {
        println!("\u{26EC} {}", item);
    }

    for item in kor.bytes() {
        println!("\u{26EC} {}", item);
    }
}
