use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;
use std::io::Write;

fn main() {
    println!("How many sides shall your dice have?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut input: i32 = input.trim().parse().expect("not a number");

    let random_number = rand::thread_rng().gen_range(1..=input);

    print!("Rolling dice");

    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
    print!(".");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
    print!(".");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
    print!(".");
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));

    println!("\nYour number is {}", random_number);
}
