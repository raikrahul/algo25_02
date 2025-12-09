04 Find Any Duplicate Number

PROBLEM
Given array of N integers where each element is in [1, N-1],
find any duplicated value. Array may be mutated.
Constraint: O(N) time, O(1) space.

Input:  [3, 1, 3, 4, 2, 6, 5]  N=7
Output: 3

FIGURE 1: Value-Index Mapping
Each value V can serve as an index into the array.

Index:   0     1     2     3     4     5     6
Value:   3     1     3     4     2     6     5
         |     |     |     |     |     |     |
         v     v     v     v     v     v     v
Target:  3     1     3     4     2     6     5

Index 3 is targeted by both a[0]=3 and a[2]=3.
Collision indicates duplicate value 3.

FIGURE 2: Negation Marking Trace

Initial: [3, 1, 3, 4, 2, 6, 5]

i=0: a[0]=3, target=3, a[3]=4 (positive), mark: a[3]=-4
     [3, 1, 3, -4, 2, 6, 5]

i=1: a[1]=1, target=1, a[1]=1 (positive), mark: a[1]=-1
     [3, -1, 3, -4, 2, 6, 5]

i=2: a[2]=3, target=3, a[3]=-4 (NEGATIVE)
     Duplicate found: 3

FIGURE 3: Absolute Value Requirement

At i=3 in array [3, -1, 3, -4, 2, 6, 5]:
  a[3] = -4 (marked negative earlier)
  target = |-4| = 4 (not -4, which would crash)
  a[4] = 2 (positive), mark: a[4]=-2

Always use |a[i]| when computing target index.

FIGURE 4: Check-Before-Mark Order

Wrong order:
  a[target] = -a[target]  // mark first
  if a[target] < 0        // always true now

Correct order:
  if a[target] < 0        // check first
      return target       // duplicate
  a[target] = -a[target]  // mark second

FIGURE 5: Return Value Selection

At detection:
  i = 2 (current loop index)
  a[i] = 3 (value at current position)
  target = 3 (index we checked)

Return: |a[i]| = 3 (the duplicated value)
Do not return i or target directly.

IMPLEMENTATION

fn find_duplicate(a: &mut [i32]) -> i32 {
    for i in 0..a.len() {
        let val = a[i].abs() as usize;
        if a[val] < 0 {
            return val as i32;
        }
        a[val] = -a[val];
    }
    panic!("No duplicate found")
}
