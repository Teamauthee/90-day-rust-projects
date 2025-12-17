mod greeting;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let message = greeting::greeting(&args[1]);
    println!("{}", message);
}
