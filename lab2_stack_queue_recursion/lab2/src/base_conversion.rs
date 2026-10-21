use crate::ui::{clear_screen, print_page_title, read_choice, read_u64, wait_for_enter};

// To make the solution more modular, the underlying data strucutures of this lab
// are implemented as sub-modules separately.
use super::data_structures::array_stack::Stack;

pub fn run() {
    loop {
        clear_screen();
        print_page_title("Base Conversion");

        println!("  1. Perform a conversion");
        println!("  0. Return to the main menu");
        println!();

        match read_choice() {
            1 => {
                let number = read_u64("Please enter a non-negative decimal integer: ");
                println!();
                println!("  {number} (decimal) = {} (octal)", decimal_to_octal(number));
            }
            0 => return,
            _ => println!("Invalid choice — please enter 0 or 1."),
        }

        wait_for_enter();
    }
}
pub fn decimal_to_octal(number: u64) -> String {
    // Converts a decimal number (represented as a 64-bit unsigned integer in Rust)
    // into a String which is its octal representation

    // edge case: if number == 0, return "0" directly
    // or we can use a loop to express the C's do ... while semantics

    if number == 0 {
        return String::from("0");
    }

    let mut dividend = number;
    let oct_divisor = 8u64;
    let mut digits_stack = Stack::new();

    while dividend != 0 {
        digits_stack.push((dividend % oct_divisor) as u8);
        dividend /= oct_divisor;
    }

    let mut res = String::new();
    while let Some(current_digit) = digits_stack.pop() {
        res.push(char::from(current_digit + b'0'));
    }
    res
}

#[cfg(test)]
mod test {
    use crate::base_conversion::decimal_to_octal;

    #[test]
    fn conversion() {
        // Since the std lib explicitly implements
        // `PartialEq` between `string` and `&str`
        //
        // We can compare a String against a &str
        // == doesn't use Deref coercision
        // So don't conflate the two things!
        assert_eq!(decimal_to_octal(0), "0");
        assert_eq!(decimal_to_octal(1), "1");
        assert_eq!(decimal_to_octal(7), "7");
        assert_eq!(decimal_to_octal(8), "10");
        assert_eq!(decimal_to_octal(1348), "2504");
        assert_eq!(decimal_to_octal(u64::MAX), format!("{:o}", u64::MAX));
    }
}
