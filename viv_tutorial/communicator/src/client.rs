use chrono::{self, Local, Offset};

pub fn connect() {
    // returns DateTime<Local>
    println!("{:?}", Local::now());

    // returns DateTime<Utc>
    // NOTE: Available on crate feature *clock* only.
    println!("{:?}", Utc::now());
}
