use std::io;
// Note:
// The visibility of an `impl` block's methods is independent of the visibility of the module containing
// that `impl` block.
//
// There's a distinction between module visilibility and type-associated items.

// Building trees from preorder input
mod construction;

// Sideways, indented tree printing
mod display;

// Recursive and iterative travesals
mod traversal;

use super::ui::{clear_screen, print_page_title, read_choice, wait_for_enter};
// In the current file, we will write the
// `BinaryTree<T>` and `Node<T>` definitions.

pub use construction::ParseError;

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

pub fn run() {
    let binary_tree = loop {
        clear_screen();
        print_page_title("Constructing a binary tree now");
        print!("Please enter a binary tree's preorder traversal: ");

        let mut one_line = String::new();
        io::stdin().read_line(&mut one_line).unwrap();

        let res = BinaryTree::from_preorder_str(&one_line, ' ');
        if let Ok(res) = res {
            break res;
        } else {
            println!("Please enter a valid preorder traversal!");
        }

        wait_for_enter();
    };

    loop {
        clear_screen();
        print_page_title("Choose action for the tree");

        println!("  1. Print its sideways diagram");
        println!("  2. Print its traversal");
        println!("  0. Return to the main menu");
        println!();

        match read_choice() {
            0 => break,
            1 => println!(
                "The sideway diagram is\n{}",
                binary_tree.to_sideways_string()
            ),
            2 => traversal::run(&binary_tree),
            _ => println!("Invalid choice — please enter 0 or 1."),
        }

        wait_for_enter();
    }
}
