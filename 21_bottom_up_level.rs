use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;  // VecDeque not VecDequeue

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

/// T=[3,[4,[5,∅,∅],[1,∅,∅]],[7,[6,∅,∅],[8,∅,∅]]] → output=[5,1,6,8,4,7,3]
/// T=[1,[2,∅,∅],[3,∅,∅]] → output=[2,3,1]
/// T=∅ → output=[]
pub fn bottom_up_level(root: Option<Rc<RefCell<TreeNode>>>) {
    // queue: FIFO container for BFS
    // T=[3,4,7,5,1,6,8] → queue will hold: [0x100] → [0x110,0x120] → [0x130,0x140,0x150,0x160]
    // 0x100.val=3, 0x110.val=4, 0x120.val=7, 0x130.val=5, 0x140.val=1, 0x150.val=6, 0x160.val=8
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    
    // T=∅ → root=None → is_none()=true → return immediately → no panic on unwrap
    // T=[3,...] → root=Some(0x100) → is_none()=false → continue
    if root.is_none() { return; }

    // root=Some(0x100) → unwrap()=0x100 → push_back → queue=[0x100] → queue.len()=1
    // push_back not push: VecDeque uses push_back/push_front
    queue.push_back(root.unwrap());
    
    // levels: stores each level as separate Vec
    // after pass₁: levels=[[3]]
    // after pass₂: levels=[[3],[4,7]]
    // after pass₃: levels=[[3],[4,7],[5,1,6,8]]
    // levels.len() at end = 3 = tree height
    let mut levels: Vec<Vec<i32>> = Vec::new();
    
    // pass₁: queue=[0x100] → len=1 → 1>0 ✓ → enter
    // pass₂: queue=[0x110,0x120] → len=2 → 2>0 ✓ → enter
    // pass₃: queue=[0x130,0x140,0x150,0x160] → len=4 → 4>0 ✓ → enter
    // pass₄: queue=[] → len=0 → 0>0 ✗ → exit
    while !queue.is_empty() {
        // CRITICAL: capture len BEFORE loop modifies queue
        // pass₁: level_size=1 → loop 1 time → pop 1 node → push 2 children → queue grows to 2
        // pass₂: level_size=2 → loop 2 times → pop 2 nodes → push 4 children → queue grows to 4
        // pass₃: level_size=4 → loop 4 times → pop 4 nodes → push 0 children (leaves) → queue=0
        // BUG if level_size not saved: queue.len() changes mid-loop → levels merge
        let level_size = queue.len();
        
        // current_level: collects vals for this level only
        // pass₁: current_level will be [3]
        // pass₂: current_level will be [4,7]
        // pass₃: current_level will be [5,1,6,8]
        let mut current_level: Vec<i32> = Vec::new();
        
        // pass₁: i=0 only (level_size=1)
        // pass₂: i=0,1 (level_size=2)
        // pass₃: i=0,1,2,3 (level_size=4)
        for _ in 0..level_size {
            // pass₁ iter₀: queue=[0x100] → pop_front()=Some(0x100) → unwrap()=0x100
            // pass₂ iter₀: queue=[0x110,0x120] → pop_front()=Some(0x110) → node=0x110
            // pass₂ iter₁: queue=[0x120,0x130,0x140] → pop_front()=Some(0x120) → node=0x120
            let node = queue.pop_front().unwrap();
            
            // borrow() returns Ref<TreeNode> for reading
            // pass₁: node=0x100 → borrow() → node_ref.val=3, node_ref.left=Some(0x110), node_ref.right=Some(0x120)
            // pass₃ iter₀: node=0x130 → node_ref.val=5, node_ref.left=None, node_ref.right=None
            let node_ref = node.borrow();  // semicolon not colon!
            
            // pass₁: current_level=[] → push(3) → current_level=[3]
            // pass₂ iter₀: current_level=[] → push(4) → [4]
            // pass₂ iter₁: current_level=[4] → push(7) → [4,7]
            // pass₃: push 5,1,6,8 → [5,1,6,8]
            current_level.push(node_ref.val);
            
            // pass₁: node_ref.left=Some(0x110) → matches → clone()=0x110 → push_back → queue=[0x110]
            // pass₃ iter₀: node_ref.left=None → no match → skip
            if let Some(left) = node_ref.left.clone() {
                queue.push_back(left);  // was: right (BUG: undefined variable)
            }
            
            // pass₁: node_ref.right=Some(0x120) → clone()=0x120 → push_back → queue=[0x110,0x120]
            // pass₃ iter₀: node_ref.right=None → skip
            // THIS BLOCK WAS MISSING
            if let Some(right) = node_ref.right.clone() {
                queue.push_back(right);
            }
        }
        // MOVED OUTSIDE for loop: levels.push must happen once per level, not once per node
        // pass₁: current_level=[3] → levels=[[3]]
        // pass₂: current_level=[4,7] → levels=[[3],[4,7]]
        // pass₃: current_level=[5,1,6,8] → levels=[[3],[4,7],[5,1,6,8]]
        // BUG if inside for: levels would be [[3],[4],[7],[5],[1],[6],[8]] → 7 levels instead of 3
        levels.push(current_level);
    }
    
    // levels=[[3],[4,7],[5,1,6,8]] → iter().rev() yields [5,1,6,8] then [4,7] then [3]
    // output: "5 1 6 8 4 7 3 \n"
    // verification: L₂ first, L₁ second, L₀ last ✓
    for level in levels.iter().rev() {
        for val in level {
            print!("{} ", val);
        }
    }
    println!();
}

