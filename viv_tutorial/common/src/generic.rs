pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
pub fn generic_run() {
    let p = Point { x: 5, y: 10 };
    println!("\u{26EC} p.x = {}", p.x());

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("\u{26EC} The longest string is {}", result);
}

// --> Life Time
// 반환되는 참조작가 x를 참조하는지 혹은 y를 참조하는지 러스트가 알수 없기 때문에,
// 제네릭 라이프타임 파라미터를 추가하여 빌림 검사기가 분석을 수행할 수 있도록 하여야 함.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 시그니처 내의 모든 참조자들이 동일한 라이프 타임 'a를 가지고 있어야 함
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
