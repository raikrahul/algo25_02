// 17: Clone Linked List with Random Pointer
// Rc<RefCell<Node>> for shared ownership + interior mutability
// O(N) time, O(1) auxiliary space via interleaving

use std::cell::RefCell;
use std::rc::Rc;

// Node with Rc<RefCell<>> for random pointer
// random can point to any node, self, or None
pub type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<NodeRef>,
    pub random: Option<NodeRef>,
}

impl Node {
    pub fn new(val: i32) -> NodeRef {
        Rc::new(RefCell::new(Node {
            val,
            next: None,
            random: None,
        }))
    }
}

// clone_list: original_head → cloned_head
//
// DERIVATION (from markdown lines 42-48):
// 42. orig[i].r = orig[j]           ← given
// 43. orig[j].next = clone[j]       ← temporary storage
// 44. ∴ orig[i].r.next = clone[j]   ← substitution
// 45. ∴ clone[i].r = orig[i].r.next ← assignment
// 46. ∴ O(1) lookup
//
// PHASE₁: interleave clones into original chain
// PHASE₂: copy random pointers using orig.r.next
// PHASE₃: separate chains
//
pub fn clone_list(head: Option<NodeRef>) -> Option<NodeRef> {
    // line 148: N=0 → return ∅
    let head = head?;

    // TODO: PHASE₁ - Interleave
    // lines 65-82 in markdown
    // cur = head
    // while cur is Some:
    //   clone = Node::new(cur.val)
    //   clone.next = cur.next
    //   cur.next = clone
    //   cur = clone.next
    //
    // Example (N=3):
    // before: 0x100→0x108→0x110→∅
    // after:  0x100→0x200→0x108→0x208→0x110→0x210→∅

    // TODO: PHASE₂ - Copy random
    // lines 88-106 in markdown
    // orig = head
    // while orig is Some:
    //   clone = orig.next
    //   if orig.random is Some:
    //     clone.random = orig.random.next  ← KEY INSIGHT (line 45)
    //   else:
    //     clone.random = None  ← GUARD₁ (line 143)
    //   orig = clone.next
    //
    // Example:
    // orig.r = 0x110, 0x110.next = 0x210
    // clone.r = 0x210 ✓

    // TODO: PHASE₃ - Separate
    // lines 118-132 in markdown
    // clone_head = head.next  ← save before separation
    // orig = head
    // while orig is Some:
    //   clone = orig.next
    //   orig.next = clone.next
    //   if clone.next is Some:
    //     clone.next = clone.next.next  ← GUARD₂ (line 147)
    //   orig = orig.next
    //
    // return clone_head

    // todo!()
    let mut curr = Some(head.clone());
    while let Some(node) = curr 
    {
        let clone = 
            Node::new(node.borrow().val);
        clone.borrow_mut().next = node.borrow().next.clone();

        node.borrow_mut().next = Some(clone.clone());
        curr = clone.borrow().next.clone();
    }

    let mut orig = Some(head.clone());
    while let Some(orig_node)  = orig 
    {
        let clone = orig_node.borrow().next.clone().unwrap();

        if let Some(rand) = orig_node.borrow().random.clone()
        {
            clone.borrow_mut().random = rand.borrow().next.clone();
        }
        orig = clone.borrow().next.clone();
    }

    // P3: separate chains (md 55-72)
    let clone_head = head.borrow().next.clone();
    let mut orig = Some(head.clone());
    
    while let Some(orig_node) = orig {
        let clone = orig_node.borrow().next.clone().unwrap();
        orig_node.borrow_mut().next = clone.borrow().next.clone();
        
        if clone.borrow().next.is_some() {
            let next_orig = clone.borrow().next.clone().unwrap();
            clone.borrow_mut().next = next_orig.borrow().next.clone();
        }
        
        orig = orig_node.borrow().next.clone();
    }
    
    clone_head
}

// Helper: Vec → List (no random pointers set)
// [10,20,30] → 0x100(10)→0x108(20)→0x110(30)→∅
fn to_list(v: Vec<i32>) -> Option<NodeRef> {
    let mut head: Option<NodeRef> = None;
    for &val in v.iter().rev() {
        let node = Node::new(val);
        node.borrow_mut().next = head;
        head = Some(node);
    }
    head
}

// Helper: List → Vec (values only)
fn to_vec(head: &Option<NodeRef>) -> Vec<i32> {
    let mut v = Vec::new();
    let mut cur = head.clone();
    while let Some(node) = cur {
        v.push(node.borrow().val);
        cur = node.borrow().next.clone();
    }
    v
}

// Helper: Collect all nodes into Vec for indexing
fn collect_nodes(head: &Option<NodeRef>) -> Vec<NodeRef> {
    let mut nodes = Vec::new();
    let mut cur = head.clone();
    while let Some(node) = cur {
        nodes.push(Rc::clone(&node));
        cur = node.borrow().next.clone();
    }
    nodes
}

