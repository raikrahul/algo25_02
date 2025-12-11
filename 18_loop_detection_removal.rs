// 18: Loop Detection and Removal in Singly Linked List
// Constraint: O(n) time, O(1) space
// Find loop, find loop start, break loop

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    // val: i32
    // Memory: 0x100, 0x108, 0x110, ...
    // Example: val=10 at 0x100, val=20 at 0x108
    pub val: i32,
    
    // next: Link (Rc<RefCell<Node>> for loop support)
    // Example: 0x100.next = 0x108, 0x128.next = 0x110 (loop)
    pub next: Link,
}

impl Node {
    pub fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

// has_loop: head → bool
// 
// Input Drawing (N=6, loop_idx=2):
//   0x100(10)→0x108(20)→0x110(40)→0x118(60)→0x120(30)→0x128(50)→0x110
//                        ↑_________________________________________|
//
// Calculation:
//   s=0x100, f=0x100
//   iter₁: s→0x108, f→0x110
//   iter₂: s→0x110, f→0x120
//   iter₃: s→0x118, f→0x110 (wrapped)
//   iter₄: s→0x120, f→0x120
//   s=f → loop exists ✓
//
// F1: check s=f before moving → false positive at head
// F2: s+1, f+1 → never meet
//
pub fn has_loop(_head: &Link) -> bool {
    // TODO: implement Floyd's cycle detection
    // s=head, f=head
    // while f.next ≠ ∅ ∧ f.next.next ≠ ∅:
    //   s+1, f+2
    //   if s=f: return true
    // return false
    let mut slow = _head.clone();
    let mut fast = _head.clone();
    
    loop {

        // fast = fast.as_ref().borrow().next.clone();
        // slow = slow.as_ref().borrow().next.clone();
        fast = match fast {
            Some(node) => node.borrow().next.clone(),
            None => return false,
        };
        fast = match fast {
            Some(node) => node.borrow().next.clone(),
            None => return false,
        };

        slow = match slow {
            Some(node) => node.borrow().next.clone(),
            None => return false, // Should not be reached if fast checks are correct
        };

        if let (Some(s_node), Some(f_node)) = (&slow, &fast) {
            if Rc::ptr_eq(s_node, f_node) {
                return true;
            }
        } else {
            // This case should be handled by the match statements above,
            // returning false if fast or slow become None.
            // If we reach here, it implies one of them became None,
            // but the other didn't, which shouldn't happen if fast leads.
            // However, to be fully robust, if we somehow reach here with a None, no loop.
            return false;
        }
    }
}

// find_loop_start: head, meeting_point → loop_start
//
// Input Drawing:
//   head=0x100, meeting=0x120
//   0x100(10)→0x108(20)→0x110(40)→0x118(60)→0x120(30)→0x128(50)→0x110
//
// Mathematics:
//   μ = tail_len, λ = loop_len, k = slow inside loop
//   n×λ = μ + k → μ = n×λ - k
//   ptr1 from head: moves μ → arrives at loop_start
//   ptr2 from meeting: moves μ → arrives at loop_start
//   ∴ they meet at loop_start
//
// F3: assume meeting = loop_start → ✗ (meeting = 0x120, start = 0x110)
// F4: count loop length, ignore tail → ✗
//
pub fn find_loop_start(_head: &Link, _meeting: &Link) -> Link {
    // p1=head=0x100, p2=meeting=0x120
    // step₀: p1=0x100, p2=0x120, ≠
    // step₁: p1→0x108, p2→0x128, ≠
    // step₂: p1→0x110, p2→0x110, = → return 0x110
    let mut p1 = _head.clone();
    let mut p2 = _meeting.clone();

    loop {
        // check if p1 = p2 (both at loop_start)
        match (&p1, &p2) {
            (Some(n1), Some(n2)) if Rc::ptr_eq(n1, n2) => return p1,
            _ => {}
        }
        
        // move p1 forward +1
        p1 = match p1 {
            Some(node) => node.borrow().next.clone(),
            None => return None,
        };
        
        // move p2 forward +1
        p2 = match p2 {
            Some(node) => node.borrow().next.clone(),
            None => return None,
        };
    }
}

// detect_and_remove_loop: head → ()
//
// Three phases:
//   phase 1: detect loop, find meeting point
//   phase 2: find loop start using head + meeting
//   phase 3: traverse loop, find node before loop_start, break
//
// Input Drawing (N=6, loop_idx=2):
//   BEFORE: 0x100→0x108→0x110→0x118→0x120→0x128→0x110 (loop)
//   AFTER:  0x100→0x108→0x110→0x118→0x120→0x128→∅ (linear)
//
// F5: break at meeting.next → breaks middle of loop ✗
// F6: edge N=1 self-loop → 0x100.next = 0x100 → set 0x100.next = ∅
//
pub fn detect_and_remove_loop(_head: &Link) {
    // phase 1: find meeting point using Floyd's algorithm
    // s=0x100, f=0x100 → ... → s=f=0x120 (meeting)
    let mut slow = _head.clone();
    let mut fast = _head.clone();
    let mut meeting: Link = None;
    
    loop {
        // fast +2
        for _ in 0..2 {
            if let Some(f) = fast {
                fast = f.borrow().next.clone();
            } else {
                return; // no loop
            }
        }
        // slow +1
        if let Some(s) = slow {
            slow = s.borrow().next.clone();
        }
        // check meeting
        match (&slow, &fast) {
            (Some(s), Some(f)) if Rc::ptr_eq(s, f) => {
                meeting = slow.clone();
                break;
            }
            (None, _) | (_, None) => return, // no loop
            _ => {}
        }
    }
    
    // phase 2: find loop_start
    // p1=0x100, p2=0x120 → p1=p2=0x110 (loop_start)
    let loop_start = find_loop_start(_head, &meeting);
    
    if loop_start.is_none() {
        return;
    }
    
    // phase 3: find node before loop_start, break loop
    // traverse: 0x110→0x118→0x120→0x128→(0x110)
    // find: 0x128.next = 0x110 = loop_start → set 0x128.next = None
    let loop_start_ref = loop_start.as_ref().unwrap();
    let mut curr = loop_start.clone();
    
    loop {
        if let Some(node) = curr {
            let next = node.borrow().next.clone();
            if let Some(next_node) = &next {
                if Rc::ptr_eq(next_node, loop_start_ref) {
                    // curr.next = loop_start → break here
                    node.borrow_mut().next = None;
                    return;
                }
            }
            curr = next;
        } else {
            return;
        }
    }
}

// Helper: create list with loop
// values: [v0, v1, ..., vn-1], loop_start_idx
// Returns head
// Example: [10,20,40,60,30,50], loop_idx=2
//   0x100(10)→0x108(20)→0x110(40)→0x118(60)→0x120(30)→0x128(50)→0x110
pub fn create_list_with_loop(values: Vec<i32>, loop_start_idx: Option<usize>) -> Link {
    if values.is_empty() {
        return None;
    }
    
    // Create all nodes
    let nodes: Vec<Rc<RefCell<Node>>> = values
        .iter()
        .map(|&v| Node::new(v))
        .collect();
    
    // Link nodes: node[i].next = node[i+1]
    for i in 0..nodes.len() - 1 {
        nodes[i].borrow_mut().next = Some(Rc::clone(&nodes[i + 1]));
    }
    
    // Create loop if specified
    // Last node.next = node[loop_start_idx]
    if let Some(idx) = loop_start_idx {
        if idx < nodes.len() {
            nodes[nodes.len() - 1].borrow_mut().next = Some(Rc::clone(&nodes[idx]));
        }
    }
    
    Some(Rc::clone(&nodes[0]))
}

// Helper: list → vec (only works after loop removed)
pub fn to_vec(head: &Link) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head.clone();
    let mut count = 0;
    let max_iter = 1000; // safety limit
    
