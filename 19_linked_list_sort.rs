// 19: Sorting Linked List
// Constraint: cannot copy node values, only rewire .next pointers
// External applications hold references to nodes; addresses must preserve values
// Time: O(n log n) merge sort, O(n²) insertion sort
// Space: O(log n) recursive, O(1) bottom-up
//
// Address preservation example:
//   input:  0x100(4)→0x108(2)→0x110(3)→0x118(1)→∅
//   output: 0x118(1)→0x108(2)→0x110(3)→0x100(4)→∅
//   external_ptr→0x108 still sees val=2 ✓
//
// F1: copy vals → ✗ breaks external refs
// F2: swap vals → ✗ breaks external refs
// F3: odd N split → must find correct mid
// F4: forget break left tail → merge infinite loop
// F5: merge without dummy → first node complex
// F6: N=1 → return immediately
// F7: N=2 → one comparison

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    // val: i32
    // Address: 0x100, 0x108, 0x110, 0x118
    // Example: 0x100.val=4, 0x108.val=2, 0x110.val=3, 0x118.val=1
    pub val: i32,
    
    // next: Link
    // Example: 0x100.next=0x108, 0x108.next=0x110
    // After sort: 0x118.next=0x108, 0x108.next=0x110, 0x110.next=0x100
    pub next: Link,
}

