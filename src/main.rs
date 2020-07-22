pub mod lib;
use clap::{App, Arg};
use lib::Dumper;
use std::fs::File;
fn main() {
    let matches = App::new("memcache-dumper")
        .version("0.0.1")
        .author("Sudarshan Reddy")
        .about("just a small memcache dump loader")
        .arg("-d 'dump'")
        .arg("-l 'load'")
        .arg("-f 'value to file'")
        .args(&[
            Arg::with_name("key")
                .about("memcache key")
                .index(1)
                .required(true),
            Arg::with_name("value")
                .about("value or file name")
                .index(2)
                .required(true),
        ])
        .get_matches();
    println!("{}", matches.value_of("key").unwrap());
    let mut d = Dumper::new("memcache://127.0.0.1:11211?timeout=10&tcp_nodelay=true")
        .expect("dumper init failed");
    let f = File::open("test.file").expect("open failed");
    d.load_key_from_file("test", f).expect("load failed");

    let f2 = File::create("op.file").expect("create f2 failed");
    d.dump_key_to_file("test", f2).expect("save failed");
}
