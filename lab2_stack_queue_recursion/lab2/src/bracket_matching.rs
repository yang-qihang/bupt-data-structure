// Since the second problem (bracket matching) doesn't dictate the use
// a particular implementation of a stack, I use linked list here mainly
// to refresh the linked list's implementation a bit.
//
// A singly linked list here would be good enough.

// To make the solution modular, the data structures as underpinnings,
// are put in the following module.
use super::data_structures::linked_stack::LinkedList;
use crate::ui::{clear_screen, print_page_title, read_choice, read_line, wait_for_enter};

struct BracketSequence {
    brackets: LinkedList<Bracket>,
}

enum Bracket {
    RoundOpen,   // (
    RoundClose,  // )
    SquareOpen,  // [
    SquareClose, // ]
}

#[derive(Debug, PartialEq)]
enum MatchResult {
    Matched,
    Mismatched,
}

impl BracketSequence {
    pub fn parse(input: &str) -> Option<Self> {
        let brackets = input
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .map(Bracket::parse)
            .collect::<Option<Vec<_>>>()?;

        Some(BracketSequence {
            brackets: LinkedList::from(brackets),
        })
    }

    pub fn check_matching(&self) -> MatchResult {
        let mut checking_stack = LinkedList::new();

        // Although Iterator::next() returns Option<Self::Item>,
        // Rust's for loop automatically handles the Option returned by next().
        for current_bracket in &self.brackets {
            match current_bracket {
                Bracket::RoundOpen => checking_stack.push(Bracket::RoundOpen),
                Bracket::SquareOpen => checking_stack.push(Bracket::SquareOpen),
                Bracket::RoundClose => {
                    let Some(Bracket::RoundOpen) = checking_stack.pop() else {
                        return MatchResult::Mismatched;
                    };
                }
                Bracket::SquareClose => {
                    let Some(Bracket::SquareOpen) = checking_stack.pop() else {
                        return MatchResult::Mismatched;
                    };
                }
            }
        }

        if checking_stack.is_empty() {
            MatchResult::Matched
        } else {
            MatchResult::Mismatched
        }
    }
}

impl Bracket {
    fn parse(ch: char) -> Option<Self> {
        match ch {
            '(' | '（' => Some(Self::RoundOpen),
            ')' | '）' => Some(Self::RoundClose),
            '[' | '［' => Some(Self::SquareOpen),
            ']' | '］' => Some(Self::SquareClose),
            _ => None,
        }
    }
}

pub fn run() {
    loop {
        clear_screen();
        print_page_title("Bracket Matching");

        println!("  1. Check a bracket sequence");
        println!("  0. Return to the main menu");
        println!();

        match read_choice() {
            1 => {
                let input = read_line("Please enter a bracket sequence: ");
                println!();

                match BracketSequence::parse(&input) {
                    Some(sequence) => match sequence.check_matching() {
                        MatchResult::Matched => println!("  The brackets are matched."),
                        MatchResult::Mismatched => println!("  The brackets are NOT matched."),
                    },
                    None => println!("  Invalid input — only (), [] and whitespace are allowed."),
                }
            }
            0 => return,
            _ => println!("Invalid choice — please enter 0 or 1."),
        }

        wait_for_enter();
    }
}

#[cfg(test)]
mod test {
    use crate::bracket_matching::MatchResult;

    use super::{BracketSequence, MatchResult::*};

    enum WhetherShouldBreakDuringParse {
        Should(&'static str),
        ShouldNot(&'static str, MatchResult),
    }

    use WhetherShouldBreakDuringParse::*;

    #[test]
    fn some_test() {
        let test_seq_answer_pair = vec![
            Should("[(!!)]"),
            ShouldNot("（[ ]（））", Matched),
            ShouldNot("[（ ））", Mismatched),
            ShouldNot("", Matched),
            ShouldNot("(([]", Mismatched),
            ShouldNot("]", Mismatched),
        ];

        for current_pair in test_seq_answer_pair {
            match current_pair {
                Should(str) => assert!(BracketSequence::parse(str).is_none()),
                ShouldNot(str, res) => {
                    assert_eq!(BracketSequence::parse(str).unwrap().check_matching(), res)
                }
            }
        }
    }
}
