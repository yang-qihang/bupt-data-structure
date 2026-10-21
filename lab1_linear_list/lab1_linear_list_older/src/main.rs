use std::io::{self, Write};

use lab1_linear_list::{
    city_list_linked, city_list_sequential, josephus_linked, josephus_sequential,
};

fn main() {
    print_header();

    loop {
        print_menu();

        match read_choice() {
            1 => {
                clear_separator();
                println!("Sequential List — City List");
                clear_separator();

                city_list_sequential::run();
            }

            2 => {
                clear_separator();
                println!("Linked List — City List");
                clear_separator();

                city_list_linked::run();
            }

            3 => {
                clear_separator();
                println!("Sequential List — Josephus Ring");
                clear_separator();

                josephus_sequential::run();
            }

            4 => {
                clear_separator();
                println!("Circular Linked List — Josephus Ring");
                clear_separator();

                josephus_linked::run();
            }

            0 => {
                println!();
                println!("Exiting the program.");
                println!("Thank you for using the Linear List Lab program.");
                break;
            }

            _ => {
                println!();
                println!("Invalid choice. Please enter a number from 0 to 4.");
            }
        }

        wait_for_enter();
    }
}

fn print_header() {
    println!(
        r#"
============================================================
                    DATA STRUCTURES LAB
============================================================

Lab 1: Linear List

Name:       Yang Qihang
Student ID: 2025211681
Class:      2025211316

Implementation Language: Rust

This program contains four implementations:

    • City List using a Sequential List
    • City List using a Linked List
    • Josephus Ring using a Sequential List
    • Josephus Ring using a Circular Linked List

============================================================
"#
    );
}

fn print_menu() {
    println!();
    println!("===================== MAIN MENU ===========================");
    println!();
    println!("  1. City List       - Sequential List");
    println!("  2. City List       - Linked List");
    println!("  3. Josephus Ring   - Sequential List");
    println!("  4. Josephus Ring   - Circular Linked List");
    println!();
    println!("  0. Exit");
    println!();
    println!("===========================================================");
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

fn wait_for_enter() {
    println!();
    println!("Press Enter to return to the main menu...");

    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read input: {error}");
    }
}

fn clear_separator() {
    println!();
    println!("===========================================================");
}
