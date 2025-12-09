06 Merge Arrays

PROBLEM
Merge sorted array a2 (N elements) into sorted array a1 (M elements + N empty slots).
Result: a1 contains all M+N elements in sorted order.

Input:  a1=[2,5,9,_,_,_] (M=3), a2=[1,6,8] (N=3)
Output: a1=[1,2,5,6,8,9]

FIGURE 1: Initial Memory Layout

a1: [2, 5, 9, ?, ?, ?]
     0  1  2  3  4  5
     [--sorted--][empty]

a2: [1, 6, 8]
     0  1  2

Gap slots 3-5 in a1 reserved for merged result.

FIGURE 2: Why Merge from Back to Front

Forward merge (wrong):
  Compare a1[0]=2 vs a2[0]=1. Put smaller (1) at a1[0].
  Problem: Original a1[0]=2 is overwritten. Data lost.

Backward merge (correct):
  Compare a1[2]=9 vs a2[2]=8. Put larger (9) at a1[5].
  a1[2] still contains 9, can reference it later.
  No overwriting of unprocessed data.

FIGURE 3: Merge Trace

Initial: a1=[2,5,9,?,?,?], a2=[1,6,8]
         ptr_a1=2, ptr_a2=2, ptr_write=5

Step 1: a1[2]=9 > a2[2]=8. Put 9 at a1[5]. ptr_a1=1.
        a1=[2,5,9,?,?,9]

Step 2: a1[1]=5 < a2[2]=8. Put 8 at a1[4]. ptr_a2=1.
        a1=[2,5,9,?,8,9]

Step 3: a1[1]=5 < a2[1]=6. Put 6 at a1[3]. ptr_a2=0.
        a1=[2,5,9,6,8,9]

Step 4: a1[1]=5 > a2[0]=1. Put 5 at a1[2]. ptr_a1=0.
        a1=[2,5,5,6,8,9]

Step 5: a1[0]=2 > a2[0]=1. Put 2 at a1[1]. ptr_a1=None.
        a1=[2,2,5,6,8,9]

Step 6: a1 exhausted. Copy remaining a2. Put 1 at a1[0].
        a1=[1,2,5,6,8,9]

FIGURE 4: Why No Leftover Loop for a1

When a2 exhausts first:
  a1 still has elements at indices 0..ptr_a1.
  These elements are ALREADY in a1.
  No copy needed. Already in correct position.

When a1 exhausts first:
  a2 still has elements at indices 0..ptr_a2.
  These elements are in SEPARATE array a2.
  Must copy them into remaining slots of a1.

FIGURE 5: Pointer Exhaustion

a1=[7,8,9,?,?,?], a2=[1,2,3]  (all a1 > all a2)

Steps: 9→a1[5], 8→a1[4], 7→a1[3]. ptr_a1=None.
       a1=[7,8,9,7,8,9], a2=[1,2,3] untouched.

Leftover: copy a2[2]→a1[2], a2[1]→a1[1], a2[0]→a1[0].
          a1=[1,2,3,7,8,9]

IMPLEMENTATION

fn merge(a1: &mut Vec<i32>, m: usize, a2: &[i32], n: usize) {
    let mut ptr_a1 = if m > 0 { Some(m - 1) } else { None };
    let mut ptr_a2 = if n > 0 { Some(n - 1) } else { None };
    let mut ptr_write = m + n - 1;

    while let (Some(i), Some(j)) = (ptr_a1, ptr_a2) {
        if a1[i] > a2[j] {
            a1[ptr_write] = a1[i];
            ptr_a1 = i.checked_sub(1);
        } else {
            a1[ptr_write] = a2[j];
            ptr_a2 = j.checked_sub(1);
        }
        ptr_write = ptr_write.saturating_sub(1);
    }

    while let Some(j) = ptr_a2 {
        a1[ptr_write] = a2[j];
        ptr_a2 = j.checked_sub(1);
        ptr_write = ptr_write.saturating_sub(1);
    }
}
