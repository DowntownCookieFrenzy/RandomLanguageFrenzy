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
