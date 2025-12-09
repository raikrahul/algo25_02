
fn find_anchor(arr: &[i32]) -> Option<usize> {
    // ═══════════════════════════════════════════════════════════════════════════════════════════
    // WHY NOT SEARCH FOR ZERO DIRECTLY?
    // ═══════════════════════════════════════════════════════════════════════════════════════════
    // Traditional binary search: find target T in sorted array. Compare arr[mid] to T.
    // THIS PROBLEM: No target T exists. We search for INDEX i where arr[i] = i.
    // TRICK: Define g(i) = arr[i] - i. Search for g(i) = 0.
    // g(i) is NON-DECREASING because:
    //   - arr is sorted + distinct → arr[i+1] >= arr[i] + 1
    //   - g(i+1) = arr[i+1] - (i+1) >= (arr[i]+1) - i - 1 = arr[i] - i = g(i)
    // So we binary search on the IMPLICIT array g[], looking for where g crosses zero.
    // ═══════════════════════════════════════════════════════════════════════════════════════════

    // ───────────────────────────────────────────────────────────────────────────────────────────
    // low: usize - The LEFT boundary of search space (INCLUSIVE).
    // ───────────────────────────────────────────────────────────────────────────────────────────
    // SMALL SCALE (n=4):  arr = [-5, -2, 2, 6]. low = 0. Search space = [0, 1, 2, 3].
    // MID SCALE (n=7):    arr = [-20, -10, -1, 3, 5, 9, 12]. low = 0. Search space = [0..6].
    // LARGE SCALE (n=1000000): arr = [-999999, ..., 500000, ...]. low = 0.
    // EDGE (n=0): arr = []. low = 0. high = 0. Loop never runs. Return None.
    // EDGE (n=1): arr = [0]. low = 0. high = 1. mid = 0. arr[0]=0. 0==0? YES. Return Some(0).
    // EDGE (n=1): arr = [5]. low = 0. high = 1. mid = 0. arr[0]=5. 5==0? NO. 5>0? YES. high=0. Exit. None.
    let mut low: usize = 0;

    // ───────────────────────────────────────────────────────────────────────────────────────────
    // high: usize - The RIGHT boundary of search space (EXCLUSIVE, like Rust ranges).
    // ───────────────────────────────────────────────────────────────────────────────────────────
    // SMALL SCALE (n=4):  high = 4. Valid indices = [0, 1, 2, 3]. 4 is OUT OF BOUNDS.
    // MID SCALE (n=7):    high = 7. Valid indices = [0..6].
    // LARGE SCALE (n=1000000): high = 1000000.
    // WHY EXCLUSIVE: Rust convention. [low, high) = half-open interval. Avoids off-by-one.
    // CALCULATION: arr.len() returns count of elements.
    //   arr = [-5, -2, 2, 6] → arr.len() = 4.
    //   arr = [-20, -10, -1, 3, 5, 9, 12] → arr.len() = 7.
    let mut high: usize = arr.len();

    // ───────────────────────────────────────────────────────────────────────────────────────────
    // LOOP: while low < high
    // ───────────────────────────────────────────────────────────────────────────────────────────
    // TERMINATION: When low >= high, search space is empty. No anchor found.
    // SMALL SCALE TRACE (arr = [-5, -2, 2, 6]):
    //   Iter 1: low=0, high=4. 0<4? YES. Continue.
    //   mid = 0 + (4-0)/2 = 2. arr[2]=2. 2==2? YES. Return Some(2).
    // MID SCALE TRACE (arr = [1, 2, 3, 4], no anchor):
    //   Iter 1: low=0, high=4. mid=2. arr[2]=3. 3==2? NO. 3>2? YES. high=2.
    //   Iter 2: low=0, high=2. mid=1. arr[1]=2. 2==1? NO. 2>1? YES. high=1.
    //   Iter 3: low=0, high=1. mid=0. arr[0]=1. 1==0? NO. 1>0? YES. high=0.
    //   Iter 4: low=0, high=0. 0<0? NO. Exit. Return None.
    // LARGE SCALE (n=1000000, anchor at 500000):
    //   Binary search cuts in half each time. log2(1000000) ≈ 20 iterations max.
    //   Iter 1: mid = 500000. If arr[500000]=500000, done in 1 step.
    while low < high {
        // ───────────────────────────────────────────────────────────────────────────────────────
        // mid: usize - The MIDDLE index of current search space.
        // ───────────────────────────────────────────────────────────────────────────────────────
        // FORMULA: mid = low + (high - low) / 2.
        // WHY NOT (low + high) / 2? Overflow at large scales. If low=2^31, high=2^31, sum overflows.
        // CALCULATION EXAMPLES:
        //   low=0, high=4:  mid = 0 + (4-0)/2 = 0 + 4/2 = 0 + 2 = 2.
        //   low=0, high=7:  mid = 0 + (7-0)/2 = 0 + 7/2 = 0 + 3 = 3. (Integer division: 7/2=3, not 3.5)
        //   low=4, high=7:  mid = 4 + (7-4)/2 = 4 + 3/2 = 4 + 1 = 5.
        //   low=5, high=7:  mid = 5 + (7-5)/2 = 5 + 2/2 = 5 + 1 = 6.
        //   low=6, high=7:  mid = 6 + (7-6)/2 = 6 + 1/2 = 6 + 0 = 6. (1/2=0 in integer division!)
        // EDGE: low=0, high=1: mid = 0 + (1-0)/2 = 0 + 0 = 0. (Single element case.)
        let mid = low + (high - low) / 2;

        // ───────────────────────────────────────────────────────────────────────────────────────
        // COMPARISON: arr[mid] vs mid (as i32)
        // ───────────────────────────────────────────────────────────────────────────────────────
        // TYPE CAST NEEDED: arr[mid] is i32 (can be negative). mid is usize (always >= 0).
        // Rust forbids comparing i32 to usize directly. Cast mid to i32.
        // EXAMPLE VALUES:
        //   arr = [-20, -10, -1, 3, 5, 9, 12], mid = 3.
        //   arr[3] = 3 (i32). mid as i32 = 3 (i32). 3 == 3? YES.
        //   arr = [-5, -2, 2, 6], mid = 0.
        //   arr[0] = -5 (i32). mid as i32 = 0 (i32). -5 == 0? NO. -5 > 0? NO. Else branch.
        // g(mid) CALCULATION:
        //   g(mid) = arr[mid] - mid.
        //   arr[3]=3, mid=3: g(3) = 3 - 3 = 0. ANCHOR.
        //   arr[0]=-5, mid=0: g(0) = -5 - 0 = -5 < 0. Anchor is RIGHT.
        //   arr[0]=1, mid=0: g(0) = 1 - 0 = 1 > 0. Anchor is LEFT.

        if arr[mid] == mid as i32 {
            // ═══════════════════════════════════════════════════════════════════════════════════
            // FOUND: arr[mid] = mid means g(mid) = 0. This index IS the anchor.
            // ═══════════════════════════════════════════════════════════════════════════════════
            // SMALL: arr=[-5,-2,2,6], mid=2. arr[2]=2. Return Some(2).
            // MID: arr=[-20,-10,-1,3,5,9,12], mid=3. arr[3]=3. Return Some(3).
            // LARGE: arr with 10^6 elements, arr[500000]=500000. Return Some(500000).
            return Some(mid);
        } else if arr[mid] > mid as i32 {
            // ═══════════════════════════════════════════════════════════════════════════════════
            // g(mid) > 0: The value is TOO BIG. Anchor (if exists) is to the LEFT.
            // ═══════════════════════════════════════════════════════════════════════════════════
            // WHY LEFT: g is non-decreasing. If g(mid) > 0, then for all j > mid, g(j) >= g(mid) > 0.
            //   No anchor can exist to the right. Discard right half.
            // NUMERICAL PROOF (arr = [0, 5, 10, 15]):
            //   g = [0, 4, 8, 12]. All positive except g(0)=0.
            //   mid = 1. g(1) = 4 > 0.
            //   g(2) = 8 >= 4 > 0. g(3) = 12 >= 8 > 0. No anchor at 2 or 3.
            //   Anchor must be at index < 1. Set high = 1.
            // UPDATE: high = mid (not mid-1, because [low, high) is half-open).
            // CALCULATION:
            //   low=0, high=4, mid=2. arr[2]=10. 10 > 2? YES. high = 2.
            //   New search space: [0, 2) = indices 0, 1.
            high = mid;
        } else {
            // ═══════════════════════════════════════════════════════════════════════════════════
            // g(mid) < 0: The value is TOO SMALL. Anchor (if exists) is to the RIGHT.
            // ═══════════════════════════════════════════════════════════════════════════════════
            // WHY RIGHT: g is non-decreasing. If g(mid) < 0, then for all j < mid, g(j) <= g(mid) < 0.
            //   No anchor can exist to the left. Discard left half INCLUDING mid.
            // NUMERICAL PROOF (arr = [-10, -5, -1, 0, 1, 5, 6]):
            //   g = [-10, -6, -3, -3, -3, 0, 0].
            //   mid = 3. g(3) = -3 < 0.
            //   g(2) = -3 <= -3 < 0. g(1) = -6 <= -3 < 0. g(0) = -10 <= -6 < 0.
            //   No anchor at 0, 1, 2, or 3. Anchor must be at index > 3. Set low = 4.
            // UPDATE: low = mid + 1 (exclude mid, it's not an anchor).
            // CALCULATION:
            //   low=0, high=7, mid=3. arr[3]=0. 0 < 3? YES. low = 3 + 1 = 4.
            //   New search space: [4, 7) = indices 4, 5, 6.
            // WHY mid + 1 NOT mid:
            //   mid is already checked and is NOT an anchor (g(mid) != 0).
            //   Including mid in next iteration wastes a comparison.
            low = mid + 1;
        }
    }

    // ───────────────────────────────────────────────────────────────────────────────────────────
    // NO ANCHOR FOUND: Loop exited because low >= high. Search space is empty.
    // ───────────────────────────────────────────────────────────────────────────────────────────
    // EXAMPLE: arr = [1, 2, 3, 4]. g = [1, 1, 1, 1]. All positive. No zero crossing.
    //   Trace: low shrinks high repeatedly until high=0. low=0, high=0. Exit. Return None.
    // EDGE: arr = []. low=0, high=0. 0<0? NO. Never entered loop. Return None.
    // EDGE: arr = [-5]. low=0, high=1. mid=0. arr[0]=-5. -5<0. low=1. 1<1? NO. Return None.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        // Index: 0   1   2  3  4  5   6
        // Value:-20 -10 -1  3  5  9  12
        let arr = vec![-20, -10, -1, 3, 5, 9, 12];
        assert_eq!(find_anchor(&arr), Some(3));
    }

    #[test]
    fn test_no_anchor() {
        // Index: 0  1  2  3
        // Value: 1  2  3  4
        // Dims:  1-0=1, 2-1=1, ... all > 0
        let arr = vec![1, 2, 3, 4];
        assert_eq!(find_anchor(&arr), None);
    }
    
    #[test]
    fn test_negative_anchor() {
        // Index: 0   1   2   3
        // Value:-5  -2   2   6
        // Dims: -5, -3,  0,  3
        // Anchor at 2? A[2]=2.
        let arr = vec![-5, -2, 2, 6];
        assert_eq!(find_anchor(&arr), Some(2));
    }
}
