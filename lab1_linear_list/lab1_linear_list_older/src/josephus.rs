use std::io::{self, Write};
// The josephus problem

// the linked list implementation
pub mod linked;

// the sequential implementation
pub mod sequential;

fn read_usize(prompt: &str) -> usize {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<usize>() {
                Ok(value) => return value,
                Err(_) => println!("Please enter a valid non-negative integer."),
            },

            Err(error) => {
                eprintln!("Failed to read input: {error}");
            }
        }
    }
}
