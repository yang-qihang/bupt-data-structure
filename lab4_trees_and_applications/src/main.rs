use lab4_trees_and_applications::binary_tree;
use lab4_trees_and_applications::{
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
            0 => {
                println!();
                println!("Goodbye!");
                break;
            }
            1 => {
                clear_screen();
                binary_tree::run();
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
    println!("Lab 4: Trees and Their Applications ");
    println!();
    println!("Name:       Yang Qihang");
    println!("Student ID: 2025211681");
    println!("Class:      2025211316");
    println!("Language:   Rust");
    println!();
    println!("Includes:");
    println!("  • Construction of a binary tree");
    println!("  • All kinds of travesals");
    println!("  • Its sideways tree representation");

}

fn print_menu() {
    print_page_title("Main Menu");
    println!("  1. Construct a binary tree first");
    println!("  0. Exit");
    println!();
    println!("Note:");
    println!("  You need to construct a binary tree\n  before you can perform other actions!");
    println!();
}
