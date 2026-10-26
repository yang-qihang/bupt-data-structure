use lab3::{
    substring_search,
    ui::{clear_screen, print_page_title, read_choice, wait_for_enter},
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
                substring_search::run();
            }
            0 => {
                println!();
                println!("Goodbye!");
                break;
            }
            _ => {
                println!();
                println!("Invalid choice — please enter a number from 0 to 1.");
            }
        }
        wait_for_enter();
    }
}

fn print_header() {
    println!("Data Structures Lab");
    println!("Lab 3: String Pattern Matching");
    println!();
    println!("Name:       Yang Qihang");
    println!("Student ID: 2025211681");
    println!("Class:      2025211316");
    println!("Language:   Rust");
    println!();
    println!("Includes:");
    println!("  • The KMP pattern matching algorithm");
}

fn print_menu() {
    print_page_title("Main Menu");
    println!("  1. The KMP pattern matching algorithm");
    println!("  0. Exit");
    println!();
}
