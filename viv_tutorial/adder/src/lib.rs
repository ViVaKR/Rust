pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        let result = add(43, 57);
        assert_eq!(result, 100);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        // 큰 사각형이 작은 사각형을 정말로 담을 수 있는지 검사.
        //true 검사
        assert!(larger.can_hold(&smaller));

        // 동치 테스트 : assert_eq!, assert_ne!
        // should_panic : 패닉 테스트.
    }
}
