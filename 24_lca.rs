// ============================================================================
// LEAST COMMON ANCESTOR (LCA) IN BINARY TREE
// ============================================================================
//
// INPUT: tree root t, nodes p, q (valid nodes in tree)
// OUTPUT: first common ancestor while travelling toward root
//
// CONSTRAINT: cannot modify tree node structure
//
// ============================================================================

use std::cell::RefCell;
use std::rc::Rc;

/// Binary tree node.
///
/// MEMORY LAYOUT for node with val=2, left=4, right=5:
///   0x100: TreeNode { val: 2, left: Some(0x110), right: Some(0x120) }
///   0x110: TreeNode { val: 4, left: None, right: None }
///   0x120: TreeNode { val: 5, left: None, right: None }
///
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

/// Find Least Common Ancestor of nodes p and q in tree rooted at t.
///
/// TRACE p=4, q=5 on tree:
///        1
///       / \
///      2   3
///     / \
///    4   5
///
///   find_lca(1,4,5):
///     1!=4, 1!=5
///     left = find_lca(2,4,5)
///       2!=4, 2!=5
///       left = find_lca(4,4,5) → 4==4 → return Some(4)
///       right = find_lca(5,4,5) → 5==5 → return Some(5)
///       left=Some(4), right=Some(5) → BOTH → return Some(2)
///     right = find_lca(3,4,5) → returns None
///     left=Some(2), right=None → return Some(2)
///   ANSWER = 2 ✓
///
/// TRACE p=4, q=3:
///   find_lca(1,4,3):
///     left = find_lca(2,4,3) → returns Some(4) (found 4, not 3)
///     right = find_lca(3,4,3) → 3==3 → return Some(3)
///     left=Some(4), right=Some(3) → BOTH → return Some(1)
///   ANSWER = 1 ✓
///
/// TRACE p=2, q=4 (ancestor case):
///   find_lca(1,2,4):
///     left = find_lca(2,2,4) → 2==2 → return Some(2) (STOP, don't descend)
///     right = find_lca(3,2,4) → None
///     left=Some(2), right=None → return Some(2)
///   ANSWER = 2 ✓ (p is ancestor of q, returns p)
///
/// COMPLEXITY:
///   TIME: O(N) - visit each node at most once
///   SPACE: O(H) - recursion stack, H=height, worst O(N) skewed
///
/// YOUR TRAPS:
///   TRAP1: returning first found instead of checking both subtrees
///   TRAP2: forgetting base case node==p OR node==q
///   TRAP3: continuing recursion after finding p or q
///   TRAP4: confusing "found node" with "LCA node" in return value
///
pub fn find_lca(t: &TreeLink, p: &TreeLink, q: &TreeLink) -> TreeLink {
    // YOUR SOLUTION HERE
    //
    // STEP 1: base case - node is null
    //   if t.is_none() → return None
    //
    // STEP 2: base case - node is p or q
    //   if Rc::ptr_eq(t, p) or Rc::ptr_eq(t, q) → return t.clone()
    //
    // STEP 3: recurse left and right
    //   let left_result = find_lca(&t.borrow().left, p, q)
    //   let right_result = find_lca(&t.borrow().right, p, q)
    //
    // STEP 4: decision logic
    //   if left_result.is_some() AND right_result.is_some() → return t.clone() (CURRENT IS LCA)
    //   if left_result.is_some() → return left_result
    //   if right_result.is_some() → return right_result
    //   return None
    //
    // todo!("implement find_lca")
    if t.is_none() {
        return None;
    }

    let node = t.as_ref().unwrap();
    if Rc::ptr_eq(node, p.as_ref().unwrap()) {
        return t.clone();
    }

    if Rc::ptr_eq(node, q.as_ref().unwrap()) {
        return t.clone();
    }

    // STEP 3: recurse left and right
    let left_result = find_lca(&node.borrow().left, p, q);
    let right_result = find_lca(&node.borrow().right, p, q);

    // STEP 4: decision logic
    if left_result.is_some() && right_result.is_some() {
        // Both found → current node is LCA
        return t.clone();
    }
    if left_result.is_some() {
        return left_result;
    }
    if right_result.is_some() {
        return right_result;
    }
    None
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Create tree node wrapped in Rc<RefCell<>>.
pub fn new_node(val: i32) -> TreeLink {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

/// Attach left child to parent.
///
/// BEFORE: parent=0x100{val:1,left:None,right:None}
/// attach_left(parent, child=0x110{val:2})
/// AFTER: parent=0x100{val:1,left:Some(0x110),right:None}
///
pub fn attach_left(parent: &TreeLink, child: &TreeLink) {
    if let Some(p) = parent {
        p.borrow_mut().left = child.clone();
    }
}

/// Attach right child to parent.
pub fn attach_right(parent: &TreeLink, child: &TreeLink) {
    if let Some(p) = parent {
        p.borrow_mut().right = child.clone();
    }
}

/// Build test tree:
///        1
///       / \
///      2   3
///     / \
///    4   5
///
/// Returns (root, node4, node5, node2, node3) for testing.
///
pub fn build_test_tree() -> (TreeLink, TreeLink, TreeLink, TreeLink, TreeLink) {
    let n1 = new_node(1);
    let n2 = new_node(2);
    let n3 = new_node(3);
    let n4 = new_node(4);
    let n5 = new_node(5);

    attach_left(&n1, &n2);
    attach_right(&n1, &n3);
    attach_left(&n2, &n4);
    attach_right(&n2, &n5);

    (n1, n4, n5, n2, n3)
}

/// Build larger test tree:
///          1
///        /   \
///       2     3
///      / \   / \
///     4   5 6   7
///
/// Returns (root, n4, n5, n6, n7, n2, n3).
///
pub fn build_large_tree() -> (
    TreeLink,
    TreeLink,
    TreeLink,
    TreeLink,
    TreeLink,
    TreeLink,
    TreeLink,
) {
    let n1 = new_node(1);
    let n2 = new_node(2);
    let n3 = new_node(3);
    let n4 = new_node(4);
    let n5 = new_node(5);
    let n6 = new_node(6);
    let n7 = new_node(7);

    attach_left(&n1, &n2);
    attach_right(&n1, &n3);
    attach_left(&n2, &n4);
    attach_right(&n2, &n5);
    attach_left(&n3, &n6);
    attach_right(&n3, &n7);

    (n1, n4, n5, n6, n7, n2, n3)
}

// ============================================================================
// TEST CASES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to check if two TreeLinks point to same node.
    fn same_node(a: &TreeLink, b: &TreeLink) -> bool {
        match (a, b) {
            (Some(ra), Some(rb)) => Rc::ptr_eq(ra, rb),
            (None, None) => true,
            _ => false,
        }
    }

    #[test]
    fn test_lca_siblings() {
        // p=4, q=5 → LCA=2 (siblings under 2)
        //        1
        //       / \
        //      2   3
        //     / \
        //    4   5
        let (root, n4, n5, n2, _n3) = build_test_tree();
        let result = find_lca(&root, &n4, &n5);
        assert!(same_node(&result, &n2), "LCA(4,5) should be 2");
    }

    #[test]
    fn test_lca_different_subtrees() {
        // p=4, q=3 → LCA=1 (4 in left subtree, 3 in right subtree)
        let (root, n4, _n5, _n2, n3) = build_test_tree();
        let result = find_lca(&root, &n4, &n3);
        assert!(same_node(&result, &root), "LCA(4,3) should be 1 (root)");
    }

    #[test]
    fn test_lca_ancestor_case() {
        // p=2, q=4 → LCA=2 (2 is ancestor of 4)
        let (root, n4, _n5, n2, _n3) = build_test_tree();
        let result = find_lca(&root, &n2, &n4);
        assert!(same_node(&result, &n2), "LCA(2,4) should be 2 (ancestor)");
    }

    #[test]
    fn test_lca_same_node() {
        // p=4, q=4 → LCA=4
        let (root, n4, _n5, _n2, _n3) = build_test_tree();
        let result = find_lca(&root, &n4, &n4);
        assert!(same_node(&result, &n4), "LCA(4,4) should be 4");
    }

    #[test]
    fn test_lca_root_is_lca() {
        // p=4, q=7 → LCA=1 (opposite ends of tree)
        //          1
        //        /   \
        //       2     3
        //      / \   / \
        //     4   5 6   7
        let (root, n4, _n5, _n6, n7, _n2, _n3) = build_large_tree();
        let result = find_lca(&root, &n4, &n7);
        assert!(same_node(&result, &root), "LCA(4,7) should be 1 (root)");
    }

    #[test]
    fn test_lca_cousins_left() {
        // p=4, q=5 → LCA=2
        let (root, n4, n5, _n6, _n7, n2, _n3) = build_large_tree();
        let result = find_lca(&root, &n4, &n5);
        assert!(same_node(&result, &n2), "LCA(4,5) should be 2");
    }

    #[test]
    fn test_lca_cousins_right() {
        // p=6, q=7 → LCA=3
        let (root, _n4, _n5, n6, n7, _n2, n3) = build_large_tree();
        let result = find_lca(&root, &n6, &n7);
        assert!(same_node(&result, &n3), "LCA(6,7) should be 3");
    }

    #[test]
    fn test_lca_single_node_tree() {
        // tree = {1}, p=1, q=1 → LCA=1
        let root = new_node(1);
        let result = find_lca(&root, &root, &root);
        assert!(
            same_node(&result, &root),
            "LCA(1,1) in single node should be 1"
        );
    }

    #[test]
    fn test_lca_deep_left() {
        // Skewed tree: 1→2→4, p=4, q=2 → LCA=2
        //    1
        //   /
        //  2
        // /
        //4
        let n1 = new_node(1);
        let n2 = new_node(2);
        let n4 = new_node(4);
        attach_left(&n1, &n2);
        attach_left(&n2, &n4);

        let result = find_lca(&n1, &n4, &n2);
        assert!(
            same_node(&result, &n2),
            "LCA(4,2) in skewed tree should be 2"
        );
    }

    #[test]
    fn test_lca_p_is_root() {
        // p=1 (root), q=4 → LCA=1
        let (root, n4, _n5, _n2, _n3) = build_test_tree();
        let result = find_lca(&root, &root, &n4);
        assert!(same_node(&result, &root), "LCA(1,4) should be 1 (root)");
    }
}

fn main() {
    println!("LCA Problem - Run tests with: cargo test --bin lca");
}
