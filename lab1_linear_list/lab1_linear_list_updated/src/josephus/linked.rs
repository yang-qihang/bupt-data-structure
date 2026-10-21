use super::read_usize;
use crate::ui::{clear_screen, print_page_title, wait_for_enter};
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

#[derive(Debug)]
pub struct PersonNode {
    id: usize,
    password: usize,
    next: Option<Rc<RefCell<PersonNode>>>,
}

#[derive(Debug)]
pub struct PersonLinkedList {
    handle: Option<Rc<RefCell<PersonNode>>>,
}

impl PersonLinkedList {
    pub fn create_from_vec_of_passwd(passwd: Vec<usize>) -> PersonLinkedList {
        // edge case: if there are no passwords, return an empty list.
        if passwd.is_empty() {
            return PersonLinkedList { handle: None };
        }

        let mut to_add = passwd.into_iter().enumerate();
        let (_, first_person_passwd) = to_add.next().unwrap();
        let first_person = Rc::new(RefCell::new(PersonNode {
            id: 1,
            password: first_person_passwd,
            next: None,
        }));
        // The following was a failed attempt to use traditional references as the handle
        // while traversing, which doesn't work because the result from borrow_mut is a temporary
        // value, which will very soon be destroyed.
        //
        // let mut ptr_to_current_next = &mut first_person.next;
        // for (idx, passwd) in to_add {
        //     let temp_node = PersonNode {
        //         id: idx + 1,
        //         password: passwd,
        //         next: None,
        //     };
        //     *ptr_to_current_next = Some(Rc::new(RefCell::new(temp_node)));
        //     ptr_to_current_next = &mut ptr_to_current_next.as_mut().unwrap().borrow_mut().next;
        // }

        // Beware of what current means:
        // It doesn't mean the current node's next field
        // It instead means just the current node (or the "last" node)
        let mut current = first_person.clone();
        for (idx, passwd) in to_add {
            let temp_node = Rc::new(RefCell::new(PersonNode {
                id: idx + 1,
                password: passwd,
                next: None,
            }));
            // Improvement: the temp_node passed into Some() here should be cloned, then
            // current would be naturally able to move the temp_node.
            //
            // Improvement: don't do current.deref().borrow_mut() since Deref Coercion will do that
            // for you implicitly.
            current.borrow_mut().next = Some(temp_node.clone());
            current = temp_node;
        }
        // Improvement: don't do current.deref().borrow() since Deref Coercion will do that
        // for you implicitly.
        // current.deref().borrow_mut().next = Some(first_person.clone());
        current.borrow_mut().next = Some(first_person.clone());
        PersonLinkedList {
            handle: Some(first_person.clone()),
        }
    }
    pub fn elimination_order(&mut self, initial_m: usize) -> Vec<usize> {
        // Part 0:
        // invariant: self has contain at least some elements;
        // also, initial_m has to be a positive integer
        if self.handle.is_none() {
            return Vec::new();
        } else if initial_m == 0 {
            panic!("Please provide a positive integer to be the initial m!")
        }

        // Part 1:
        // find the last node (which is the node that's before the first node)
        // since the list wraps around!
        let mut current: Rc<RefCell<PersonNode>> = self.handle.as_ref().unwrap().clone();
        let to_cmp_first: Rc<RefCell<PersonNode>> = current.clone();

        while !Rc::ptr_eq(current.borrow().next.as_ref().unwrap(), &to_cmp_first) {
            // continue the loop if the current node's next one isn't the first node
            let temp = current.borrow().next.as_ref().unwrap().clone();
            current = temp;
        }

        let mut current_prev_node = current;
        let mut current_limit = initial_m;
        let mut current_count = 0;
        let mut order: Vec<usize> = Vec::new(); // Stores the order in which people get eliminated.

        //let mut debug_cnt = 0;

        while !Rc::ptr_eq(
            // as long as the current node doesn't point to the current node
            // i.e. the length of the loop is greater than 1
            current_prev_node.borrow().next.as_ref().unwrap(),
            &current_prev_node,
        ) {
            if current_count == current_limit - 1 {
                // temporarily store the person that's going to be removed
                let tmp_node_to_remove = current_prev_node.borrow().next.as_ref().unwrap().clone();
                order.push(tmp_node_to_remove.borrow().id);
                current_limit = tmp_node_to_remove.borrow().password;
                current_count = 0;
                current_prev_node.borrow_mut().next = tmp_node_to_remove.borrow().next.clone();
            } else {
                current_count += 1;
                let temp = current_prev_node.borrow().next.as_ref().unwrap().clone();
                current_prev_node = temp;
            }
        }
        order.push(current_prev_node.borrow().id);
        // break the circle
        current_prev_node.borrow_mut().next = None;
        order
    }
}

