use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;
use std::io::{Read};
use std::io::Write;
use std::process::Command;

fn main() {
    println!("How many sides shall your dice have?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("not a number");

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

    println!("Press any key to exit...");
    let _ = std::io::stdin().read(&mut [0u8]);

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "exit"])
            .status()
            .expect("Failed to execute command");
    } else {
        println!("This only works on Windows.");
    }
}
