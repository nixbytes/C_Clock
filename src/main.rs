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
    if parts.len() != 3{
        eprintln!("ERRORS! Invalid input format");
    }

    let mut hours: u32 = parts[0].parse().unwrap_or(0);
    let mut mins: u32 = parts[1].parse().unwrap_or(0);
    let mut secs: u32 = parts[2].parse().unwrap_or(0);
}