impl Node {
    pub fn new(val: i32) -> Rc<RefCell<Node>> {
        // Creates node at new address
        // val=4 → 0x100(4), val=2 → 0x108(2)
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

// find_middle: head → (left_tail, mid)
//
// Input: 0x100(4)→0x108(2)→0x110(3)→0x118(1)→∅
//   N=4, mid at position 2 (0-indexed)
//   s=0x100, f=0x100
//   iter₁: s→0x108, f→0x110
//   iter₂: s→0x110, f→∅ (or f.next=∅)
//   stop: f=∅ or f.next=∅
//   mid=0x110, left_tail=0x108
//
// N=5: [1,2,3,4,5]
//   s→1,2,3; f→1,3,5,∅
//   mid=3, left=[1,2], right=[3,4,5]
//
// N=3: [1,2,3]
//   s→1,2; f→1,3,∅
//   mid=2, left=[1], right=[2,3]
//
// F3: N=2 → s→1,2; f→1,3=∅, mid=2, left=[1], right=[2]
//
pub fn find_middle(_head: &Link) -> (Link, Link) {
    // TODO: slow/fast to find middle
    // return (left_tail, middle)
    // caller must break: left_tail.next = None
    let mut slow = _head.clone();
    let mut fast = _head.clone();
    let mut prev:Link  = None;
    while fast.is_some() && fast.as_ref().unwrap().borrow().next.is_some()
    {
        prev = slow.clone();
        slow = slow.and_then
            (|node| 
            node.borrow().next.clone());
        fast = fast.and_then(|node| node.borrow().next.clone());
        fast = fast.and_then(|node| node.borrow().next.clone());
        
    }
    (prev, slow)
}

// split_list: head → (left_head, right_head)
//
// Input: 0x100(4)→0x108(2)→0x110(3)→0x118(1)→∅
//   find_middle → left_tail=0x108, mid=0x110
//   break: 0x108.next = None
//   left:  0x100(4)→0x108(2)→∅
//   right: 0x110(3)→0x118(1)→∅
//
// F4: forget 0x108.next=None → left still points to right → merge loops
//
pub fn split_list(_head: Link) -> (Link, Link) {
    let (left_tail, middle) = find_middle(&_head);
    if let Some(node) = &left_tail {
        node.borrow_mut().next = None;
    }
    (_head, middle)
}

// merge_sorted: left, right → merged_head
//
// Input: left=0x108(2)→0x100(4)→∅, right=0x118(1)→0x110(3)→∅
//   Compare: 2 vs 1, 1<2, pick 0x118(1), advance right
//   Compare: 2 vs 3, 2<3, pick 0x108(2), advance left
//   Compare: 4 vs 3, 3<4, pick 0x110(3), advance right
//   right=∅, append left: 0x100(4)
//   result: 0x118(1)→0x108(2)→0x110(3)→0x100(4)→∅
//
// Algorithm:
//   dummy = Node::new(0)
//   curr = dummy
//   while L≠∅ ∧ R≠∅:
//     if L.val ≤ R.val: curr.next=L, L=L.next
//     else: curr.next=R, R=R.next
//     curr=curr.next
//   curr.next = L or R (whichever is not None)
//   return dummy.next
//
// F5: without dummy → handling first node is complex
// F9: forget to append leftover → loses tail
//
pub fn merge_sorted(_left: Link, _right: Link) -> Link {
    let dummy = Node::new(0);
    let mut curr = dummy.clone();
    let mut left = _left;
    let mut right = _right;
    
    loop {
        let (l_node, r_node) = match (&left, &right) {
            (Some(l), Some(r)) => (l.clone(), r.clone()),
            _ => break,
        };
        
        if l_node.borrow().val <= r_node.borrow().val {
            curr.borrow_mut().next = left.clone();
            left = left.and_then(|n| n.borrow().next.clone());
        } else {
            curr.borrow_mut().next = right.clone();
            right = right.and_then(|n| n.borrow().next.clone());
        }
        
        let next_node = curr.borrow().next.clone();
        if let Some(n) = next_node {
            curr = n;
        }
    }
    
    if left.is_some() {
        curr.borrow_mut().next = left;
    } else {
        curr.borrow_mut().next = right;
    }
    
    let result = dummy.borrow().next.clone();
    result
}

// merge_sort: head → sorted_head
//
// Input: 0x100(4)→0x108(2)→0x110(3)→0x118(1)→∅
//   Base case: N≤1 → return head
//   split → left=[4,2], right=[3,1]
//   recurse left → [2,4]
//   recurse right → [1,3]
//   merge → [1,2,3,4]
//
// Recursion tree for N=4:
//   [4,2,3,1]
//     ├─[4,2]
//     │   ├─[4] → return
//     │   └─[2] → return
//     │   merge→[2,4]
//     └─[3,1]
//         ├─[3] → return
//         └─[1] → return
//         merge→[1,3]
//   merge→[1,2,3,4]
//
// T(n) = 2T(n/2) + O(n) → O(n log n)
//
pub fn merge_sort(_head: Link) -> Link {
    let head_node = match &_head {
        Some(n) => n.clone(),
        None => return None,
    };
    
    if head_node.borrow().next.is_none() {
        return _head;
    }
    
    let (left, right) = split_list(_head);
    let sorted_left = merge_sort(left);
    let sorted_right = merge_sort(right);
    merge_sorted(sorted_left, sorted_right)
}

// insertion_sort: head → sorted_head
//
// Input: 0x100(4)→0x108(2)→0x110(3)→0x118(1)→∅
//
// step₁: sorted=∅, curr=0x100(4)
//   insert 4 at head: sorted=0x100(4)→∅
//
// step₂: curr=0x108(2)
//   2<4, insert before 4: sorted=0x108(2)→0x100(4)→∅
//
// step₃: curr=0x110(3)
//   2<3<4, insert between: sorted=0x108(2)→0x110(3)→0x100(4)→∅
//
// step₄: curr=0x118(1)
//   1<2, insert before 2: sorted=0x118(1)→0x108(2)→0x110(3)→0x100(4)→∅
//
// O(n²) time worst case (reversed list)
//
// F11: insertion at head → must update sorted_head
// F12: insertion at tail → traverse entire sorted
//
pub fn insertion_sort(_head: Link) -> Link {
    let mut sorted: Link = None;
    let mut current = _head;
    
    while let Some(node) = current {
        let next = node.borrow().next.clone();
        node.borrow_mut().next = None;
        sorted = sorted_insert(sorted, node);
        current = next;
    }
    sorted
}

// sorted_insert: sorted_head, node → new_sorted_head
//
// Insert single node into sorted list
//
// Example 1: sorted=0x108(2)→0x100(4)→∅, node=0x110(3)
//   2<3<4, insert between
//   result: 0x108(2)→0x110(3)→0x100(4)→∅
//
// Example 2: sorted=0x108(2)→∅, node=0x118(1)
//   1<2, insert at head
//   result: 0x118(1)→0x108(2)→∅
//
// Example 3: sorted=0x108(2)→∅, node=0x100(4)
//   4>2, insert at tail
//   result: 0x108(2)→0x100(4)→∅
//
pub fn sorted_insert(_sorted_head: Link, _node: Rc<RefCell<Node>>) -> Link {
    let node_val = _node.borrow().val;
    
    if _sorted_head.is_none() {
        return Some(_node);
    }
    
    let head_val = match &_sorted_head {
        Some(h) => h.borrow().val,
        None => return Some(_node),
    };
    
    if head_val >= node_val {
        _node.borrow_mut().next = _sorted_head;
        return Some(_node);
    }
    
    let mut curr = _sorted_head.clone();
    loop {
        let curr_node = match &curr {
            Some(n) => n.clone(),
            None => break,
        };
        
        let next = curr_node.borrow().next.clone();
        let should_insert = match &next {
            None => true,
            Some(n) => n.borrow().val >= node_val,
        };
        
        if should_insert {
            _node.borrow_mut().next = next;
            curr_node.borrow_mut().next = Some(_node);
            break;
        }
        
        curr = curr_node.borrow().next.clone();
    }
    _sorted_head
}

// Helper: create list from values
// [4,2,3,1] → 0x100(4)→0x108(2)→0x110(3)→0x118(1)→∅
pub fn from_vec(values: Vec<i32>) -> Link {
    if values.is_empty() {
        return None;
    }
    
    let nodes: Vec<Rc<RefCell<Node>>> = values
        .iter()
        .map(|&v| Node::new(v))
        .collect();
    
    for i in 0..nodes.len() - 1 {
        nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
    }
    
    Some(Rc::clone(&nodes[0]))
}

// Helper: list to vec
// 0x118(1)→0x108(2)→0x110(3)→0x100(4)→∅ → [1,2,3,4]
pub fn to_vec(head: &Link) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head.clone();
    let mut count = 0;
    let max_iter = 1000;
    
    while let Some(node) = current {
        if count >= max_iter {
            panic!("Infinite loop detected!");
        }
        result.push(node.borrow().val);
        current = node.borrow().next.clone();
        count += 1;
    }
    
    result
}

// Helper: get node addresses for verification
// Returns vec of (address_id, value) pairs
pub fn get_node_addresses(head: &Link) -> Vec<(usize, i32)> {
    let mut result = Vec::new();
    let mut current = head.clone();
    
    while let Some(node) = current {
        let addr = Rc::as_ptr(&node) as usize;
        let val = node.borrow().val;
        result.push((addr, val));
        current = node.borrow().next.clone();
    }
    
    result
}

fn main() {
    // Example: [4,2,3,1]
    let values = vec![4, 2, 3, 1];
    let head = from_vec(values.clone());
    
    println!("Input: {:?}", values);
    println!("Before sort: {:?}", to_vec(&head));
    
    // Get addresses before sort
    let addrs_before = get_node_addresses(&head);
    println!("Addresses before: {:?}", addrs_before);
    
    // TODO: After implementing merge_sort:
    // let sorted = merge_sort(head);
    // println!("After sort: {:?}", to_vec(&sorted));
    // let addrs_after = get_node_addresses(&sorted);
    // println!("Addresses after: {:?}", addrs_after);
    // Verify: same addresses, same values, different order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n4_basic() {
        // [4,2,3,1] → [1,2,3,4]
        let head = from_vec(vec![4, 2, 3, 1]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_n1_single() {
        // [5] → [5]
        let head = from_vec(vec![5]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![5]);
    }

    #[test]
    fn test_n0_empty() {
        // [] → []
        let head = from_vec(vec![]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![]);
    }

    #[test]
    fn test_n2_swap() {
        // [3,1] → [1,3]
        let head = from_vec(vec![3, 1]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 3]);
    }

    #[test]
    fn test_n2_sorted() {
        // [1,3] → [1,3]
        let head = from_vec(vec![1, 3]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 3]);
    }

