01 Array Zero Prefix Counting

PROBLEM
Count contiguous zeroes at the start of an array.
Input:  [0, 0, 0, 3, 2, 8]
Output: 3

FIGURE 1: Data Transformation
Convert values to boolean for binary search.

Index:   0    1    2    3    4    5
Value: [ 0  | 0  | 0  | 3  | 2  | 8  ]
Bool:  [ T  | T  | T  | F  | F  | F  ]
                       ^
                       First FALSE = answer index

FIGURE 2: Infinite Loop Trap
Observe what happens with bad pointer update.

Input: [0, 0]
Target: 2

Iteration 1:
  L=0, R=1, Mid=(0+1)/2=0
  Arr[0]=0, set L=Mid (WRONG: L stays 0)

Iteration 2:
  L=0, R=1, Mid=0
  Same state. Infinite loop.

Fix: Use L=Mid+1 when Arr[Mid]=0.

FIGURE 3: Hybrid Strategy Trace
Input: [0, 0, 0, 50]
Target: 3

Phase 1 (Binary Search):
  L=0, R=3. |R-L|=3>1. Continue.
  Mid=1. Arr[1]=0. L=Mid+1=2.
  L=2, R=3. |R-L|=1. Stop.
  Result: L=2 (guaranteed count so far).

Phase 2 (Linear Scan):
  Scan [2,3].
  i=2: Arr[2]=0. Count=1.
  i=3: Arr[3]=50. Stop.

Total = L + Count = 2 + 1 = 3. Correct.

IMPLEMENTATION

fn count_zeroes(arr: &[i32]) -> i32 {
    let n = arr.len() as i32;
    if n == 0 { return 0; }

    let (mut l, mut r) = (0, n - 1);

    // Binary search: reduce to <=2 elements
    while (r - l).abs() > 1 {
        let mid = l + (r - l) / 2;
        if arr[mid as usize] == 0 {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    // Linear scan: count remaining zeroes
    let mut count = 0;
    for i in l..=r {
        if arr[i as usize] == 0 {
            count += 1;
        } else {
            break;
        }
    }

    l + count
}