// Helper: Set random pointers by index
// randoms[i] = index of node that node[i].random points to
// randoms[i] = -1 means random is None
// Example: [2, 1, 0] means:
//   node[0].random = node[2]
//   node[1].random = node[1] (self)
//   node[2].random = node[0]
fn set_randoms(head: &Option<NodeRef>, randoms: Vec<i32>) {
    let nodes = collect_nodes(head);
    for (i, &rand_idx) in randoms.iter().enumerate() {
        if rand_idx >= 0 {
            nodes[i].borrow_mut().random = Some(Rc::clone(&nodes[rand_idx as usize]));
        }
    }
}

// Helper: Get random pointer indices (for verification)
// Returns vector of indices, -1 for None random
fn get_random_indices(head: &Option<NodeRef>) -> Vec<i32> {
    let nodes = collect_nodes(head);
    let mut indices = Vec::new();
    
    for node in &nodes {
        let random = node.borrow().random.clone();
        if let Some(rand_node) = random {
            // Find index of random target
            let idx = nodes.iter().position(|n| Rc::ptr_eq(n, &rand_node));
            indices.push(idx.map_or(-1, |i| i as i32));
        } else {
            indices.push(-1);
        }
    }
    indices
}

fn main() {
    // Example: N=3, randoms: [2,1,0]
    // node[0].r→node[2], node[1].r→node[1], node[2].r→node[0]
    let head = to_list(vec![10, 20, 30]);
    set_randoms(&head, vec![2, 1, 0]);
    
    println!("Original values: {:?}", to_vec(&head));
    println!("Original randoms: {:?}", get_random_indices(&head));
    
    // TODO: test clone_list
    // let cloned = clone_list(head);
    // println!("Cloned values: {:?}", to_vec(&cloned));
    // println!("Cloned randoms: {:?}", get_random_indices(&cloned));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n3_mixed_randoms() {
        // N=3
        // values: [10, 20, 30]
        // randoms: [2, 1, 0]
        //   node[0].r → node[2]
        //   node[1].r → node[1] (self)
        //   node[2].r → node[0]
        let head = to_list(vec![10, 20, 30]);
        set_randoms(&head, vec![2, 1, 0]);
        
        let cloned = clone_list(head);
        
        assert_eq!(to_vec(&cloned), vec![10, 20, 30]);
        assert_eq!(get_random_indices(&cloned), vec![2, 1, 0]);
    }

    #[test]
    fn test_n5_complex() {
        // N=5
        // values: [10, 20, 30, 40, 50]
        // randoms: [2, 4, 0, 3, 1]
        let head = to_list(vec![10, 20, 30, 40, 50]);
        set_randoms(&head, vec![2, 4, 0, 3, 1]);
        
        let cloned = clone_list(head);
        
        assert_eq!(to_vec(&cloned), vec![10, 20, 30, 40, 50]);
        assert_eq!(get_random_indices(&cloned), vec![2, 4, 0, 3, 1]);
    }

    #[test]
    fn test_n1_self_random() {
        // N=1, random points to self
        let head = to_list(vec![42]);
        set_randoms(&head, vec![0]);
        
        let cloned = clone_list(head);
        
        assert_eq!(to_vec(&cloned), vec![42]);
        assert_eq!(get_random_indices(&cloned), vec![0]);
    }

    #[test]
    fn test_n1_null_random() {
        // N=1, random is None
        let head = to_list(vec![42]);
        // no set_randoms call, random stays None
        
        let cloned = clone_list(head);
        
        assert_eq!(to_vec(&cloned), vec![42]);
        assert_eq!(get_random_indices(&cloned), vec![-1]);
    }

    #[test]
    fn test_n2_cross_randoms() {
        // N=2, cross random pointers
        // node[0].r → node[1]
        // node[1].r → node[0]
        let head = to_list(vec![10, 20]);
        set_randoms(&head, vec![1, 0]);
        
        let cloned = clone_list(head);
        
        assert_eq!(to_vec(&cloned), vec![10, 20]);
        assert_eq!(get_random_indices(&cloned), vec![1, 0]);
    }

    #[test]
    fn test_n0_empty() {
        // N=0, empty list
        let head: Option<NodeRef> = None;
        let cloned = clone_list(head);
        assert!(cloned.is_none());
    }

    #[test]
    fn test_n3_all_null_randoms() {
        // N=3, all random pointers None
        let head = to_list(vec![1, 2, 3]);
        // no set_randoms, all None
        
        let cloned = clone_list(head);
        
        assert_eq!(to_vec(&cloned), vec![1, 2, 3]);
        assert_eq!(get_random_indices(&cloned), vec![-1, -1, -1]);
    }

    #[test]
    fn test_n4_all_self_randoms() {
        // N=4, all random pointers point to self
        let head = to_list(vec![1, 2, 3, 4]);
        set_randoms(&head, vec![0, 1, 2, 3]);
        
        let cloned = clone_list(head);
        
        assert_eq!(to_vec(&cloned), vec![1, 2, 3, 4]);
        assert_eq!(get_random_indices(&cloned), vec![0, 1, 2, 3]);
    }
}
