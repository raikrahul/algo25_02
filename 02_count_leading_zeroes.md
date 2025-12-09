02 Count Leading Zeroes

PROBLEM
Given array with contiguous zeroes at start followed by non-zero integers,
return count of zeroes.

Input:  [0, 0, 0, 0, 0, 3, 2, 8, 11, 10, 15, 22]
Output: 5

FIGURE 1: Data Pattern
Array contains two zones. Transition point = answer.

index:   0   1   2   3   4   5   6   7   8   9  10  11
       +---+---+---+---+---+---+---+---+---+---+---+---+
value: | 0 | 0 | 0 | 0 | 0 | 3 | 2 | 8 |11 |10 |15 |22 |
       +---+---+---+---+---+---+---+---+---+---+---+---+
zone:  [====ZERO ZONE=====][======NON-ZERO ZONE=======]
                           ^
                           Answer = 5 (first non-zero index)

FIGURE 2: Binary Search Decision Table

arr[mid] == 0:  Mid is in ZERO zone. Boundary is AFTER mid.
                Action: low = mid + 1 (EXCLUDE mid)

arr[mid] != 0:  Mid is in NON-ZERO zone. Boundary is AT or BEFORE mid.
                Action: high = mid (INCLUDE mid, it might be the answer)

FIGURE 3: Why low = mid + 1

arr = [0, 0, 3, 5, 7], mid = 1, arr[1] = 0

index:   0   1   2   3   4
       +---+---+---+---+---+
value: | 0 | 0 | 3 | 5 | 7 |
       +---+---+---+---+---+
             ^
           mid=1, value=0

Index 1 is zero. Index 1 is NOT the answer.
The answer is strictly AFTER index 1.
Exclude index 1: low = mid + 1 = 2.

If low = mid (wrong): low stays at 1, infinite loop.

FIGURE 4: Why high = mid (not mid - 1)

arr = [0, 0, 3, 5, 7], mid = 2, arr[2] = 3

index:   0   1   2   3   4
       +---+---+---+---+---+
value: | 0 | 0 | 3 | 5 | 7 |
       +---+---+---+---+---+
             ^
           mid=2, value=3

Index 2 is non-zero. Is it the FIRST non-zero? Unknown.
Index 2 might be the answer. Must include it.
Include index 2: high = mid = 2.

If high = mid - 1 (wrong): high = 1, skip index 2, lose answer.

FIGURE 5: Trace for arr = [0, 0, 3, 5, 7]

Step 1: low=0, high=5, mid=2, arr[2]=3!=0, high=2
Step 2: low=0, high=2, mid=1, arr[1]=0, low=2
Step 3: low=2, high=2, stop
Answer: low = 2. Correct.

FIGURE 6: Trace for arr = [0, 0, 0, 0] (all zeroes)

Step 1: low=0, high=4, mid=2, arr[2]=0, low=3
Step 2: low=3, high=4, mid=3, arr[3]=0, low=4
Step 3: low=4, high=4, stop
Answer: low = 4 = n. Correct.

FIGURE 7: Trace for arr = [7, 5, 8, 3, 1] (no zeroes)

Step 1: low=0, high=5, mid=2, arr[2]=8!=0, high=2
Step 2: low=0, high=2, mid=1, arr[1]=5!=0, high=1
Step 3: low=0, high=1, mid=0, arr[0]=7!=0, high=0
Step 4: low=0, high=0, stop
Answer: low = 0. Correct.

IMPLEMENTATION

fn count_zeroes(arr: &[i32]) -> usize {
    let n = arr.len();
    if n == 0 { return 0; }

    let (mut low, mut high) = (0, n);

    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] == 0 {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    low
}
