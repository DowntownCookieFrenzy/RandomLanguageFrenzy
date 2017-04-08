extern crate curl;

use curl::easy::Easy;

// fn get<a> (a: &String){
//    let mut dst: Vec<u8> = Vec::new();
//    let mut easy = Easy::new();
//    easy.url(a.as_str()).unwrap();
//
//    let mut transfer = easy.transfer();
//    transfer.write_function(|data| {
//        dst.extend_from_slice(data);
//        Ok(data.len())
//    }).unwrap();
//    transfer.perform().unwrap();
//    (dst);
// }

fn print(a: u8){
    println!("{}", a);
}

fn modify(a: &mut u8){
    *a = 4u8;
}

fn main() {
    let mut v: u8 = 3;
    print(v);
    modify(&mut v);
    print(v);
}
