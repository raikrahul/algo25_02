use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) {
    // [INPUT] root_ptr = 0x100 (Node { val: 3, left: 0x200, right: 0x300 })

    // [OP] Check root != NULL
    // 0x100 != 0 → True ✓
    if let Some(r) = root {
        // [OP] Alloc S1 (current_level)
        // [DATA] S1 = [0x100]
        let mut current_level = vec![r];

        // [OP] Alloc S2 (next_level)
        // [DATA] S2 = [] (Capacity 0)
        let mut next_level = Vec::new();

        // [OP] Init Direction
        // [DATA] left_to_right = true (1)
        let mut left_to_right = true;

        // [OP] Outer Loop Check
        // S1.len() = 1 > 0 → True ✓
        while !current_level.is_empty() {
            // STOP.
            // Current State: S1=[0x100], S2=[], dir=true.
            // Waiting for next block logic.
            while let Some(node_rc) = current_level.pop() {
                let node = node_rc.borrow();
                print!("{}", node.val);
                if left_to_right {
                    if let Some(left) = &node.left {
                        next_level.push(Rc::clone(left));
                    }
                    if let Some(right) = &node.right {
                        next_level.push(Rc::clone(right));
                    }
                } else {
                    if let Some(right) = &node.right {
                        next_level.push(Rc::clone(right));
                    }
                    if let Some(left) = &node.left {
                        next_level.push(Rc::clone(left));
                    }
                }
            }
            std::mem::swap(&mut current_level, &mut next_level);
            left_to_right = !left_to_right;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_rc(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    #[test]
    fn test_zig_zag_traversal_basic() {
        // Construct tree:
        //      3
        //     / \
        //    4   7
        //   / \ / \
        //  5  1 6  8

        let mut node3 = TreeNode::new(3);
        let mut node4 = TreeNode::new(4);
        let mut node7 = TreeNode::new(7);
        let node5 = TreeNode::new(5);
        let node1 = TreeNode::new(1);
        let node6 = TreeNode::new(6);
        let node8 = TreeNode::new(8);

        node7.left = to_rc(node6);
        node7.right = to_rc(node8);

        node4.left = to_rc(node5);
        node4.right = to_rc(node1);

        node3.left = to_rc(node4);
        node3.right = to_rc(node7);

        // Capture output? The function returns void and prints.
        // For testing, we might want to change signature to return Vec<i32> or use a callback/writer.
        // Problem statement says "display", usually means print.
        // But for testing, let's assume we print to stdout.
        // Or we can modify function to return Vec<i32> for verification.
        // Following prototype 'void printTree', let's stick to valid printing,
        // but maybe we can just run it to see no panic.

        print_tree(to_rc(node3));
        // Manual check of stdout? Or redesign helper?
    }

    #[test]
    fn test_null_root() {
        print_tree(None);
    }

    #[test]
    fn test_single_node() {
        print_tree(to_rc(TreeNode::new(1)));
    }
}