    #[test]
    fn test_n5_odd() {
        // [5,1,4,2,3] → [1,2,3,4,5]
        let head = from_vec(vec![5, 1, 4, 2, 3]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_n7_larger() {
        // [7,4,9,2,5,1,8] → [1,2,4,5,7,8,9]
        let head = from_vec(vec![7, 4, 9, 2, 5, 1, 8]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 2, 4, 5, 7, 8, 9]);
    }

    #[test]
    fn test_n8_power_of_2() {
        // [8,4,6,2,7,3,5,1] → [1,2,3,4,5,6,7,8]
        let head = from_vec(vec![8, 4, 6, 2, 7, 3, 5, 1]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_reversed() {
        // [5,4,3,2,1] → [1,2,3,4,5]
        let head = from_vec(vec![5, 4, 3, 2, 1]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_already_sorted() {
        // [1,2,3,4,5] → [1,2,3,4,5]
        let head = from_vec(vec![1, 2, 3, 4, 5]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_duplicates() {
        // [3,1,2,1,3] → [1,1,2,3,3]
        let head = from_vec(vec![3, 1, 2, 1, 3]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_all_same() {
        // [5,5,5,5] → [5,5,5,5]
        let head = from_vec(vec![5, 5, 5, 5]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![5, 5, 5, 5]);
    }

    #[test]
    fn test_negative() {
        // [3,-1,2,-5,0] → [-5,-1,0,2,3]
        let head = from_vec(vec![3, -1, 2, -5, 0]);
        let sorted = merge_sort(head);
        assert_eq!(to_vec(&sorted), vec![-5, -1, 0, 2, 3]);
    }

    #[test]
    fn test_address_preservation() {
        // Verify addresses stay same, only .next rewired
        let head = from_vec(vec![4, 2, 3, 1]);
        let addrs_before: Vec<(usize, i32)> = get_node_addresses(&head);
        
        let sorted = merge_sort(head);
        let addrs_after: Vec<(usize, i32)> = get_node_addresses(&sorted);
        
        // Each (addr, val) pair should exist in before
        for (addr, val) in &addrs_after {
            assert!(addrs_before.contains(&(*addr, *val)), 
                "Address {:#x} with val {} not found in original", addr, val);
        }
        
        // Same number of nodes
        assert_eq!(addrs_before.len(), addrs_after.len());
    }

    // Insertion sort tests
    #[test]
    fn test_insertion_n4() {
        let head = from_vec(vec![4, 2, 3, 1]);
        let sorted = insertion_sort(head);
        assert_eq!(to_vec(&sorted), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_insertion_n1() {
        let head = from_vec(vec![7]);
        let sorted = insertion_sort(head);
        assert_eq!(to_vec(&sorted), vec![7]);
    }

    #[test]
    fn test_insertion_n0() {
        let head = from_vec(vec![]);
        let sorted = insertion_sort(head);
        assert_eq!(to_vec(&sorted), vec![]);
    }

    // Merge function tests
    #[test]
    fn test_merge_basic() {
        let left = from_vec(vec![2, 4]);
        let right = from_vec(vec![1, 3]);
        let merged = merge_sorted(left, right);
        assert_eq!(to_vec(&merged), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_merge_unequal() {
        let left = from_vec(vec![1, 3, 5]);
        let right = from_vec(vec![2, 4]);
        let merged = merge_sorted(left, right);
        assert_eq!(to_vec(&merged), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_one_empty() {
        let left = from_vec(vec![1, 2, 3]);
        let right = from_vec(vec![]);
        let merged = merge_sorted(left, right);
        assert_eq!(to_vec(&merged), vec![1, 2, 3]);
    }
}
