// ============================================================================
// PROBLEM: Count Leading Zeroes
// ============================================================================
// WHAT: Given array with contiguous 0s at start, followed by non-zero integers
// RETURN: The count of leading zeroes
// WHY BINARY SEARCH: Array has TWO ZONES - [ZERO-ZONE][NON-ZERO-ZONE] - sorted by "is zero"
// ============================================================================

// ============================================================================
// NUMERICAL EXAMPLE 1 (MIDDLE-SCALE, n=12)
// ============================================================================
// arr = [0, 0, 0, 0, 0, 3, 2, 8, 11, 10, 15, 22]
// n = 12 (this is arr.len() which is 12 because we have indices 0,1,2,3,4,5,6,7,8,9,10,11)
// expected answer = 5 (because indices 0,1,2,3,4 have value 0, that's 5 zeroes)
//
// BINARY SEARCH TRACE:
// ┌─────────────────────────────────────────────────────────────────────────┐
// │ ITERATION 1                                                             │
// │ low = 0, high = 12 (using high = n, NOT n-1, because answer could be n) │
// │ mid = 0 + (12 - 0) / 2 = 0 + 12/2 = 0 + 6 = 6                           │
// │                                                                         │
// │ index:   0   1   2   3   4   5   6   7   8   9  10  11                  │
// │        +---+---+---+---+---+---+---+---+---+---+---+---+                 │
// │ value: | 0 | 0 | 0 | 0 | 0 | 3 | 2 | 8 |11 |10 |15 |22 |                │
// │        +---+---+---+---+---+---+---+---+---+---+---+---+                 │
// │          ↑                       ↑                       ↑(imaginary)   │
// │         low=0                  mid=6                  high=12           │
// │                                                                         │
// │ arr[mid] = arr[6] = 2                                                   │
// │ QUESTION: Is 2 == 0? NO                                                 │
// │ MEANING: First non-zero is AT index 6 or BEFORE index 6                 │
// │ ACTION: Shrink search to LEFT half, high = mid = 6                      │
// │ RESULT: low = 0, high = 6                                               │
// └─────────────────────────────────────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────────────────┐
// │ ITERATION 2                                                             │
// │ low = 0, high = 6                                                       │
// │ mid = 0 + (6 - 0) / 2 = 0 + 6/2 = 0 + 3 = 3                             │
// │                                                                         │
// │ index:   0   1   2   3   4   5                                          │
// │        +---+---+---+---+---+---+                                        │
// │ value: | 0 | 0 | 0 | 0 | 0 | 3 |                                        │
// │        +---+---+---+---+---+---+                                        │
// │          ↑           ↑           ↑                                      │
// │         low=0      mid=3      high=6                                    │
// │                                                                         │
// │ arr[mid] = arr[3] = 0                                                   │
// │ QUESTION: Is 0 == 0? YES                                                │
// │ MEANING: First non-zero is AFTER index 3 (strictly greater than 3)     │
// │ ACTION: Shrink search to RIGHT half, low = mid + 1 = 3 + 1 = 4          │
// │ RESULT: low = 4, high = 6                                               │
// └─────────────────────────────────────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────────────────┐
// │ ITERATION 3                                                             │
// │ low = 4, high = 6                                                       │
// │ mid = 4 + (6 - 4) / 2 = 4 + 2/2 = 4 + 1 = 5                             │
// │                                                                         │
// │ index:   4   5                                                          │
// │        +---+---+                                                        │
// │ value: | 0 | 3 |                                                        │
// │        +---+---+                                                        │
// │          ↑   ↑   ↑                                                      │
// │        low=4 mid=5 high=6                                               │
// │                                                                         │
// │ arr[mid] = arr[5] = 3                                                   │
// │ QUESTION: Is 3 == 0? NO                                                 │
// │ MEANING: First non-zero is AT index 5 or BEFORE index 5                 │
// │ ACTION: high = mid = 5                                                  │
// │ RESULT: low = 4, high = 5                                               │
// └─────────────────────────────────────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────────────────┐
// │ ITERATION 4                                                             │
// │ low = 4, high = 5                                                       │
// │ mid = 4 + (5 - 4) / 2 = 4 + 1/2 = 4 + 0 = 4 (integer division!)        │
// │                                                                         │
// │ index:   4                                                              │
// │        +---+                                                            │
// │ value: | 0 |                                                            │
// │        +---+                                                            │
// │          ↑                                                              │
// │      low=mid=4  high=5                                                  │
// │                                                                         │
// │ arr[mid] = arr[4] = 0                                                   │
// │ QUESTION: Is 0 == 0? YES                                                │
// │ MEANING: First non-zero is AFTER index 4                                │
// │ ACTION: low = mid + 1 = 4 + 1 = 5                                       │
// │ RESULT: low = 5, high = 5                                               │
// └─────────────────────────────────────────────────────────────────────────┘
//
// ┌─────────────────────────────────────────────────────────────────────────┐
// │ TERMINATION CHECK                                                       │
// │ low = 5, high = 5                                                       │
// │ CONDITION: low < high? 5 < 5? NO (5 is not less than 5)                │
// │ STOP LOOP                                                               │
// │ RETURN low = 5                                                          │
// │ VERIFICATION: arr[0..5] = [0,0,0,0,0] = 5 zeroes ✓                      │
// └─────────────────────────────────────────────────────────────────────────┘

