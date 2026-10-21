use std::io::{self, Write};

use lab1_linear_list::{
    city_list_linked, city_list_sequential, josephus_linked, josephus_sequential,
    ui::{clear_screen, print_page_title, wait_for_enter},
};

fn main() {
    clear_screen();
    print_header();

    loop {
        print_menu();

        match read_choice() {
            1 => {
                clear_screen();
                city_list_sequential::run();
            }

            2 => {
                clear_screen();
                city_list_linked::run();
            }

            3 => {
                clear_screen();
                josephus_sequential::run();
            }

            4 => {
                clear_screen();
                josephus_linked::run();
            }

            0 => {
                println!();
                println!("Goodbye!");
                break;
            }

            _ => {
                println!();
                println!("Invalid choice — please enter a number from 0 to 4.");
            }
        }

        wait_for_enter();
        clear_screen();
    }
}

fn print_header() {
    println!("Data Structures Lab");
    println!("Lab 1: Linear List");
    println!();
    println!("Name:       Yang Qihang");
    println!("Student ID: 2025211681");
    println!("Class:      2025211316");
    println!("Language:   Rust");
    println!();
    println!("Includes:");
    println!("  • City List using a Sequential List");
    println!("  • City List using a Linked List");
    println!("  • Josephus Ring using a Sequential List");
    println!("  • Josephus Ring using a Circular Linked List");
}

fn print_menu() {
    print_page_title("Main Menu");
    println!("  1. City List — Sequential List");
    println!("  2. City List — Linked List");
    println!("  3. Josephus Ring — Sequential List");
    println!("  4. Josephus Ring — Circular Linked List");
    println!();
    println!("  0. Exit");
    println!();
}

fn read_choice() -> u32 {
    loop {
        print!("Please enter your choice: ");

        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(choice) => return choice,
                Err(_) => {
                    println!("Please enter a valid integer.");
                }
            },

            Err(error) => {
                eprintln!("Failed to read input: {error}");
            }
        }
    }
}

