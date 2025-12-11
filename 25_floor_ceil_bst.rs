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

// 01. Function: Find Floor(x).
// 02. Definition: Largest value v in tree such that v <= x.
// 03. Constraint: Time O(H), Space O(1).
// 04. Input: Root node, Target x.
// 05. Output: Option<i32> (Some(val) or None).
pub fn floor(root: Option<Rc<RefCell<TreeNode>>>, x: i32) -> Option<i32> {
    let mut res: Option<i32> = None;
    let mut curr = root;
    while let Some(node) = curr {
        let val = node.borrow().val;
        if val == x {
            return Some(x);
        } else if val > x {
            curr = node.borrow().left.clone();
        } else {
            res = Some(val);
            curr = node.borrow().right.clone();
        }
    }
    res
}

pub fn ceil(root: Option<Rc<RefCell<TreeNode>>>, x: i32) -> Option<i32> {
    let mut res: Option<i32> = None;
    let mut curr = root;
    while let Some(node) = curr {
        let val = node.borrow().val;
        if val == x {
            return Some(x);
        } else if val < x {
            curr = node.borrow().right.clone();
        } else {
            res = Some(val);
            curr = node.borrow().left.clone();
        }
    }
    res
}

// Helper: Insert into BST
fn insert(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if let Some(r) = root {
        let mut node = r.borrow_mut();
        if val < node.val {
            insert(&mut node.left, val);
        } else {
            insert(&mut node.right, val);
        }
    } else {
        *root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build tree from array
    fn build_bst(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;
        for &v in values {
            insert(&mut root, v);
        }
        root
    }

    #[test]
    fn test_floor_ceil_main_example() {
        // Data: [13, 9, 16, 5, 10, 14, 18]
        // Tree:
        //      13
        //     /  \
        //    9    16
        //   /\    / \
        //  5  10 14 18

        let root = build_bst(&[13, 9, 16, 5, 10, 14, 18]);

        // Case 1: Floor/Ceil of existing value (10)
        assert_eq!(floor(root.clone(), 10), Some(10));
        assert_eq!(ceil(root.clone(), 10), Some(10));

        // Case 2: Floor/Ceil of value between nodes (17)
        // 17 is between 16 and 18.
        assert_eq!(floor(root.clone(), 17), Some(16));
        assert_eq!(ceil(root.clone(), 17), Some(18));

        // Case 3: Floor/Ceil of value smaller than all (4)
        assert_eq!(floor(root.clone(), 4), None);
        assert_eq!(ceil(root.clone(), 4), Some(5));

        // Case 4: Floor/Ceil of value larger than all (20)
        assert_eq!(floor(root.clone(), 20), Some(18));
        assert_eq!(ceil(root.clone(), 20), None);
    }

    #[test]
    fn test_empty_tree() {
        let root = None;
        assert_eq!(floor(root.clone(), 10), None);
        assert_eq!(ceil(root, 10), None);
    }

    #[test]
    fn test_single_node() {
        let root = build_bst(&[10]);
        // Equal
        assert_eq!(floor(root.clone(), 10), Some(10));
        assert_eq!(ceil(root.clone(), 10), Some(10));
        // Smaller
        assert_eq!(floor(root.clone(), 5), None);
        assert_eq!(ceil(root.clone(), 5), Some(10));
        // Larger
        assert_eq!(floor(root.clone(), 15), Some(10));
        assert_eq!(ceil(root.clone(), 15), None);
    }
}
