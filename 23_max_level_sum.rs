use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
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

pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // [INPUT] root: Option<Rc<RefCell<TreeNode>>>
    // Example: root=Some(0x100) where 0x100→TreeNode{val:3, left:Some(0x200), right:Some(0x300)}
    // 0x200→TreeNode{val:4, left:Some(0x400→val:5), right:Some(0x500→val:1)}
    // 0x300→TreeNode{val:7, left:Some(0x600→val:1), right:Some(0x700→val:2)}
    // All leaf nodes: left=None, right=None

    // [EDGE] Check root=None
    // If root=None → return 0 (no levels, no sum, spec says "return false" but signature is i32)
    // Alternatively return i32::MIN or use Option<i32>? Following spec loosely: return 0

    // [OP] Allocate queue Q
    // VecDeque<Rc<RefCell<TreeNode>>>: double-ended queue for BFS
    // Initial state: Q=empty, capacity=0

    // [OP] Initialize max_sum
    // Type: i32
    // Value: i32::MIN (since all tree values positive, any level sum will be ≥1 > MIN)
    // Why MIN not 0? If we init 0 and tree has 1 node with val=-5 (but problem says positive), sum=-5 < 0 → max stays 0 (WRONG)
    // Since problem guarantees positive: init 0 safe? If tree non-empty, first level updates max ✓
    // Conservative: use i32::MIN

    // [LOOP] BFS outer loop: while Q not empty
    // Each iteration processes one complete level

    // [LOOP INNER] For each level:
    // 1. level_count = Q.len() [CRITICAL: capture size BEFORE popping, else count changes mid-loop → F7]
    // 2. curr_sum = 0 [CRITICAL: reset for each level → F10]
    // 3. For i in 0..level_count:
    //    a. node_rc = Q.pop_front().unwrap() [safe: loop count guarantees Q non-empty]
    //    b. node = node_rc.borrow() [acquires Ref<TreeNode>]
    //    c. curr_sum += node.val [accumulate level sum]
    //    d. if let Some(left) = &node.left { Q.push_back(Rc::clone(left)); } [push left child if exists → F8]
    //    e. if let Some(right) = &node.right { Q.push_back(Rc::clone(right)); } [push right child]
    // 4. max_sum = max(max_sum, curr_sum) [update global max → F4]

    // [OUTPUT] return max_sum

    // STOP. Waiting for block-by-block implementation.
    // DO NOT WRITE SOLUTION.

    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    let mut max_sum: i32 = i32::MIN;

    if let Some(root_node) = root {
        queue.push_back(root_node);
    } else {
        return 0 as i32;
    }

    while !queue.is_empty() {
        let level_count = queue.len();
        let mut curr_sum: i32 = 0;
        for _ in 0..level_count {
            // Pop front of queue → node_rc: Rc<RefCell<TreeNode>>
            // queue=[0x100] → pop_front() → Some(0x100) → unwrap() → 0x100
            let node_rc = queue.pop_front().unwrap();

            // Borrow node to read val, left, right
            // 0x100.borrow() → Ref<TreeNode{val:3, left:Some(0x200), right:Some(0x300)}>
            let node = node_rc.borrow();

            // Accumulate sum: curr_sum=0 + 3 = 3
            curr_sum += node.val;

            // Push left child if exists: Some(0x200) → push 0x200
            if let Some(left) = &node.left {
                queue.push_back(Rc::clone(left));
            }

            // Push right child if exists: Some(0x300) → push 0x300
            if let Some(right) = &node.right {
                queue.push_back(Rc::clone(right));
            }
        }
        // After level: curr_sum=3 (L0), max_sum=max(-2147483648, 3)=3
        max_sum = max_sum.max(curr_sum);
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_rc(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    #[test]
    fn test_max_level_sum_basic() {
        // [TEST CASE 1] From problem statement
        // Tree structure:
        //       3
        //      / \
        //     4   7
        //    / \ / \
        //   5  1 1  2

        // [BUILD] Construct tree bottom-up
        // Leaf nodes: 5,1,1,2 (no children)
        let node5 = TreeNode::new(5);
        let node1_left = TreeNode::new(1);
        let node1_right_of_7 = TreeNode::new(1);
        let node2 = TreeNode::new(2);

        // [BUILD] Internal node 4: left=5, right=1
        let mut node4 = TreeNode::new(4);
        node4.left = to_rc(node5);
        node4.right = to_rc(node1_left);

        // [BUILD] Internal node 7: left=1, right=2
        let mut node7 = TreeNode::new(7);
        node7.left = to_rc(node1_right_of_7);
        node7.right = to_rc(node2);

        // [BUILD] Root node 3: left=4, right=7
        let mut node3 = TreeNode::new(3);
        node3.left = to_rc(node4);
        node3.right = to_rc(node7);

        // [EXPECTED] Level sums:
        // L0: 3 → sum=3
        // L1: 4+7 → sum=11
        // L2: 5+1+1+2 → sum=9
        // max(3, 11, 9) = 11

        let result = max_level_sum(to_rc(node3));
        assert_eq!(result, 11);
    }

    #[test]
    fn test_empty_tree() {
        // [TEST CASE 2] root=None
        // [EXPECTED] return 0 (no levels)
        let result = max_level_sum(None);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_node() {
        // [TEST CASE 3] root only, val=42
        // [EXPECTED] L0: sum=42, max=42
        let result = max_level_sum(to_rc(TreeNode::new(42)));
        assert_eq!(result, 42);
    }

    #[test]
    fn test_two_levels_root_larger() {
        // [TEST CASE 4]
        // Tree:
        //     10
        //    /  \
        //   3    4
        // [EXPECTED] L0: 10, L1: 3+4=7, max=10
        let mut root = TreeNode::new(10);
        root.left = to_rc(TreeNode::new(3));
        root.right = to_rc(TreeNode::new(4));

        let result = max_level_sum(to_rc(root));
        assert_eq!(result, 10);
    }

    #[test]
    fn test_two_levels_children_larger() {
        // [TEST CASE 5]
        // Tree:
        //      2
        //     / \
        //    3   4
        // [EXPECTED] L0: 2, L1: 3+4=7, max=7
        let mut root = TreeNode::new(2);
        root.left = to_rc(TreeNode::new(3));
        root.right = to_rc(TreeNode::new(4));

        let result = max_level_sum(to_rc(root));
        assert_eq!(result, 7);
    }

    #[test]
    fn test_skewed_left() {
        // [TEST CASE 6] Left-skewed tree
        // Tree:
        //   1
        //  /
        // 2
        // /
        // 3
        // /
        // 4
        // [EXPECTED] L0: 1, L1: 2, L2: 3, L3: 4, max=4
        let mut node1 = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);
        let mut node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);

        node3.left = to_rc(node4);
        node2.left = to_rc(node3);
        node1.left = to_rc(node2);

        let result = max_level_sum(to_rc(node1));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_single_child_mix() {
        // [TEST CASE 7] Tree with missing children
        // Tree:
        //     10
        //    /
        //   5
        //    \
        //     3
        // [EXPECTED] L0: 10, L1: 5, L2: 3, max=10
        let mut node10 = TreeNode::new(10);
        let mut node5 = TreeNode::new(5);
        let node3 = TreeNode::new(3);

        node5.right = to_rc(node3);
        node10.left = to_rc(node5);

        let result = max_level_sum(to_rc(node10));
        assert_eq!(result, 10);
    }

    #[test]
    fn test_full_balanced_tree() {
        // [TEST CASE 8] Full balanced tree depth=2
        // Tree:
        //       10
        //      /  \
        //     5    15
        //    / \   / \
        //   2  3  7  8
        // [EXPECTED] L0: 10, L1: 5+15=20, L2: 2+3+7+8=20, max=20 (tie at 20)
        let mut root = TreeNode::new(10);

        let mut node5 = TreeNode::new(5);
        node5.left = to_rc(TreeNode::new(2));
        node5.right = to_rc(TreeNode::new(3));

        let mut node15 = TreeNode::new(15);
        node15.left = to_rc(TreeNode::new(7));
        node15.right = to_rc(TreeNode::new(8));

        root.left = to_rc(node5);
        root.right = to_rc(node15);

        let result = max_level_sum(to_rc(root));
        assert_eq!(result, 20);
    }
}
