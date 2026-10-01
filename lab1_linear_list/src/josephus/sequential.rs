use super::read_usize;
use std::io::{self, Write};

pub struct Person {
    id: usize,
    password: usize,
}

pub struct JosephusRing {
    people: Vec<Person>,
}

impl JosephusRing {
    pub fn create_from_vec_of_passwd(passwd: Vec<usize>) -> JosephusRing {
        let people = passwd
            .into_iter()
            .enumerate()
            .map(|(idx, passwd)| Person {
                id: idx + 1,
                password: passwd,
            })
            .collect::<Vec<_>>();
        JosephusRing { people }
    }
    pub fn elimination_order(&mut self, initial_m: usize) -> Vec<usize> {
        // Note:
        // This function, elimination_order takes a parameter that has type JosephusRing,
        // and will DESTROY it while simulating to work out the order of elimination.
        let mut order: Vec<usize> = Vec::new();
        let mut current_m = initial_m;
        let mut current_pos = 0;

        while !self.people.is_empty() {
            let current_len = self.people.len();
            // current_pos_to_remove denotes the position of the person that is now to be removed.
            let current_pos_to_remove = if current_pos == 0 && current_m.is_multiple_of(current_len)
            {
                current_len - 1
            } else {
                (current_pos + current_m % current_len - 1) % current_len
            };
            let to_remove = self.people.remove(current_pos_to_remove);
            order.push(to_remove.id);
            current_m = to_remove.password;

            // the position from which we start the next round may not be current_pos_to_remove
            // in some edge case, it might be equal to 0, which is handled below.
            current_pos = if current_pos_to_remove == current_len - 1 {
                0
            } else {
                current_pos_to_remove
            }
        }
        order
    }
}

pub fn run() {
    loop {
        println!();
        println!("============= JOSEPHUS RING: SEQUENTIAL LIST =============");
        println!("  1. Run Josephus simulation");
        println!("  2. Run sample from the assignment");
        println!("  0. Return to main menu");
        println!("===========================================================");

        match read_usize("Please enter your choice: ") {
            1 => run_custom_simulation(),

            2 => run_sample(),

            0 => break,

            _ => {
                println!("Invalid choice. Please enter 0, 1, or 2.");
            }
        }
    }
}

fn run_custom_simulation() {
    println!();
    println!("Enter the passwords of all people.");
    println!("Example: 3 1 7 2 4 8 4");

    let passwords = read_positive_usize_vec("Passwords: ");

    let initial_m = read_positive_usize("Initial m: ");

    let person_count = passwords.len();

    let mut ring = JosephusRing::create_from_vec_of_passwd(passwords);

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

    let mut ring = JosephusRing::create_from_vec_of_passwd(passwords);

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
        let value = read_usize(prompt);

        if value > 0 {
            return value;
        }

        println!("Please enter a positive integer.");
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

            Ok(values) if values.iter().any(|&value| value == 0) => {
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
        let mut test_list = JosephusRing::create_from_vec_of_passwd(test_passwd);
        let order = test_list.elimination_order(initial_m);
        assert_eq!(order, vec![6, 1, 4, 7, 2, 3, 5]);
    }
}
