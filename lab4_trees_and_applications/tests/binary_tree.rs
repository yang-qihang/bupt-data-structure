//! Integration tests for the lab's public binary-tree API.
//!
//! Place this file at `tests/binary_tree.rs` in your crate.
//! If the package name differs, update the import below.

use lab4_trees_and_applications::{BinaryTree, ParseError};
use std::fmt;

const SAMPLE: &str = "ABC  DE G  F   ";

fn parse(input: &str) -> BinaryTree<char> {
    BinaryTree::from_preorder_str(input, ' ')
        .expect("the test fixture must be a valid preorder encoding")
}

fn preorder_recursive(tree: &BinaryTree<char>) -> String {
    let mut out = String::new();
    tree.preorder_recursive(|value| out.push(*value));
    out
}

fn inorder_recursive(tree: &BinaryTree<char>) -> String {
    let mut out = String::new();
    tree.inorder_recursive(|value| out.push(*value));
    out
}

fn postorder_recursive(tree: &BinaryTree<char>) -> String {
    let mut out = String::new();
    tree.postorder_recursive(|value| out.push(*value));
    out
}

fn preorder_iterative(tree: &BinaryTree<char>) -> String {
    let mut out = String::new();
    tree.preorder_iterative(|value| out.push(*value));
    out
}

fn inorder_iterative(tree: &BinaryTree<char>) -> String {
    let mut out = String::new();
    tree.inorder_iterative(|value| out.push(*value));
    out
}

fn postorder_iterative(tree: &BinaryTree<char>) -> String {
    let mut out = String::new();
    tree.postorder_iterative(|value| out.push(*value));
    out
}

// ---------- Construction and error handling ----------

#[test]
fn empty_tree_is_a_valid_tree() {
    let parsed = parse(" ");
    let constructed = BinaryTree::<char>::new();
    for tree in [&parsed, &constructed] {
        assert_eq!(preorder_recursive(tree), "");
        assert_eq!(inorder_recursive(tree), "");
        assert_eq!(postorder_recursive(tree), "");
        assert_eq!(tree.to_sideways_string(), "");
    }
}

#[test]
fn incomplete_encodings_are_rejected() {
    for input in ["", "A", "A ", "AB  ", "ABC  DE G  F  "] {
        assert!(
            matches!(
                BinaryTree::from_preorder_str(input, ' '),
                Err(ParseError::UnexpectedEnd)
            ),
            "expected UnexpectedEnd for {input:?}"
        );
    }
}

#[test]
fn trailing_data_is_rejected() {
    for input in ["  ", "A   ", "A  X", "ABC  DE G  F    "] {
        assert!(
            matches!(
                BinaryTree::from_preorder_str(input, ' '),
                Err(ParseError::TrailingInput)
            ),
            "expected TrailingInput for {input:?}"
        );
    }
}

#[test]
fn configurable_null_markers_and_unicode_work() {
    let tree = BinaryTree::from_preorder_str("🌳∅∅", '∅').unwrap();
    assert_eq!(preorder_recursive(&tree), "🌳");
    assert_eq!(inorder_recursive(&tree), "🌳");
    assert_eq!(postorder_recursive(&tree), "🌳");

    // With '#' as the marker, a literal space is a valid node value.
    let tree = BinaryTree::from_preorder_str(" # ##", '#').unwrap();
    assert_eq!(preorder_recursive(&tree), "  ");
}

// ---------- Instructor's example ----------

#[test]
fn sample_recursive_preorder() {
    assert_eq!(preorder_recursive(&parse(SAMPLE)), "ABCDEGF");
}

#[test]
fn sample_recursive_inorder() {
    assert_eq!(inorder_recursive(&parse(SAMPLE)), "CBEGDFA");
}

#[test]
fn sample_recursive_postorder() {
    assert_eq!(postorder_recursive(&parse(SAMPLE)), "CGEFDBA");
}

#[test]
fn sample_iterative_preorder() {
    assert_eq!(preorder_iterative(&parse(SAMPLE)), "ABCDEGF");
}

#[test]
fn sample_iterative_inorder() {
    assert_eq!(inorder_iterative(&parse(SAMPLE)), "CBEGDFA");
}

#[test]
fn sample_iterative_postorder() {
    assert_eq!(postorder_iterative(&parse(SAMPLE)), "CGEFDBA");
}

// ---------- Smallest nonempty tree ----------

#[test]
fn single_node_all_traversals() {
    let tree = parse("X  ");
    assert_eq!(preorder_recursive(&tree), "X");
    assert_eq!(inorder_recursive(&tree), "X");
    assert_eq!(postorder_recursive(&tree), "X");
    assert_eq!(preorder_iterative(&tree), "X");
    assert_eq!(inorder_iterative(&tree), "X");
    assert_eq!(postorder_iterative(&tree), "X");
}

#[test]
fn empty_tree_all_iterative_traversals() {
    let tree = parse(" ");
    assert_eq!(preorder_iterative(&tree), "");
    assert_eq!(inorder_iterative(&tree), "");
    assert_eq!(postorder_iterative(&tree), "");
}

// ---------- Formatting ----------

#[test]
fn sideways_display_single_node() {
    assert_eq!(parse("X  ").to_sideways_string(), "X\n");
}

