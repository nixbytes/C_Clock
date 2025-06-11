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

    //    Reads a line of input.
    //    Splits it by : into a vector of strings.
    //    Tries to parse them into integers. If parsing fails, uses 0 (basic error fallback).

    let parts: Vec<&str> = input.trim().split(':').collect();
    if parts.len() != 3 {
        eprintln!("ERRORS! Invalid input format");
    }

    let mut hours: u32 = parts[0].parse().unwrap_or(0);
    let mut mins: u32 = parts[1].parse().unwrap_or(0);
    let mut secs: u32 = parts[2].parse().unwrap_or(0);

    // validate 12-hour format and valid minute/second range
    if hours < 1 || hours > 12 || mins > 59 || secs > 59 {
        eprintln!("ERROR! Invalid input.");
        return;
    }
    
    loop {
        secs +=1;
        if secs > 59{
            secs = 0;
            mins += 1;
        }

        if mins > 59{
            mins = 0;
            hours += 1;
        }

        if hours > 12{
            hours = 1;  
        }

        println!("\n Clock :");
        println!(" {:02}:{:02}:{:02}", hours, mins, secs);

        sleep(Duration::from_millis(100));

        // Clear the screen (Unix-like systems)
        Command::new("clear").status().unwrap();
    }
    
}
