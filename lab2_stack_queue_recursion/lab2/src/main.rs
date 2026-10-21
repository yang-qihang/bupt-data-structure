use lab2::{
    base_conversion, bracket_matching, parking_lot,
    ui::{clear_screen, print_page_title, wait_for_enter, read_choice},
};

fn main() {
    clear_screen();
    print_header();
    wait_for_enter();

    loop {
        clear_screen();
        print_menu();
        
        match read_choice() {
            1 => {
                clear_screen();
                base_conversion::run();
            } 
            2 => {
                clear_screen();
                bracket_matching::run();
            }
            3 => {
                clear_screen();
                parking_lot::run();
            }
            0 => {
                println!();
                println!("Goodbye!");
                break;
            }
            _ => {
                println!();
                println!("Invalid choice — please enter a number from 0 to 3.");
            }
        }
        
        // Users can take a pause before the current output
        // is all cleaned, especially if users want to exit
        // or users enter something invalid!
        wait_for_enter();
    }
        
}

// The following are three extra helper utilities with
// reading user's input.
// There are not really generic reading functions
// so they are not included in the ui.rs.

fn print_header() {
    println!("Data Structures Lab");
    println!("Lab 2: Stack, Queue and Recursion");
    println!();
    println!("Name:       Yang Qihang");
    println!("Student ID: 2025211681");
    println!("Class:      2025211316");
    println!("Language:   Rust");
    println!();
    println!("Includes:");
    println!("  • Base Conversion");
    println!("  • Bracket Matching");
    println!("  • Parking Lot");
}

fn print_menu() {
    print_page_title("Main Menu");
    println!("  1. Base Conversion");
    println!("  2. Bracket Matching");
    println!("  3. Parking Lot");
    println!();
    println!("  0. Exit");
    println!();
}
