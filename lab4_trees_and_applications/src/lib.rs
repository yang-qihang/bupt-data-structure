pub mod binary_tree;
pub mod ui;

// we re-export the BinaryTree datatype
// as well as the ParseError type.
// 
// This helps to make both types accessible
// directly from the crate root.

pub use binary_tree::{BinaryTree, ParseError};