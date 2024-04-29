/* 트레이트
- 특정한 타입을 가지고 공유할 수 있는 기능 정의
- 공통된 기능을 추상적으로 정의할 수 있음.
- 인터페이스 기능과 유사함.

*/

/// 트레이트 선언부
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headlinie: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headlinie, self.author, self.location)
    }
}

pub struct Tweet {
    pub name: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.content)
    }
}

pub fn runner() {
    let tweet = Tweet {
        name: String::from("horse_ebooks"),
        content: String::from("of course , as you probably already know people"),
        reply: false,
        retweet: false,
    };
    println!("\u{26EC} I new tweet {}", tweet.summarize());
    let article = NewsArticle {
        headlinie: String::from("Penguins win the Stanley Cup Championship"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the HHH",
        ),
    };
    println!("\u{26EC} New article available! {}", article.summarize());
}
