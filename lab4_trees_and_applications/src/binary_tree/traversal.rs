use super::BinaryTree;
use super::Node;
use crate::ui::clear_screen;
use crate::ui::print_page_title;
use crate::ui::wait_for_enter;
use std::io;
use std::io::Write;

// ============================================================
// Recursive traversals
// ============================================================

impl<T> BinaryTree<T> {
    pub fn preorder_recursive(&self, mut visit: impl FnMut(&T)) {
        Self::visit_preorder(self.root.as_deref(), &mut visit);
    }

    pub fn inorder_recursive(&self, mut visit: impl FnMut(&T)) {
        Self::visit_inorder(self.root.as_deref(), &mut visit);
    }

    pub fn postorder_recursive(&self, mut visit: impl FnMut(&T)) {
        Self::visit_postorder(self.root.as_deref(), &mut visit);
    }

    // Private recursive helpers
    fn visit_preorder(current: Option<&Node<T>>, visit: &mut impl FnMut(&T)) {
        if current.is_none() {
            return;
        }
        let current = current.unwrap();

        visit(&current.value);
        // reborrow
        Self::visit_preorder(current.left.as_deref(), visit);

        // reborrow
        Self::visit_preorder(current.right.as_deref(), visit);
    }

    fn visit_inorder(current: Option<&Node<T>>, visit: &mut impl FnMut(&T)) {
        if current.is_none() {
            return;
        }
        let current = current.unwrap();
        // reborrow!
        Self::visit_inorder(current.left.as_deref(), visit);
        visit(&current.value);
        // reborrow!
        Self::visit_inorder(current.right.as_deref(), visit);
    }

    fn visit_postorder(current: Option<&Node<T>>, visit: &mut impl FnMut(&T)) {
        if current.is_none() {
            return;
        }
        let current = current.unwrap();
        // reborrow!
        Self::visit_postorder(current.left.as_deref(), visit);
        // reborrow!
        Self::visit_postorder(current.right.as_deref(), visit);
        visit(&current.value)
    }
}

// ============================================================
// Iterative traversals
// ============================================================

impl<T> BinaryTree<T> {
    pub fn preorder_iterative(&self, mut visit: impl FnMut(&T)) {
        let mut to_visit = Vec::<Option<&Node<T>>>::new();

        let mut current_node = self.root.as_deref();

        while current_node.is_some() || !to_visit.is_empty() {
            if current_node.is_none() {
                current_node = to_visit
                    .pop()
                    .expect("By logic, here the stack cannot be empty!");
            } else {
                visit(&current_node.unwrap().value);
                to_visit.push(current_node.unwrap().right.as_deref());
                current_node = current_node.unwrap().left.as_deref();
            }
        }
    }

    pub fn inorder_iterative(&self, mut visit: impl FnMut(&T)) {
        // we need to temporarily store nodes that need:
        // to visit themselves
        // to visit their right children

        let mut to_finish = Vec::<&Node<T>>::new();

        let mut current_node = self.root.as_deref();

        while current_node.is_some() || !to_finish.is_empty() {
            if current_node.is_none() {
                // there's a bit of logic reasoning here
                // if current_node is none, but the condition for the while loop is true
                // there must be some elements in the stack still.
                current_node = Some(to_finish.pop().unwrap());
                visit(&current_node.unwrap().value);
                current_node = current_node.unwrap().right.as_deref();
            } else {
                to_finish.push(current_node.unwrap());
                current_node = current_node.unwrap().left.as_deref();
            }
        }
    }

    pub fn postorder_iterative(&self, mut visit: impl FnMut(&T)) {
        // This is a bit different from before
        // We need to revisit each node in this stack twice
        //   For the 1st time: its rchild
        //   For the 2nd time: itself!!
        //
        // We need some sort of flag to differentiate the two states
        enum RemainState {
            ItsRightChild,
            Itself,
        }
        struct NodeToFinish<'a, T>(&'a Node<T>, RemainState);

        let mut to_finish = Vec::<NodeToFinish<T>>::new();

        let mut current_node = self.root.as_deref();

        while current_node.is_some() || !to_finish.is_empty() {
            if current_node.is_none() {
                let mut to_finish_node = to_finish.pop().unwrap();
                match to_finish_node.1 {
                    RemainState::ItsRightChild => {
                        to_finish_node.1 = RemainState::Itself;
                        current_node = to_finish_node.0.right.as_deref();
                        to_finish.push(to_finish_node);
                    }
                    RemainState::Itself => {
                        visit(&to_finish_node.0.value);
                    }
                }
            } else {
                to_finish.push(NodeToFinish(
                    current_node.unwrap(),
                    RemainState::ItsRightChild,
                ));
                current_node = current_node.unwrap().left.as_deref();
            }
        }
    }
}

pub fn run(tree: &BinaryTree<char>) {
    #[derive(Debug, Clone, Copy)]
    enum TraversalOrder {
        Preorder,
        Inorder,
        Postorder,
    }

    #[derive(Debug, Clone, Copy)]
    enum TraversalMode {
        Recursive,
        Iterative,
    }
    use TraversalMode::*;
    use TraversalOrder::*;

    loop {
        let order = loop {
            clear_screen();
            print_page_title("Traversing the tree now");
            println!("In which order do you want to traverse the tree?");
            println!("  1. Preorder");
            println!("  2. Inorder");
            println!("  3. Postorder");
            println!("Press 0 to return to the tree's main menu.");
            println!();

            print!("Please enter your choice(0/1/2/3): ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim().parse::<i32>().unwrap();

            match choice {
                0 => return,
                1 => break Preorder,
                2 => break Inorder,
                3 => break Postorder,
                _ => println!("Please enter 1, 2, or 0!"),
            }

            wait_for_enter();
        };

        let mode = loop {
            clear_screen();
            println!("In which mode do you want to traverse the tree?");
            println!("  0. Recursive");
            println!("  1. Iterative");
            println!();

            print!("Please enter your choice(0/1): ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim().parse::<i32>().unwrap();

            match choice {
                0 => break Recursive,
                1 => break Iterative,
                _ => println!("Please enter 1, 2, or 0!"),
            }

            wait_for_enter();
        };

        let mut visit = |value: &char| print!("{value}");

        match (order, mode) {
            (Preorder, Recursive) => tree.preorder_recursive(&mut visit),
            (Preorder, Iterative) => tree.preorder_iterative(&mut visit),

            (Inorder, Recursive) => tree.inorder_recursive(&mut visit),
            (Inorder, Iterative) => tree.inorder_iterative(&mut visit),

            (Postorder, Recursive) => tree.postorder_recursive(&mut visit),
            (Postorder, Iterative) => tree.postorder_iterative(&mut visit),
        }
        
        wait_for_enter();
    }
}
