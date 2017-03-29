use std::env;
use std::io::{stdout, Write};

extern crate curl;
use curl::easy::Easy;

fn main() {
    let url = env::args().nth(1).unwrap();
    println!("Retrieving: {:?}", url);

    let mut easy = Easy::new();
    easy.url(&url).unwrap();

    easy.write_function(|data| Ok(stdout().write(data).unwrap()))
        .unwrap();

    easy.perform().unwrap();
}
