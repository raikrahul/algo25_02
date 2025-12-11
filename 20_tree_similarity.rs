// 20_tree_similarity.rs
// Tree Similarity: check if two binary trees have identical structure
// Time: O(min(n1,n2)), Space: O(min(h1,h2))

use std::cell::RefCell;
use std::rc::Rc;

// TreeNode definition
// addr=0x100: val=5, left=0x108, right=0x110
// addr=0x108: val=3, left=∅, right=∅
// addr=0x110: val=7, left=∅, right=∅
#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        // alloc: heap[0x100] = {val, left=None, right=None}
        // size: 8 + 8 + 8 = 24 bytes (val + 2 Option<Rc<RefCell>>)
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn with_children(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Self {
        // example: with_children(1, Some(node2), Some(node3))
        // result: {val=1, left→0x108, right→0x110}
        TreeNode { val, left, right }
    }
}

type TreeNodeRef = Option<Rc<RefCell<TreeNode>>>;

// Helper to wrap node in Rc<RefCell<>>
fn wrap(node: TreeNode) -> TreeNodeRef {
    // input: TreeNode{val=5, left=None, right=None}
    // output: Some(Rc::new(RefCell::new(node)))
    // ref_count: 1
    Some(Rc::new(RefCell::new(node)))
}

/// Check if two binary trees have identical structure
/// 
/// Example trace:
/// t1:     1          t2:     9
///        / \               / \
///       2   3             8   7
/// 
/// call₀: is_similar(Some(1), Some(9))
///   t1=Some ✓, t2=Some ✓ → both non-null
///   call₁: is_similar(Some(2), Some(8)) → true
///   call₂: is_similar(Some(3), Some(7)) → true
///   return: true ∧ true = true
/// 
/// Failures to avoid:
/// F1: checking val equality (1≠9) → WRONG, structure only
/// F2: returning after one subtree matches → need BOTH
/// F3: wrong base case order → check both-null BEFORE one-null
pub fn is_similar(t1: TreeNodeRef, t2: TreeNodeRef) -> bool {
    // TODO: implement
    // 
    // Base cases:
    // t1=None, t2=None → ?
    // t1=None, t2=Some → ?
    // t1=Some, t2=None → ?
    // 
    // Recursive case:
    // t1=Some, t2=Some → recurse(left) ? recurse(right)
    // 
    // Combine with: ∧ or ∨ ?

    match (t1, t2) 
    {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(t1), Some(t2)) =>
        {
          let left1     =   t1.borrow().left.clone();
          let left2     =   t2.borrow().left.clone();
          let right1    =  t1.borrow().right.clone();
          let right2    =  t2.borrow().right.clone();
          return is_similar(left1, left2) && is_similar(right1, right2)

        }
        
    }
}

