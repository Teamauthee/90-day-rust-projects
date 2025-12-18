use std::io;
fn main() {
    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read line.");

    let n_fibo = user_choice
        .trim()
        .parse::<u128>()
        .expect("Please type a number");

    let result = fibonacci(n_fibo);

    println!("The result is {result}");
}

fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b: u128 = 1;
    let mut temp: u128;

    for _ in 2..=n {
        temp = a + b;
        a = b;
        b = temp;
    }
    b
}
