//Import Necessary Modules
use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
/*

io is used for input/output.

Command is used to run the clear command.

sleep and Duration are used to pause the program like usleep.

*/
fn main() {
    let mut input = String::new();
    println!("Please Set time (HH:MM:SS): ");

    // Flush stdout to make sure the prompt is shown
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split(':').collect();

    println!("{}", parts[0]);
    println!("{}", parts[1]);
    println!("{}", parts[2]);
}
