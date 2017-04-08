use std::thread;

fn main() {

    for i in 1..20{
        thread::spawn(move || {
            println!("Hello");
        });
    }

    thread::sleep_ms(10);
}
