extern crate communicator;
extern crate util;

fn main() {
    communicator::client::connect();
    util::start::viv_hashmap();
    let msg: String = util::start::viv_menu();
    println!("\u{2766} {}", msg);
}
