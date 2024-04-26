use chrono::{DateTime, Local};

pub fn connect() {
    let now: DateTime<Local> = Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
}
