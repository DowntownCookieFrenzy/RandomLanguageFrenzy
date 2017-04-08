extern crate curl;

use curl::easy;
use curl::easy::Easy;
use std::io::{stdout,Write};
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("Please specify a url!");
    }

    let mut easy = Easy::new();
    easy.url(args[1].as_str()).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
}
