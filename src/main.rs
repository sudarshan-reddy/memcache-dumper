pub mod lib;
use lib::Dumper;
use std::fs::File;
fn main() {
    let mut d = Dumper::new("memcache://127.0.0.1:11211?timeout=10&tcp_nodelay=true")
        .expect("dumper init failed");
    let f = File::open("test.file").expect("open failed");
    d.load_key_from_file("test", f).expect("load failed");
}