    while let Some(node) = current {
        if count >= max_iter {
            panic!("Infinite loop detected - loop not removed!");
        }
        result.push(node.borrow().val);
        current = node.borrow().next.clone();
        count += 1;
    }
    
    result
}

// Helper: check if loop exists (for testing)
pub fn loop_exists(head: &Link) -> bool {
    let mut slow = head.clone();
    let mut fast = head.clone();
    
    loop {
        // Move fast twice
        for _ in 0..2 {
            if let Some(f) = fast {
                fast = f.borrow().next.clone();
            } else {
                return false; // No loop
            }
        }
        
        // Move slow once
        if let Some(s) = slow {
            slow = s.borrow().next.clone();
        }
        
        // Check if they meet
        match (&slow, &fast) {
            (Some(s), Some(f)) => {
                if Rc::ptr_eq(s, f) {
                    return true;
                }
            }
            _ => return false,
        }
    }
}

fn main() {
    // Example: N=6, loop at index 2
    // 10→20→40→60→30→50→(back to 40)
    let values = vec![10, 20, 40, 60, 30, 50];
    let head = create_list_with_loop(values.clone(), Some(2));
    
    println!("Created list with loop:");
    println!("Values: {:?}", values);
    println!("Loop starts at index: 2 (value 40)");
    println!("Has loop: {}", loop_exists(&head));
    
    // TODO: After implementing detect_and_remove_loop:
    // detect_and_remove_loop(&head);
    // println!("After removal - Has loop: {}", loop_exists(&head));
    // println!("List: {:?}", to_vec(&head));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n6_loop_at_2() {
        // 10→20→40→60→30→50→(40)
        // tail_len=2, loop_len=4
        let head = create_list_with_loop(vec![10, 20, 40, 60, 30, 50], Some(2));
        assert!(loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert!(!loop_exists(&head));
        assert_eq!(to_vec(&head), vec![10, 20, 40, 60, 30, 50]);
    }

    #[test]
    fn test_n5_loop_at_1() {
        // 1→2→3→4→5→(2)
        // tail_len=1, loop_len=4
        let head = create_list_with_loop(vec![1, 2, 3, 4, 5], Some(1));
        assert!(loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert!(!loop_exists(&head));
        assert_eq!(to_vec(&head), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_n3_full_loop() {
        // 7→8→9→(7)
        // tail_len=0, loop_len=3
        let head = create_list_with_loop(vec![7, 8, 9], Some(0));
        assert!(loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert!(!loop_exists(&head));
        assert_eq!(to_vec(&head), vec![7, 8, 9]);
    }

    #[test]
    fn test_n1_self_loop() {
        // 5→(5)
        // tail_len=0, loop_len=1
        let head = create_list_with_loop(vec![5], Some(0));
        assert!(loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert!(!loop_exists(&head));
        assert_eq!(to_vec(&head), vec![5]);
    }

    #[test]
    fn test_n2_full_loop() {
        // A→B→(A)
        // tail_len=0, loop_len=2
        let head = create_list_with_loop(vec![100, 200], Some(0));
        assert!(loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert!(!loop_exists(&head));
        assert_eq!(to_vec(&head), vec![100, 200]);
    }

    #[test]
    fn test_n2_tail_plus_self_loop() {
        // A→B→(B)
        // tail_len=1, loop_len=1
        let head = create_list_with_loop(vec![10, 20], Some(1));
        assert!(loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert!(!loop_exists(&head));
        assert_eq!(to_vec(&head), vec![10, 20]);
    }

    #[test]
    fn test_n7_loop_at_3() {
        // 1→2→3→4→5→6→7→(4)
        // tail_len=3, loop_len=4
        let head = create_list_with_loop(vec![1, 2, 3, 4, 5, 6, 7], Some(3));
        assert!(loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert!(!loop_exists(&head));
        assert_eq!(to_vec(&head), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_n10_balanced() {
        // tail_len=5, loop_len=5
        let head = create_list_with_loop(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], Some(5));
        assert!(loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert!(!loop_exists(&head));
        assert_eq!(to_vec(&head), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_no_loop() {
        // No loop case
        let head = create_list_with_loop(vec![1, 2, 3, 4, 5], None);
        assert!(!loop_exists(&head));
        
        // Should handle gracefully (no-op)
        detect_and_remove_loop(&head);
        
        assert!(!loop_exists(&head));
        assert_eq!(to_vec(&head), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_empty_list() {
        let head = create_list_with_loop(vec![], None);
        assert!(!loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert_eq!(to_vec(&head), vec![]);
    }

    #[test]
    fn test_n1_no_loop() {
        let head = create_list_with_loop(vec![42], None);
        assert!(!loop_exists(&head));
        
        detect_and_remove_loop(&head);
        
        assert_eq!(to_vec(&head), vec![42]);
    }
}
