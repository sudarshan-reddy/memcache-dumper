pub mod lib;
use clap::{App, Arg};
use lib::Dumper;
use std::fs::File;
use std::io;
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
                .required(false),
        ])
        .get_matches();
    let key = matches
        .value_of("key")
        .expect("first argument <key> invalid");

    let value = matches
        .value_of("value")
        .expect("second argument <value> invalid");

    let is_dump = matches.is_present("d");
    let is_load = matches.is_present("l");
    if is_dump && is_load {
        panic!("both dump and load cant be set");
    }

    let is_value_from_file = matches.is_present("f");

    let mut d = Dumper::new("memcache://127.0.0.1:11211?timeout=10&tcp_nodelay=true")
        .expect("dumper init failed");

    if is_dump {
        if is_value_from_file {
            let f = File::create(value).expect("create failed");
            d.dump_key_to_file(key, f).expect("dump to file failed");
            return;
        }
        d.dump_key_to_file(key, io::stdout())
            .expect("dump to stdout failed");
        return;
    }

    if is_load {
        if !is_value_from_file {
            panic!("not implemented yet")
        }
        let f = File::open("test.file").expect("open failed");
        d.load_key_from_file("test", f).expect("load failed");
    }
}