// ============================================================================
// NUMERICAL EXAMPLE 2 (LARGE-SCALE, n=1000000)
// ============================================================================
// arr = [0, 0, 0, ...(999997 more zeroes)..., 7, 8, 9]
// n = 1000000
// expected answer = 999997
//
// HOW MANY ITERATIONS?
// log₂(1000000) = log₂(10⁶) = 6 × log₂(10) = 6 × 3.322 = 19.93 ≈ 20 iterations
// BRUTE FORCE would need 999998 iterations (scan until first non-zero)
// BINARY SEARCH saves 999998 - 20 = 999978 iterations
//
// ITERATION TRACE (showing only key steps):
// iter 1: low=0, high=1000000, mid=500000, arr[500000]=0 → low=500001
// iter 2: low=500001, high=1000000, mid=750000, arr[750000]=0 → low=750001
// iter 3: low=750001, high=1000000, mid=875000, arr[875000]=0 → low=875001
// ... continues halving ...
// iter 20: low=999997, high=999997, STOP, answer=999997 ✓

// ============================================================================
// NUMERICAL EXAMPLE 3 (SMALL-SCALE, n=1)
// ============================================================================
// CASE 3A: arr = [0], n = 1, expected = 1
// low=0, high=1, mid=0+(1-0)/2=0, arr[0]=0, YES==0, low=0+1=1
// low=1, high=1, STOP, return 1 ✓
//
// CASE 3B: arr = [5], n = 1, expected = 0
// low=0, high=1, mid=0+(1-0)/2=0, arr[0]=5, NO!=0, high=0
// low=0, high=0, STOP, return 0 ✓

// ============================================================================
// NUMERICAL EXAMPLE 4 (EDGE-CASE: ALL ZEROES, n=4)
// ============================================================================
// arr = [0, 0, 0, 0], n = 4, expected = 4
//
// iter 1: low=0, high=4, mid=0+(4-0)/2=2, arr[2]=0, low=2+1=3
// iter 2: low=3, high=4, mid=3+(4-3)/2=3, arr[3]=0, low=3+1=4
// iter 3: low=4, high=4, 4<4? NO, STOP
// return low = 4 ✓
//
// CRITICAL: If we used high=n-1=3, we would have:
// iter 1: low=0, high=3, mid=1, arr[1]=0, low=2
// iter 2: low=2, high=3, mid=2, arr[2]=0, low=3
// iter 3: low=3, high=3, STOP, return 3 ✗ WRONG! Should be 4.
// WHY WRONG? Because answer CAN BE n when all elements are zero.
// high=n allows low to reach n. high=n-1 caps answer at n-1.

