use super::BinaryTree;
use super::Node;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
// This enum covers the possibility of malformed input.
pub enum ParseError {
    UnexpectedEnd,
    TrailingInput,
}

// The standard Erorr trait requires Debug + Display.
// (std::error::Error)

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEnd => {
                write!(f, "unexpected end of preorder input")
            }
            Self::TrailingInput => {
                write!(f, "unexpected trailing characters")
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl BinaryTree<char> {
    pub fn from_preorder_str(input: &str, null_marker: char) -> Result<Self, ParseError> {
        let mut char_iter = input.chars();
        let parsed_root_node = Self::parse_subtree(&mut char_iter, null_marker)?;

        (char_iter
            .filter(|c| !c.is_whitespace())
            .next()
            .is_none())
            .then_some(BinaryTree {
                root: parsed_root_node,
            })
            .ok_or(ParseError::TrailingInput)
    }
    // private helper
    fn parse_subtree(
        sequence: &mut impl Iterator<Item = char>,
        null_marker: char,
    ) -> Result<Option<Box<Node<char>>>, ParseError> {
        match sequence.next().ok_or(ParseError::UnexpectedEnd)? {
            c if c == null_marker => Ok(None),

            value => {
                let left = Self::parse_subtree(sequence, null_marker)?;
                let right = Self::parse_subtree(sequence, null_marker)?;

                Ok(Some(Box::new(Node { value, left, right })))
            }
        }
    }
}
