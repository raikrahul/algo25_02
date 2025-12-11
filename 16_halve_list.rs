// 16: Split and Combine
// Constraint: Single pass, O(1) space
// Odd N → extra to first half

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    // val: i32
    // Example: 2, 4, 5, 3, 8, 7, 6, 1, 9
    // Memory: 0x100, 0x108, 0x110, ...
    pub val: i32,
    
    // next: Option<Box<Node>>
    // Example: 0x100.next = 0x108, 0x140.next = null
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

// halve_list: head → new_head
// 
// Input Drawing (N=9):
//   0x100(2)→0x108(4)→0x110(5)→0x118(3)→0x120(8)→0x128(7)→0x130(6)→0x138(1)→0x140(9)→null
//
// Calculation:
//   N = 9
//   9 / 2 = 4.5
//   ceil(4.5) = 5 → first half
//   floor(4.5) = 4 → second half
//   split_after_idx = 4 → 0x120
//
// Constraint Check:
//   pass_1: count → 9 steps
//   pass_2: traverse to 4 → 5 steps
//   total = 2 passes ✗
//   allowed = 1 pass
//   ∴ need different approach
//
// Speed Ratio:
//   pos_slow = t × 1
//   pos_fast = t × 2
//   when fast = 8 → t = 4 → slow = 4 ✓
//
// Stop Condition:
//   odd N=9: fast.next = null → stop
//   even N=8: fast.next.next = null → stop
//   combined: while fast.next ≠ null AND fast.next.next ≠ null
//
// Surgery (N=9):
//   step 1: second_head = slow.next = 0x128
//   step 2: slow.next = null (break at 0x120)
//   step 3: find tail → 0x140
//   step 4: tail.next = head = 0x100
//   step 5: return 0x128
//
// Edge N=1:
//   loop skips (fast.next = null)
//   second_head = null → return head unchanged
//
// Edge N=2:
//   fast.next.next = null → loop skips
//   slow = head → second_head = head.next
//   surgery → swap
//
pub fn halve_list(head: Option<Box<Node>>) -> Option<Box<Node>> {
    // md line 79-81: N=0 → return ∅
    if head.is_none() {
        return None;
    }
    
    let mut head = head.unwrap();
    
    // md line 70-73: N=1 → no swap needed
    if head.next.is_none() {
        return Some(head);
    }
    
    // md line 30: slow=0x100 fast=0x100
    let mut slow: *mut Node = &mut *head;
    let mut fast: *mut Node = &mut *head;
    
    // md line 56: while fast.next≠∅ ∧ fast.next.next≠∅
    // md lines 31-35: slow +1, fast +2
    unsafe {
        while (*fast).next.is_some() 
            && (*fast).next.as_ref().unwrap().next.is_some() 
        {
            // slow moves 1 step: md line 31
            slow = (*slow).next.as_mut().unwrap().as_mut();
            // fast moves 2 steps: md line 31
            let step1 = (*fast).next.as_mut().unwrap().as_mut();
            fast = step1.next.as_mut().unwrap().as_mut();
        }
        
        // md line 58: second_head = slow.next
        let second_head = (*slow).next.take(); // take() sets slow.next = None (md line 59)
        
        // md line 72-73: if second empty, return head
        if second_head.is_none() {
            return Some(head);
        }
        
        let mut second_head = second_head.unwrap();
        
        // md lines 62-66: find tail of second half
        let mut tail: *mut Node = &mut *second_head;
        while (*tail).next.is_some() {
            tail = (*tail).next.as_mut().unwrap().as_mut();
        }
        
        // md line 67: tail.next = head (connect)
        (*tail).next = Some(head);
        
        // md line 69: return second_head
        Some(second_head)
    }
}

// Helper: Vec → List
// [2,4,5] → 0x100(2)→0x108(4)→0x110(5)→null
fn to_list(v: Vec<i32>) -> Option<Box<Node>> {
    let mut head = None;
    for &val in v.iter().rev() {
        let mut node = Node::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

// Helper: List → Vec
// 0x100(2)→0x108(4)→null → [2,4]
fn to_vec(mut head: Option<Box<Node>>) -> Vec<i32> {
    let mut v = Vec::new();
    while let Some(node) = head {
        v.push(node.val);
        head = node.next;
    }
    v
}

fn main() {
    // Example run
    let input = vec![2, 4, 5, 3, 8, 7, 6, 1, 9];
    let head = to_list(input.clone());
    let result = halve_list(head);
    println!("Input:  {:?}", input);
    println!("Output: {:?}", to_vec(result));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n9_odd() {
        // N=9, split_after_idx=4
        // first=[2,4,5,3,8] len=5
        // second=[7,6,1,9] len=4
        // output=[7,6,1,9,2,4,5,3,8]
        let input = vec![2, 4, 5, 3, 8, 7, 6, 1, 9];
        let expected = vec![7, 6, 1, 9, 2, 4, 5, 3, 8];
        let result = to_vec(halve_list(to_list(input)));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_n8_even() {
        // N=8, split_after_idx=3
        // first=[1,3,5,7] len=4
        // second=[2,4,6,8] len=4
        // output=[2,4,6,8,1,3,5,7]
        let input = vec![1, 3, 5, 7, 2, 4, 6, 8];
        let expected = vec![2, 4, 6, 8, 1, 3, 5, 7];
        let result = to_vec(halve_list(to_list(input)));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_n1() {
        // N=1, no change
        // second_head = null → return head
        let input = vec![5];
        let expected = vec![5];
        let result = to_vec(halve_list(to_list(input)));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_n2() {
        // N=2, split_after_idx=0
        // first=[10] second=[20]
        // output=[20,10]
        let input = vec![10, 20];
        let expected = vec![20, 10];
        let result = to_vec(halve_list(to_list(input)));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_n3() {
        // N=3, ceil(3/2)=2
        // first=[1,2] second=[3]
        // output=[3,1,2]
        let input = vec![1, 2, 3];
        let expected = vec![3, 1, 2];
        let result = to_vec(halve_list(to_list(input)));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_n0() {
        // N=0, empty → empty
        let input: Vec<i32> = vec![];
        let expected: Vec<i32> = vec![];
        let result = to_vec(halve_list(to_list(input)));
        assert_eq!(result, expected);
    }
}