pub fn run() {
    loop {
        clear_screen();
        print_page_title("Josephus Ring — Circular Linked List");
        println!("  1. Run Josephus simulation");
        println!("  2. Run sample from the assignment");
        println!("  0. Return to main menu");
        println!();

        match read_usize("Please enter your choice: ") {
            1 => run_custom_simulation(),

            2 => run_sample(),

            0 => break,

            _ => {
                println!("Invalid choice. Please enter 0, 1, or 2.");
            }
        }

        wait_for_enter();
    }
}

fn run_custom_simulation() {
    println!();
    println!("Enter the passwords of all people.");
    println!("Example: 3 1 7 2 4 8 4");

    let passwords = read_positive_usize_vec("Passwords: ");

    let initial_m = read_positive_usize("Initial m: ");

    let person_count = passwords.len();

    let mut ring = PersonLinkedList::create_from_vec_of_passwd(passwords);

    let order = ring.elimination_order(initial_m);

    println!();
    println!("Number of people: {person_count}");
    println!("Initial m: {initial_m}");

    print_elimination_order(&order);
}

fn run_sample() {
    let passwords = vec![3, 1, 7, 2, 4, 8, 4];
    let initial_m = 20;

    println!();
    println!("Running the sample from the assignment:");
    println!("Passwords: 3 1 7 2 4 8 4");
    println!("Initial m: 20");

    let mut ring = PersonLinkedList::create_from_vec_of_passwd(passwords);

    let order = ring.elimination_order(initial_m);

    print_elimination_order(&order);
}

fn print_elimination_order(order: &[usize]) {
    println!();
    print!("Elimination order: ");

    for (index, id) in order.iter().enumerate() {
        if index != 0 {
            print!(" -> ");
        }

        print!("{id}");
    }

    println!();
}

fn read_positive_usize(prompt: &str) -> usize {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<usize>() {
                Ok(value) if value > 0 => return value,

                Ok(_) => {
                    println!("Please enter a positive integer.");
                }

                Err(_) => {
                    println!("Please enter a valid positive integer.");
                }
            },

            Err(error) => {
                eprintln!("Failed to read input: {error}");
            }
        }
    }
}

fn read_positive_usize_vec(prompt: &str) -> Vec<usize> {
    loop {
        print!("{prompt}");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read input: {error}");
            continue;
        }

        let parsed = input
            .split_whitespace()
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>();

        match parsed {
            Ok(values) if values.is_empty() => {
                println!("Please enter at least one password.");
            }

            Ok(values) if values.contains(&0) => {
                println!("Every password must be a positive integer.");
            }

            Ok(values) => return values,

            Err(_) => {
                println!("Invalid input. Enter positive integers separated by spaces.");
            }
        }
    }
}
#[cfg(test)]
mod test {
    // bring everything in its parent scope into scope
    use super::*;
    #[test]
    fn test_sample_from_school_docs() {
        // password: 3, 1, 7, 2, 4, 8, 4
        let test_passwd: Vec<usize> = vec![3, 1, 7, 2, 4, 8, 4];
        let initial_m: usize = 20;
        let mut test_list = PersonLinkedList::create_from_vec_of_passwd(test_passwd);
        let order = test_list.elimination_order(initial_m);
        assert_eq!(order, vec![6, 1, 4, 7, 2, 3, 5]);
    }
}