#[test]
fn sideways_display_simple_fork() {
    //       A
    //      / \
    //     B   C
    let tree = parse("AB  C  ");
    assert_eq!(tree.to_sideways_string(), " C\nA\n B\n");
}

#[test]
fn sideways_display_instructors_sample() {
    // One indentation space per level, matching the submitted display.rs.
    let expected = "A\n   F\n  D\n    G\n   E\n B\n  C\n";
    assert_eq!(parse(SAMPLE).to_sideways_string(), expected);
}

#[test]
fn write_sideways_uses_a_supplied_buffer() {
    let tree = parse("AB  C  ");
    let mut out = String::from("prefix:\n");
    tree.write_sideways(&mut out).unwrap();
    assert_eq!(out, "prefix:\n C\nA\n B\n");
}

struct AlwaysFailWriter;
impl fmt::Write for AlwaysFailWriter {
    fn write_str(&mut self, _s: &str) -> fmt::Result {
        Err(fmt::Error)
    }
}

#[test]
fn formatting_errors_are_propagated() {
    assert!(parse("X  ").write_sideways(&mut AlwaysFailWriter).is_err());
    assert!(parse(" ").write_sideways(&mut AlwaysFailWriter).is_ok());
}

// ---------- Stateful FnMut visitors ----------

#[test]
fn visitors_can_mutate_captured_state() {
    let tree = parse(SAMPLE);
    let mut count = 0;
    let mut seen = Vec::new();
    tree.preorder_recursive(|value| {
        count += 1;
        seen.push(*value);
    });
    assert_eq!(count, 7);
    assert_eq!(seen, vec!['A', 'B', 'C', 'D', 'E', 'G', 'F']);
}

// ---------- Exhaustive cross-check against an independent reference tree ----------
//
// All ordered binary-tree shapes with 0..=5 real nodes are generated.
// The oracle is derived from a separate reference representation, not from
// any traversal method in the library under test.

#[derive(Clone)]
enum Shape {
    Empty,
    Branch(Box<Shape>, Box<Shape>),
}

fn all_shapes(node_count: usize) -> Vec<Shape> {
    if node_count == 0 {
        return vec![Shape::Empty];
    }

    let mut results = Vec::new();
    for left_count in 0..node_count {
        let right_count = node_count - 1 - left_count;
        let left_shapes = all_shapes(left_count);
        let right_shapes = all_shapes(right_count);

        for left in &left_shapes {
            for right in &right_shapes {
                results.push(Shape::Branch(
                    Box::new(left.clone()),
                    Box::new(right.clone()),
                ));
            }
        }
    }
    results
}

enum ReferenceTree {
    Empty,
    Branch(char, Box<ReferenceTree>, Box<ReferenceTree>),
}

fn label_shape(shape: &Shape, next: &mut u8) -> ReferenceTree {
    match shape {
        Shape::Empty => ReferenceTree::Empty,
        Shape::Branch(left, right) => {
            let label = char::from(b'A' + *next);
            *next += 1;
            ReferenceTree::Branch(
                label,
                Box::new(label_shape(left, next)),
                Box::new(label_shape(right, next)),
            )
        }
    }
}

fn serialize_reference(tree: &ReferenceTree, out: &mut String) {
    match tree {
        ReferenceTree::Empty => out.push(' '),
        ReferenceTree::Branch(value, left, right) => {
            out.push(*value);
            serialize_reference(left, out);
            serialize_reference(right, out);
        }
    }
}

fn reference_orders(tree: &ReferenceTree) -> [String; 3] {
    match tree {
        ReferenceTree::Empty => [String::new(), String::new(), String::new()],
        ReferenceTree::Branch(value, left, right) => {
            let [lp, li, lpost] = reference_orders(left);
            let [rp, ri, rpost] = reference_orders(right);
            [
                format!("{value}{lp}{rp}"),
                format!("{li}{value}{ri}"),
                format!("{lpost}{rpost}{value}"),
            ]
        }
    }
}

fn check_all_small_shapes(
    algorithm: &str,
    actual_traversal: fn(&BinaryTree<char>) -> String,
    order_index: usize,
) {
    for size in 0..=5 {
        for shape in all_shapes(size) {
            let mut next_label = 0;
            let reference = label_shape(&shape, &mut next_label);
            let mut encoded = String::new();
            serialize_reference(&reference, &mut encoded);
            let expected = reference_orders(&reference);
            let tree = parse(&encoded);
            let actual = actual_traversal(&tree);

            assert_eq!(
                actual,
                expected[order_index],
                "algorithm={algorithm}; nodes={size}; encoding={encoded:?}"
            );
        }
    }
}

#[test]
fn exhaustive_recursive_preorder() {
    check_all_small_shapes("recursive preorder", preorder_recursive, 0);
}

#[test]
fn exhaustive_recursive_inorder() {
    check_all_small_shapes("recursive inorder", inorder_recursive, 1);
}

#[test]
fn exhaustive_recursive_postorder() {
    check_all_small_shapes("recursive postorder", postorder_recursive, 2);
}

#[test]
fn exhaustive_iterative_preorder() {
    check_all_small_shapes("iterative preorder", preorder_iterative, 0);
}

#[test]
fn exhaustive_iterative_inorder() {
    check_all_small_shapes("iterative inorder", inorder_iterative, 1);
}

#[test]
fn exhaustive_iterative_postorder() {
    check_all_small_shapes("iterative postorder", postorder_iterative, 2);
}
