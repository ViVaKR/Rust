pub trait VivTrait {
    // abstract or method which is empty
    fn tr_method(&self);
}

pub trait Printable {
    fn print(&self);
}

pub struct Book {
    pub name: String,
    pub id: u32,
}

impl Printable for Book {
    fn print(&self) {
        println!(" Printing book with id: {} and name {}", self.id, self.name);
    }
}
