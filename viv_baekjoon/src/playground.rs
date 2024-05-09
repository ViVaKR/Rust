pub fn playground() {
    // let s = dangle();

    let mut s = String::from("Hello World");
    let i = first_word(&s);
    println!("{}", i);
    s.clear(); // s == ""

    let s = String::from("Hello World");
    let hello = &s[0..5];
    let world = &s[6..];

    println!("{} {}", hello, world);

    let s = String::from("hello");
    let len = s.len();

    println!("{} {}, {}, {:?}", len, &s[..len], &s[..], &s.as_bytes());
}

// fn dangle() -> String {
//     let s = String::from("hello");
//     // &s // 실체가 없으므로 오류.
//     s
// }

/*
    --> slice, 슬라이스 : 참조자의 일종으로 소유권을 갖지 않음.
*/

/// 문자속의 공백 문자 찾기
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // iter : collection 의 각 요소를 반환
    // enumerate : iter 의 각 결과값을 튜플로 감싸서 반환 -> (요소의 인덱스, 요소의 참조자)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
