#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn giveway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

pub fn add_one_v1(x: i32) -> i32 {
    x + 1
}

pub fn adds() -> Result<i32, ()> {
    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    let add_one_v3 = |y| y + 1;
    let add_one_v4 = |z| z + 1;

    let mut sum = 0;
    sum += add_one_v1(1);
    sum += add_one_v2(2);
    sum += add_one_v3(3);
    sum += add_one_v4(4);
    Ok(sum)
}