// ============================================================================
// NUMERICAL EXAMPLE 5 (EDGE-CASE: NO ZEROES, n=5)
// ============================================================================
// arr = [7, 5, 8, 3, 1], n = 5, expected = 0
//
// iter 1: low=0, high=5, mid=2, arr[2]=8, 8!=0, high=2
// iter 2: low=0, high=2, mid=1, arr[1]=5, 5!=0, high=1
// iter 3: low=0, high=1, mid=0, arr[0]=7, 7!=0, high=0
// iter 4: low=0, high=0, 0<0? NO, STOP
// return low = 0 ✓

// ============================================================================
// NUMERICAL EXAMPLE 6 (EDGE-CASE: EMPTY ARRAY, n=0)
// ============================================================================
// arr = [], n = 0, expected = 0
//
// iter 1: low=0, high=0, 0<0? NO, SKIP LOOP ENTIRELY
// return low = 0 ✓

// ============================================================================
// NUMERICAL EXAMPLE 7 (FRACTIONAL MID CALCULATION, n=7)
// ============================================================================
// arr = [0, 0, 0, 0, 5, 6, 7], n = 7, expected = 4
//
// iter 1: low=0, high=7, mid=0+(7-0)/2=0+7/2=0+3=3 (7/2=3.5→3 integer)
//         arr[3]=0, low=4
// iter 2: low=4, high=7, mid=4+(7-4)/2=4+3/2=4+1=5 (3/2=1.5→1 integer)
//         arr[5]=6, 6!=0, high=5
// iter 3: low=4, high=5, mid=4+(5-4)/2=4+1/2=4+0=4 (1/2=0.5→0 integer)
//         arr[4]=5, 5!=0, high=4
// iter 4: low=4, high=4, 4<4? NO, STOP
// return 4 ✓
//
// SURPRISE: 7/2=3 not 4, 3/2=1 not 2, 1/2=0 not 1
// INTEGER DIVISION ALWAYS ROUNDS DOWN (truncates toward zero)