fn main() {
    // T=[3,[4,[5,∅,∅],[1,∅,∅]],[7,[6,∅,∅],[8,∅,∅]]]
    // expected output: 5 1 6 8 4 7 3
    println!("Test: Full tree");
    
    let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
    let n4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let n7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let n5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let n6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let n8 = Rc::new(RefCell::new(TreeNode::new(8)));
    
    n3.borrow_mut().left = Some(n4.clone());
    n3.borrow_mut().right = Some(n7.clone());
    n4.borrow_mut().left = Some(n5.clone());
    n4.borrow_mut().right = Some(n1.clone());
    n7.borrow_mut().left = Some(n6.clone());
    n7.borrow_mut().right = Some(n8.clone());
    
    bottom_up_level(Some(n3));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    fn connect_left(parent: &Option<Rc<RefCell<TreeNode>>>, child: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(p) = parent {
            p.borrow_mut().left = child;
        }
    }

    fn connect_right(parent: &Option<Rc<RefCell<TreeNode>>>, child: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(p) = parent {
            p.borrow_mut().right = child;
        }
    }

    #[test]
    fn test_ex_01_null() {
        // T=∅ → output=[]
        bottom_up_level(None);
    }

    #[test]
    fn test_ex_02_single_node() {
        // T=[1,∅,∅] → L₀={1} → output=[1]
        let root = node(1);
        bottom_up_level(root);
    }

    #[test]
    fn test_ex_03_left_child() {
        // T=[1,[2,∅,∅],∅] → L₀={1}, L₁={2} → output=[2,1]
        let root = node(1);
        let child = node(2);
        connect_left(&root, child);
        bottom_up_level(root);
    }

    #[test]
    fn test_ex_04_right_child() {
        // T=[1,∅,[3,∅,∅]] → L₀={1}, L₁={3} → output=[3,1]
        let root = node(1);
        let child = node(3);
        connect_right(&root, child);
        bottom_up_level(root);
    }

    #[test]
    fn test_ex_05_balanced_2() {
        // T=[1,[2,∅,∅],[3,∅,∅]] → L₀={1}, L₁={2,3} → output=[2,3,1]
        let root = node(1);
        let l = node(2);
        let r = node(3);
        connect_left(&root, l);
        connect_right(&root, r);
        bottom_up_level(root);
    }

    #[test]
    fn test_ex_06_skew_left() {
        // T=[1,[2,[3,∅,∅],∅],∅] → L₀={1}, L₁={2}, L₂={3} → output=[3,2,1]
        let root = node(1);
        let l1 = node(2);
        let l2 = node(3);
        connect_left(&root, l1.clone());
        connect_left(&l1, l2);
        bottom_up_level(root);
    }

    #[test]
    fn test_ex_07_full_example() {
        // T=[3,[4,[5,∅,∅],[1,∅,∅]],[7,[6,∅,∅],[8,∅,∅]]]
        // L₀={3}, L₁={4,7}, L₂={5,1,6,8}
        // output=[5,1,6,8,4,7,3]
        
        let root = node(3);
        let n4 = node(4);
        let n7 = node(7);
        let n5 = node(5);
        let n1 = node(1);
        let n6 = node(6);
        let n8 = node(8);

        connect_left(&root, n4.clone());
        connect_right(&root, n7.clone());
        connect_left(&n4, n5);
        connect_right(&n4, n1);
        connect_left(&n7, n6);
        connect_right(&n7, n8);

        bottom_up_level(root);
    }
}