fn main() {
    println!("Tree Similarity - run tests with: cargo test --bin 20_tree_similarity");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build trees quickly
    fn leaf(val: i32) -> TreeNodeRef {
        wrap(TreeNode::new(val))
    }

    fn node(val: i32, left: TreeNodeRef, right: TreeNodeRef) -> TreeNodeRef {
        wrap(TreeNode::with_children(val, left, right))
    }

    #[test]
    fn test_both_empty() {
        // t1 = ∅, t2 = ∅
        // expected: true (both empty = similar)
        let t1: TreeNodeRef = None;
        let t2: TreeNodeRef = None;
        assert_eq!(is_similar(t1, t2), true);
    }

    #[test]
    fn test_one_empty_t1() {
        // t1 = ∅, t2 = [5]
        // expected: false (one empty ⊕ one not)
        let t1: TreeNodeRef = None;
        let t2 = leaf(5);
        assert_eq!(is_similar(t1, t2), false);
    }

    #[test]
    fn test_one_empty_t2() {
        // t1 = [5], t2 = ∅
        // expected: false (one empty ⊕ one not)
        let t1 = leaf(5);
        let t2: TreeNodeRef = None;
        assert_eq!(is_similar(t1, t2), false);
    }

    #[test]
    fn test_single_nodes() {
        // t1 = [7], t2 = [99]
        // structure: {root} = {root}
        // expected: true (same structure, different values)
        let t1 = leaf(7);
        let t2 = leaf(99);
        assert_eq!(is_similar(t1, t2), true);
    }

    #[test]
    fn test_same_structure_different_values() {
        // t1:       1          t2:       9
        //          / \               / \
        //         2   3             8   7
        // 
        // structure₁ = structure₂ → similar
        let t1 = node(1, leaf(2), leaf(3));
        let t2 = node(9, leaf(8), leaf(7));
        assert_eq!(is_similar(t1, t2), true);
    }

    #[test]
    fn test_different_structure_left_vs_right() {
        // t1:       A          t2:       B
        //          /                      \
        //         C                        D
        // 
        // t1.left=Some, t1.right=None
        // t2.left=None, t2.right=Some
        // expected: false
        let t1 = node(1, leaf(2), None);
        let t2 = node(1, None, leaf(2));
        assert_eq!(is_similar(t1, t2), false);
    }

    #[test]
    fn test_asymmetric_depth() {
        // t1:       R          t2:       R
        //          / \               / \
        //         L   ∅             L   R
        // 
        // t1: left child only
        // t2: both children
        // expected: false
        let t1 = node(1, leaf(2), None);
        let t2 = node(1, leaf(2), leaf(3));
        assert_eq!(is_similar(t1, t2), false);
    }

    #[test]
    fn test_deep_identical_structure() {
        // t1:           1              t2:           9
        //             /   \                       /   \
        //            2     3                     8     7
        //           / \   / \                   / \   / \
        //          4   5 6   7                 1   2 3   4
        // 
        // 7 nodes each, depth=3, all structure matches
        let t1 = node(1, 
            node(2, leaf(4), leaf(5)),
            node(3, leaf(6), leaf(7))
        );
        let t2 = node(9,
            node(8, leaf(1), leaf(2)),
            node(7, leaf(3), leaf(4))
        );
        assert_eq!(is_similar(t1, t2), true);
    }

    #[test]
    fn test_deep_different_structure() {
        // t1:           1              t2:           9
        //             /   \                       /   \
        //            2     3                     8     7
        //           /     / \                   / \     \
        //          4     6   7                 1   2     4
        // 
        // t1: node 2 has only left child
        // t2: node 8 has both children
        // expected: false
        let t1 = node(1,
            node(2, leaf(4), None),
            node(3, leaf(6), leaf(7))
        );
        let t2 = node(9,
            node(8, leaf(1), leaf(2)),
            node(7, None, leaf(4))
        );
        assert_eq!(is_similar(t1, t2), false);
    }

    #[test]
    fn test_left_skewed_same() {
        // t1:     1          t2:     9
        //        /                  /
        //       2                  8
        //      /                  /
        //     3                  7
        // 
        // both left-skewed, depth=3
        // expected: true
        let t1 = node(1, node(2, leaf(3), None), None);
        let t2 = node(9, node(8, leaf(7), None), None);
        assert_eq!(is_similar(t1, t2), true);
    }

    #[test]
    fn test_right_skewed_same() {
        // t1:     1          t2:     9
        //          \                  \
        //           2                  8
        //            \                  \
        //             3                  7
        // 
        // both right-skewed, depth=3
        // expected: true
        let t1 = node(1, None, node(2, None, leaf(3)));
        let t2 = node(9, None, node(8, None, leaf(7)));
        assert_eq!(is_similar(t1, t2), true);
    }

    #[test]
    fn test_left_vs_right_skewed() {
        // t1:     1          t2:     1
        //        /                    \
        //       2                      2
        //      /                        \
        //     3                          3
        // 
        // t1: left-skewed, t2: right-skewed
        // expected: false
        let t1 = node(1, node(2, leaf(3), None), None);
        let t2 = node(1, None, node(2, None, leaf(3)));
        assert_eq!(is_similar(t1, t2), false);
    }
}