/// Counts the number of leading zeroes in an array.
///
/// # Arguments
/// * `arr` - A slice of i32 where all zeroes are at the start, followed by non-zero values
///           Example: [0,0,0,5,7,2] has 3 leading zeroes
///           Example: [0,0,0,0] has 4 leading zeroes (all of them)
///           Example: [5,7,2] has 0 leading zeroes
///           Example: [] has 0 leading zeroes (empty array)
///
/// # Returns
/// * `usize` - The count of leading zeroes, which is a number from 0 to arr.len() inclusive
///             If arr.len()=12 and answer=5, that means arr[0..5] are all zeroes
///             If answer=arr.len(), ALL elements are zeroes
///             If answer=0, FIRST element is non-zero (or array is empty)
///
/// # Binary Search Variables
/// * `low` - usize, starts at 0, will be 0 to n
///           MEANING: "all indices < low are definitely zeroes"
///           Example: if low=3, then arr[0], arr[1], arr[2] are all 0
///
/// * `high` - usize, starts at n (arr.len()), will be 0 to n
///            MEANING: "all indices >= high are definitely non-zero"
///            Example: if high=5, then arr[5], arr[6], ... are all non-zero
///
/// * `mid` - usize, calculated as low + (high - low) / 2
///           WHY THIS FORMULA? Avoids overflow. (low+high)/2 can overflow if low+high > usize::MAX
///           Example: low=4, high=7, mid=4+(7-4)/2=4+3/2=4+1=5
///           Example: low=4, high=5, mid=4+(5-4)/2=4+1/2=4+0=4 (integer division!)
///
/// # Decision Logic
/// * IF arr[mid] == 0:
///     First non-zero is AFTER mid (at index > mid)
///     So all indices <= mid are zeroes
///     So low = mid + 1 (exclude mid from future search, we know it's zero)
///
/// * IF arr[mid] != 0:
///     First non-zero is AT mid or BEFORE mid
///     So high = mid (include mid as potential answer)
///     WHY NOT high = mid - 1? Because mid itself might be the first non-zero!
pub fn count_zeroes(arr: &[i32]) -> usize {
    // ┌─────────────────────────────────────────────────────────────────────────────────────────┐
    // │ THE ASYMMETRY: WHY low = mid + 1 BUT high = mid (NOT mid - 1)?                         │
    // │                                                                                         │
    // │ EXAMPLE: arr = [0, 0, 3, 5, 7], n = 5, answer = 2                                       │
    // │                                                                                         │
    // │ CASE A: arr[mid] == 0 (mid is in ZERO zone)                                            │
    // │   mid = 1, arr[1] = 0                                                                   │
    // │   We KNOW index 1 is a ZERO. Index 1 is DEFINITELY NOT the answer.                     │
    // │   The answer is STRICTLY > 1.                                                           │
    // │   So we EXCLUDE mid from future search: low = mid + 1 = 1 + 1 = 2                      │
    // │                                                                                         │
    // │   If we did low = mid = 1 instead:                                                     │
    // │     low=1, high=2, mid=1+(2-1)/2=1, arr[1]=0, low=mid=1 → INFINITE LOOP!               │
    // │                                                                                         │
    // │ CASE B: arr[mid] != 0 (mid is in NON-ZERO zone)                                        │
    // │   mid = 2, arr[2] = 3                                                                   │
    // │   We KNOW index 2 is NON-ZERO. But is it THE FIRST non-zero? MAYBE!                    │
    // │   Index 2 MIGHT be the answer. We CANNOT exclude it.                                   │
    // │   So we INCLUDE mid in future search: high = mid = 2                                   │
    // │                                                                                         │
    // │   If we did high = mid - 1 = 2 - 1 = 1 instead:                                        │
    // │     New range = [0, 1), we search indices {0} only                                     │
    // │     We SKIP index 2 FOREVER                                                            │
    // │     But index 2 IS THE ANSWER!                                                         │
    // │     WRONG ANSWER = 1, CORRECT ANSWER = 2                                               │
    // │                                                                                         │
    // │ PROOF with arr = [0, 0, 3, 5, 7]:                                                      │
    // │   CORRECT (high = mid):                                                                │
    // │     Step 1: low=0, high=5, mid=2, arr[2]=3≠0, high=mid=2                               │
    // │     Step 2: low=0, high=2, mid=1, arr[1]=0, low=mid+1=2                                │
    // │     Step 3: low=2, high=2, 2<2? NO → STOP, return 2 ✓                                  │
    // │                                                                                         │
    // │   WRONG (high = mid - 1):                                                              │
    // │     Step 1: low=0, high=5, mid=2, arr[2]=3≠0, high=mid-1=1                             │
    // │     Step 2: low=0, high=1, mid=0, arr[0]=0, low=mid+1=1                                │
    // │     Step 3: low=1, high=1, 1<1? NO → STOP, return 1 ✗ WRONG!                           │
    // │                                                                                         │
    // │ SUMMARY TABLE:                                                                          │
    // │ ┌─────────────────┬──────────────────────────┬────────────────────────────────────────┐ │
    // │ │ arr[mid] value  │ What we KNOW             │ Action                                 │ │
    // │ ├─────────────────┼──────────────────────────┼────────────────────────────────────────┤ │
    // │ │ arr[mid] == 0   │ mid is NOT the answer    │ EXCLUDE it: low = mid + 1             │ │
    // │ │ arr[mid] != 0   │ mid MIGHT be the answer  │ INCLUDE it: high = mid                │ │
    // │ └─────────────────┴──────────────────────────┴────────────────────────────────────────┘ │
    // └─────────────────────────────────────────────────────────────────────────────────────────┘

    let n: usize = arr.len();
    // n = arr.len() → for arr = [0,0,0,0,0,3,2,8,11,10,15,22], n = 12
    //                  for arr = [0,0,3,5,7], n = 5
    //                  for arr = [], n = 0
    //                  for arr = [0], n = 1
    //                  for arr = [5], n = 1

    let mut low: usize = 0;
    // low starts at 0 because the answer could be as small as 0 (no zeroes at all)
    // Example: arr = [7,5,8,3,1] → answer = 0 (first element is already non-zero)

    let mut high: usize = n;
    // high = n (NOT n-1) because the answer could be n (all elements are zero)
    // Example: arr = [0,0,0,0] → answer = 4 = n (no non-zero element exists)
    // 
    // CRITICAL CALCULATION: What if we used high = n - 1?
    //   arr = [0,0,0,0], n = 4, high = n-1 = 3
    //   Step 1: low=0, high=3, mid=1, arr[1]=0, low=2
    //   Step 2: low=2, high=3, mid=2, arr[2]=0, low=3
    //   Step 3: low=3, high=3, STOP, return 3 ✗ WRONG! Should be 4.
    //   
    //   With high = n = 4:
    //   Step 1: low=0, high=4, mid=2, arr[2]=0, low=3
    //   Step 2: low=3, high=4, mid=3, arr[3]=0, low=4
    //   Step 3: low=4, high=4, STOP, return 4 ✓ CORRECT!

    while low < high {
        // LOOP INVARIANT: answer is in range [low, high)
        // When low == high, range [low, high) = [low, low) = empty, so STOP
        // 
        // TERMINATION PROOF:
        //   Each iteration either: low increases by at least 1 (low = mid + 1)
        //                     or: high decreases to mid (high = mid, and mid < high because mid = low + (high-low)/2)
        //   Since (high - low) shrinks each iteration, loop terminates.
        //
        // EDGE CASE: empty array, n = 0
        //   low = 0, high = 0, 0 < 0? NO → skip loop entirely, return 0 ✓

        let mid: usize = low + (high - low) / 2;
        // WHY THIS FORMULA instead of (low + high) / 2?
        //   OVERFLOW PROTECTION: if low = 2_000_000_000 and high = 2_000_000_000
        //     (low + high) = 4_000_000_000 > u32::MAX = 4_294_967_295 → OVERFLOW on 32-bit
        //     low + (high - low)/2 = 2_000_000_000 + 0/2 = 2_000_000_000 ✓ NO OVERFLOW
        //
        // CALCULATION EXAMPLES:
        //   low=0, high=12: mid = 0 + (12-0)/2 = 0 + 6 = 6
        //   low=0, high=5:  mid = 0 + (5-0)/2 = 0 + 2 = 2  (5/2=2.5→2 integer division)
        //   low=4, high=6:  mid = 4 + (6-4)/2 = 4 + 1 = 5
        //   low=4, high=5:  mid = 4 + (5-4)/2 = 4 + 0 = 4  (1/2=0.5→0 integer division)
        //   low=0, high=1:  mid = 0 + (1-0)/2 = 0 + 0 = 0  (1/2=0.5→0 integer division)
        //
        // INTEGER DIVISION ALWAYS ROUNDS DOWN (truncates toward zero):
        //   7/2 = 3 (not 4), 5/2 = 2 (not 3), 3/2 = 1 (not 2), 1/2 = 0 (not 1)

        if arr[mid] == 0 {
            low = mid + 1;
            // mid is a ZERO → mid is DEFINITELY NOT the answer
            // Answer is STRICTLY AFTER mid, so exclude mid
            //
            // CALCULATION: low=0, mid=1, arr[1]=0 → low = 1+1 = 2
            //              low=3, mid=3, arr[3]=0 → low = 3+1 = 4
            //
            // WHY NOT low = mid?
            //   low=0, high=2, mid=1, arr[1]=0, low=mid=1
            //   Next: low=1, high=2, mid=1+(2-1)/2=1, arr[1]=0, low=mid=1
            //   Next: low=1, high=2, mid=1 → INFINITE LOOP! Values never change.
            //
            //   low = mid + 1 GUARANTEES progress: low strictly increases.
        } else {
            high = mid;
            // mid is NON-ZERO → mid MIGHT be the answer (first non-zero)
            // Can't exclude mid, must include it in future search
            //
            // CALCULATION: high=5, mid=2, arr[2]=3≠0 → high = 2
            //              high=2, mid=1, arr[1]=5≠0 → high = 1
            //
            // WHY NOT high = mid - 1?
            //   arr = [0, 0, 3, 5, 7], answer = 2
            //   Step 1: low=0, high=5, mid=2, arr[2]=3≠0, high=mid-1=1
            //   Step 2: low=0, high=1, mid=0, arr[0]=0, low=1
            //   Step 3: low=1, high=1, STOP, return 1 ✗ WRONG! Answer is 2.
            //
            //   We SKIPPED index 2 by setting high=1. But index 2 WAS the answer!
            //   
            //   With high = mid = 2:
            //   Step 1: low=0, high=5, mid=2, arr[2]=3≠0, high=2
            //   Step 2: low=0, high=2, mid=1, arr[1]=0, low=2
            //   Step 3: low=2, high=2, STOP, return 2 ✓ CORRECT!
        }
    }

    low
    // At this point: low == high (they've converged)
    // low is the index of the first non-zero element (or n if all are zero)
    // low is also the COUNT of leading zeroes (indices 0..low are all zero)
    //
    // WHY return low and not high?
    //   They're equal at this point, so either works.
    //   Convention: return low.
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // TEST 1: Example from problem statement
    // ========================================================================
    // index:   0   1   2   3   4   5   6   7   8   9  10  11
    //        +---+---+---+---+---+---+---+---+---+---+---+---+
    // value: | 0 | 0 | 0 | 0 | 0 | 3 | 2 | 8 |11 |10 |15 |22 |
    //        +---+---+---+---+---+---+---+---+---+---+---+---+
    //        [=====ZERO=====][=========NON-ZERO============]
    //              5 zeroes            7 non-zeroes
    // answer = 5 because indices {0,1,2,3,4} contain 0
    #[test]
    fn test_example_from_problem() {
        let arr = [0, 0, 0, 0, 0, 3, 2, 8, 11, 10, 15, 22];
        assert_eq!(count_zeroes(&arr), 5);
    }

    // ========================================================================
    // TEST 2: Almost all zeroes (boundary near end)
    // ========================================================================
    // index:   0   1   2   3   4   5   6   7
    //        +---+---+---+---+---+---+---+---+
    // value: | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 |
    //        +---+---+---+---+---+---+---+---+
    //        [=========ZERO=============][N]
    //                 7 zeroes           1 non-zero
    // answer = 7
    // THIS TESTS: Does your search correctly find boundary when it's very far right?
    #[test]
    fn test_almost_all_zeroes() {
        let arr = [0, 0, 0, 0, 0, 0, 0, 9];
        assert_eq!(count_zeroes(&arr), 7);
    }

    // ========================================================================
    // TEST 3: Only one zero (boundary near start)
    // ========================================================================
    // index:   0   1   2   3   4   5
    //        +---+---+---+---+---+---+
    // value: | 0 | 5 | 8 | 3 | 1 | 2 |
    //        +---+---+---+---+---+---+
    //        [Z][=====NON-ZERO=======]
    //         1          5
    // answer = 1
    // THIS TESTS: Does your search correctly find boundary when it's very far left?
    #[test]
    fn test_one_zero() {
        let arr = [0, 5, 8, 3, 1, 2];
        assert_eq!(count_zeroes(&arr), 1);
    }

    // ========================================================================
    // TEST 4: No zeroes at all (boundary at index 0)
    // ========================================================================
    // index:   0   1   2   3   4
    //        +---+---+---+---+---+
    // value: | 7 | 5 | 8 | 3 | 1 |
    //        +---+---+---+---+---+
    //        [=====NON-ZERO=======]
    //                5 non-zeroes
    // answer = 0 (zero zeroes)
    // THIS TESTS: Does your search return 0 when first element is non-zero?
    #[test]
    fn test_no_zeroes() {
        let arr = [7, 5, 8, 3, 1];
        assert_eq!(count_zeroes(&arr), 0);
    }

    // ========================================================================
    // TEST 5: All zeroes (boundary at index n)
    // ========================================================================
    // index:   0   1   2   3
    //        +---+---+---+---+
    // value: | 0 | 0 | 0 | 0 |
    //        +---+---+---+---+
    //        [=====ZERO=======]
    //              4 zeroes
    // answer = 4 (n, all elements are zeroes)
    // THIS TESTS: Does your search return n when all elements are zeroes?
    // CRITICAL: If you initialized high=n-1, you might return 3 instead of 4!
    #[test]
    fn test_all_zeroes() {
        let arr = [0, 0, 0, 0];
        assert_eq!(count_zeroes(&arr), 4);
    }

    // ========================================================================
    // TEST 6: Single element - zero
    // ========================================================================
    // index:   0
    //        +---+
    // value: | 0 |
    //        +---+
    // answer = 1
    #[test]
    fn test_single_zero() {
        let arr = [0];
        assert_eq!(count_zeroes(&arr), 1);
    }

    // ========================================================================
    // TEST 7: Single element - non-zero
    // ========================================================================
    // index:   0
    //        +---+
    // value: | 5 |
    //        +---+
    // answer = 0
    #[test]
    fn test_single_non_zero() {
        let arr = [5];
        assert_eq!(count_zeroes(&arr), 0);
    }

    // ========================================================================
    // TEST 8: Empty array
    // ========================================================================
    // index: (none)
    // value: (none)
    // answer = 0 (no elements, no zeroes)
    // THIS TESTS: Does your code handle empty slice without panic?
    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(count_zeroes(&arr), 0);
    }

    // ========================================================================
    // TEST 9: Three zeroes (tests integer division in mid calculation)
    // ========================================================================
    // index:   0   1   2   3   4   5   6   7   8   9
    //        +---+---+---+---+---+---+---+---+---+---+
    // value: | 0 | 0 | 0 | 5 | 7 | 2 | 9 | 1 | 4 | 6 |
    //        +---+---+---+---+---+---+---+---+---+---+
    //        [==ZERO==][=======NON-ZERO=============]
    //           3                  7
    // answer = 3
    #[test]
    fn test_three_zeroes() {
        let arr = [0, 0, 0, 5, 7, 2, 9, 1, 4, 6];
        assert_eq!(count_zeroes(&arr), 3);
    }

    // ========================================================================
    // TEST 10-12: Two element edge cases
    // ========================================================================
    // [0, 0] → answer = 2 (all zeroes)
    // [0, 1] → answer = 1 (boundary at index 1)
    // [3, 1] → answer = 0 (no zeroes)
    #[test]
    fn test_two_elements_both_zero() {
        let arr = [0, 0];
        assert_eq!(count_zeroes(&arr), 2);
    }

    #[test]
    fn test_two_elements_one_zero() {
        let arr = [0, 1];
        assert_eq!(count_zeroes(&arr), 1);
    }

    #[test]
    fn test_two_elements_no_zero() {
        let arr = [3, 1];
        assert_eq!(count_zeroes(&arr), 0);
    }
}
