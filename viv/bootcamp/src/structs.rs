use crate::interface::Print;

pub struct Data {
    pub id: i32,
    pub subject: String,
}

impl Print for Data {
    fn print(&self) {
        println!("\u{269E} {}\n\u{269E} {}", self.id, self.subject);

    }
}
