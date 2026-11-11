use std::fmt::{self, Write};

use super::{BinaryTree, Node};

impl<T: fmt::Display> BinaryTree<T> {
    // Public interface

    /// This function provides a light wrapper around `write_sideways`
    /// so the generated result is a String
    pub fn to_sideways_string(&self) -> String {
        let mut output = String::new();

        self.write_sideways(&mut output)
            .expect("Writing to a String cannot fail");

        output
    }

    pub fn write_sideways<W: Write>(&self, output: &mut W) -> fmt::Result {
        Self::write_sideways_subtree(self.root.as_deref(), 0, output)
    }

    // Private recursive helper
    fn write_sideways_subtree<W: Write>(
        node: Option<&Node<T>>,
        depth: usize,
        output: &mut W,
    ) -> fmt::Result {

        if let Some(node) = node {
            // Its rchild first
            Self::write_sideways_subtree(node.right.as_deref(), depth + 1, output)?;
            // The current node
            writeln!(output, "{}{}", " ".repeat(depth * 3), node.value)?;
            // Its left child
            Self::write_sideways_subtree(node.left.as_deref(), depth + 1, output)?;
        }

        Ok(())
    }
}
