// =============================================================================
// Problem 3: Find Any Duplicate Number
// =============================================================================
// Given an array of N integers where each element is between 1 and N-1,
// write an efficient function to find any duplicated integer.
// You may destroy the array.
// 
// Function Prototype: fn find_duplicate(a: &mut [i32]) -> i32
// 
// Time Complexity Target: O(N)
// Space Complexity Target: O(1)
// =============================================================================

// =============================================================================
// THE CONSTRAINT THAT YOU WILL FORGET
// =============================================================================
// If the array has N elements (indices 0, 1, 2, ..., N-1)
// And each value is in the range [1, N-1]
// Then there are N slots but only N-1 possible distinct values.
//
// EXAMPLE with N=7:
// Indices:  0   1   2   3   4   5   6   (7 slots)
// Values:   must be in [1, 6]          (only 6 possible values)
// 
// Pigeonhole: 7 slots ÷ 6 values = at least ceiling(7/6) = 2 elements share same value
//
// CRITICAL OBSERVATION YOU WILL MISS:
// If a[i] = V, and V is in [1, N-1], then V is a VALID INDEX into the array!
// Value 3 can point to index 3
// Value 1 can point to index 1
// Value 6 can point to index 6
//
// This VALUE↔INDEX correspondence is the entire trick.
// =============================================================================

// =============================================================================
// NUMERICAL EXAMPLE 1: Small Array
// =============================================================================
// Array: [2, 1, 2]   N=3, values in [1, 2], indices in [0, 2]
//
// Memory layout at start:
// Address:  0x1000  0x1004  0x1008
// Index:       0       1       2
// Value:       2       1       2
//
// The value 2 appears at index 0 and index 2.
// Answer: 2
//
// Middle calculation (starting from i=1 instead of i=0):
// If we process i=1 first:
//   a[1] = 1 → go to index 1 → a[1] = 1 (positive) → mark a[1] = -1
//   Array: [2, -1, 2]
// If we process i=0 next:
//   a[0] = 2 → go to index 2 → a[2] = 2 (positive) → mark a[2] = -2
//   Array: [2, -1, -2]
// If we process i=2:
//   a[2] = -2 → |−2| = 2 → go to index 2 → a[2] = -2 (NEGATIVE!) → FOUND: 2
//
// SURPRISE: The absolute value step is CRITICAL. Without it, we get index -2 which crashes.
// =============================================================================

// =============================================================================
// NUMERICAL EXAMPLE 2: Self-Pointing Value
// =============================================================================
// Array: [1, 1]   N=2, values in [1, 1], indices in [0, 1]
//
// Memory layout:
// Index:    0     1
// Value:    1     1
//
// Step i=0:
//   a[0] = 1 → index 1 → a[1] = 1 (positive) → mark a[1] = -1
//   Array: [1, -1]
//
// Step i=1:
//   a[1] = -1 → |−1| = 1 → index 1 → a[1] = -1 (NEGATIVE!) → FOUND: 1
//
// CALCULATION CHECK:
// - We iterate 2 times (i=0, i=1)
// - We find duplicate at i=1
// - Return value is |a[1]| = |−1| = 1
//
// WHAT IF N=2 and array is [1, 1]?
// Values range [1, N-1] = [1, 1]. Only value 1 is possible.
// 2 slots, 1 possible value. Pigeonhole guarantees duplicate.
// 2 / 1 = 2 ≥ 2, so at least 2 elements share value 1. CHECK.
// =============================================================================

// =============================================================================
// NUMERICAL EXAMPLE 3: Larger Array with Trace
// =============================================================================
// Array: [3, 1, 3, 4, 2, 6, 5]   N=7, values in [1, 6], indices in [0, 6]
//
// Initial state (memory addresses are illustrative):
// Addr:   0x100  0x104  0x108  0x10C  0x110  0x114  0x118
// Index:     0      1      2      3      4      5      6
// Value:     3      1      3      4      2      6      5
//
// i=0: a[0]=3 → index 3 → a[3]=4 (pos) → a[3]=-4
//      [3, 1, 3, -4, 2, 6, 5]
//      Calculation: target_index = |3| = 3. Check a[3] = 4 > 0. Mark a[3] = -4.
//
// i=1: a[1]=1 → index 1 → a[1]=1 (pos) → a[1]=-1
//      [3, -1, 3, -4, 2, 6, 5]
//      Calculation: target_index = |1| = 1. Check a[1] = 1 > 0. Mark a[1] = -1.
//
// i=2: a[2]=3 → index 3 → a[3]=-4 (NEGATIVE!) → FOUND!
//      Return value = |3| = 3
//      Calculation: target_index = |3| = 3. Check a[3] = -4 < 0. DUPLICATE!
//
// SURPRISE RESULT: We found the duplicate at i=2 after only 3 iterations.
// The duplicate value is 3, not 2 (which is the index).
// =============================================================================

// =============================================================================
// NUMERICAL EXAMPLE 4: All Same Value
// =============================================================================
// Array: [2, 2, 2, 2]   N=4, values in [1, 3], duplicate value 2
//
// i=0: a[0]=2 → index 2 → a[2]=2 (pos) → a[2]=-2
//      [2, 2, -2, 2]
//
// i=1: a[1]=2 → index 2 → a[2]=-2 (NEGATIVE!) → FOUND!
//      Return |a[1]| = |2| = 2
//
// SURPRISE: Found on second element. Fastest possible for this case.
// 
// HARDER CALCULATION:
// What if array was [2, 2, 2, 2, 2, 2, 2, 2, 2, 2] (N=10)?
// Still finds duplicate at i=1!
// Because a[0] marks index 2, and a[1] also tries to mark index 2.
// =============================================================================

// =============================================================================
// NUMERICAL EXAMPLE 5: Edge Case with Maximum Value
// =============================================================================
// Array: [5, 4, 3, 2, 1, 3]   N=6, values in [1, 5]
//
// i=0: a[0]=5 → index 5 → a[5]=3 (pos) → a[5]=-3
//      [5, 4, 3, 2, 1, -3]
//
// i=1: a[1]=4 → index 4 → a[4]=1 (pos) → a[4]=-1
//      [5, 4, 3, 2, -1, -3]
//
// i=2: a[2]=3 → index 3 → a[3]=2 (pos) → a[3]=-2
//      [5, 4, 3, -2, -1, -3]
//
// i=3: a[3]=-2 → |−2|=2 → index 2 → a[2]=3 (pos) → a[2]=-3
//      [5, 4, -3, -2, -1, -3]
//      SURPRISE: We had to take abs of a[3] because it was already negative!
//
// i=4: a[4]=-1 → |−1|=1 → index 1 → a[1]=4 (pos) → a[1]=-4
//      [5, -4, -3, -2, -1, -3]
//
// i=5: a[5]=-3 → |−3|=3 → index 3 → a[3]=-2 (NEGATIVE!) → FOUND!
//      Return |a[5]| = |−3| = 3
//
// FRACTIONAL CHECK: Is 3 really the duplicate?
// Original array: [5, 4, 3, 2, 1, 3]
// Value 3 appears at index 2 and index 5. YES, duplicate is 3. ✓
// =============================================================================

// =============================================================================
// NUMERICAL EXAMPLE 6: Mid-scale Breaking Test
// =============================================================================
// Array: [7, 3, 1, 8, 2, 5, 9, 7, 4, 6]   N=10, values in [1, 9]
//
// Trace (showing only state change):
// Initial: [7, 3, 1, 8, 2, 5, 9, 7, 4, 6]
//
// i=0: val=7 → idx=7 → a[7]=7>0 → a[7]=-7
//      [7, 3, 1, 8, 2, 5, 9, -7, 4, 6]
//      Step calculation: |a[0]| = |7| = 7. a[7] = 7 > 0. Mark.
//
// i=1: val=3 → idx=3 → a[3]=8>0 → a[3]=-8
//      [7, 3, 1, -8, 2, 5, 9, -7, 4, 6]
//
// i=2: val=1 → idx=1 → a[1]=3>0 → a[1]=-3
//      [7, -3, 1, -8, 2, 5, 9, -7, 4, 6]
//
// i=3: val=-8 → |−8|=8 → idx=8 → a[8]=4>0 → a[8]=-4
//      [7, -3, 1, -8, 2, 5, 9, -7, -4, 6]
//      BREAK CHECK: At i=3, we read a[3]=-8. Absolute value gives 8.
//
// i=4: val=2 → idx=2 → a[2]=1>0 → a[2]=-1
//      [7, -3, -1, -8, 2, 5, 9, -7, -4, 6]
//
// i=5: val=5 → idx=5 → a[5]=5>0 → a[5]=-5
//      [7, -3, -1, -8, 2, -5, 9, -7, -4, 6]
//
// i=6: val=9 → idx=9 → a[9]=6>0 → a[9]=-6
//      [7, -3, -1, -8, 2, -5, 9, -7, -4, -6]
//
// i=7: val=-7 → |−7|=7 → idx=7 → a[7]=-7<0 → FOUND!
//      Return |a[7]| = |−7| = 7
//
// VERIFICATION: Original array [7, 3, 1, 8, 2, 5, 9, 7, 4, 6]
// Value 7 at index 0 and index 7. ✓
// =============================================================================

// =============================================================================
// WHAT INDEX-VALUE FORMULA TO USE?
// =============================================================================
// The constraint says values are [1, N-1] and indices are [0, N-1].
//
// Option A: Use value directly as index
//   target_index = |a[i]|
//   Works because 1 ≤ value ≤ N-1, so 1 ≤ target_index ≤ N-1
//   Index 0 is never targeted, but we still iterate over i=0.
//
// Option B: Use value-1 as index
//   target_index = |a[i]| - 1
//   Maps values [1, N-1] to indices [0, N-2]
//   Index N-1 is never targeted.
//
// WHICH TO USE? Depends on what feels natural.
// Option A: simpler formula, but index 0 is "wasted" as a target
// Option B: more complex formula, but uses all indices 0 to N-2
//
// For this implementation, we'll use OPTION A for simplicity.
// target_index = |a[i]|
// Since values are 1 to N-1, this targets indices 1 to N-1.
// Index 0 is never a target, but we still iterate i=0, 1, ..., N-1.
// =============================================================================

/// Find any duplicate value in an array where each element is in [1, N-1].
/// 
/// This function MUTATES the input array by negating values as visited markers.
/// 
/// # Arguments
/// * `a` - A mutable slice of i32 values. Each value must be in range [1, a.len()-1].
///
/// # Returns
/// * The duplicated integer value
///
/// # Panics
/// * If no duplicate is found (which contradicts the problem constraints)
///
/// # Time Complexity
/// O(N) where N = a.len()
///
/// # Space Complexity  
/// O(1) - uses no additional space, only mutates input array
///
/// # Algorithm Skeleton (YOU FILL IN THE BODY):
/// ```
/// for i in 0..a.len() {
///     // Step 1: Get the value at index i (might be negative if marked)
///     //         YOU MUST TAKE ABSOLUTE VALUE
///     //         let val = ???
///     
///     // Step 2: Use this value as an index into the array
///     //         let target_index = ???
///     
///     // Step 3: Check if a[target_index] is already negative
///     //         If yes: this value is the duplicate, return it
///     //         If no: mark it by negating a[target_index]
///     
///     // DON'T FORGET: return the VALUE not the INDEX when duplicate found
/// }
/// // Should never reach here if input satisfies constraints
/// panic!("No duplicate found - invalid input");
/// ```
pub fn find_duplicate(a: &mut [i32]) -> i32 {
    // ==========================================================
    // YOUR IMPLEMENTATION GOES HERE
    // DO NOT LOOK AT SOLUTIONS
    // USE THE NUMERICAL EXAMPLES ABOVE TO GUIDE YOU
    // ==========================================================

    // START MIDDLE CALCULATION i=2 (The Breaking Point)
    // -------------------------------------------------------------------------
    // Scenario: Array was [3, 1, 3, 4, 2, 6, 5]
    // Previous steps i=0, i=1 have run.
    //
    // State entering i=2:
    // Memory: [3, -1, 3, -4, 2, 6, 5]
    //          ^   ^  ^   ^  ^  ^  ^
    // Indices: 0   1  2   3  4  5  6
    //
    // i = 2
    // read a[2] -> value = 3
    // abs_val = |3| = 3  (This 3 implies "I want to visit index 3")
    // target_idx = 3     (We are targeting the slot at index 3)
    //
    // read a[target_idx] aka a[3] -> value is -4
    // CHECK: Is -4 < 0? Yes!
    // CONCLUSION: Someone visited index 3 before!
    // WHO visited it? Someone with value 3.
    // WHO am I? I am value 3.
    // Therefore, 3 is the duplicate.
    // ACTION: Return abs_val (3). DO NOT return a[3] (-4).

    // START FROM BEGINNING (The Loop)
    // -------------------------------------------------------------------------
    for i in 0..a.len() {
        // 1. SCARY MATH: Extract value at current index i.
        // It might be negative due to previous markings.
        // Example: if a[i] is -15, it means "Originally 15, but visited".
        // We need the "15" part to use as an index.
        let val = a[i];
        let val_abs = val.abs() as usize;

        // 2. POINTER JUMP: Go to the index dictated by val_abs.
        // If val_abs is 3, we look at index 3.
        // If val_abs is 100, we look at index 100.
        // Safe because problem guarantees val in [1, N-1].
        let target_idx = val_abs;

        // 3. COLLISION CHECK: Look at the target.
        // Is it negative?
        // IF YES: A previous iteration (let's say j < i) had a[j] = val_abs.
        //         That iteration marked a[target_idx] as negative.
        //         So we have found the second occurrence of val_abs.
        //         RETURN val_abs (the duplicate value).
        if a[target_idx] < 0 {
            return val_abs as i32;
        }

        // 4. MARKING: It's positive. First time seeing this value.
        // We negate the value at the target index to leave a "crumb".
        // a[target_idx] = -a[target_idx]
        // This preserves the value (magnitude) but adds 1 bit of info (sign).
        a[target_idx] = -a[target_idx];
    }

    // IMPOSSIBLE STATE per Pigeonhole Principle
    panic!("No duplicate found - input violated constraints");
}


// =============================================================================
// TEST CASES
#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test 1: Minimal case N=2
    // -------------------------------------------------------------------------
    // Array: [1, 1]
    // N = 2, values in [1, 1]
    // Only one possible value, must be duplicate
    // Expected: 1
    #[test]
    fn test_minimal_n2() {
        let mut arr = [1, 1];
        assert_eq!(find_duplicate(&mut arr), 1);
    }

    // -------------------------------------------------------------------------
    // Test 2: Small case N=3
    // -------------------------------------------------------------------------
    // Array: [2, 1, 2]
    // N = 3, values in [1, 2]
    // Value 2 appears at indices 0 and 2
    // Expected: 2
    #[test]
    fn test_small_n3() {
        let mut arr = [2, 1, 2];
        assert_eq!(find_duplicate(&mut arr), 2);
    }

    // -------------------------------------------------------------------------
    // Test 3: Another small case N=3
    // -------------------------------------------------------------------------
    // Array: [1, 2, 1]
    // N = 3, values in [1, 2]
    // Value 1 appears at indices 0 and 2
    // Expected: 1
    #[test]
    fn test_small_n3_variant() {
        let mut arr = [1, 2, 1];
        assert_eq!(find_duplicate(&mut arr), 1);
    }

    // -------------------------------------------------------------------------
    // Test 4: Example from problem with N=7
    // -------------------------------------------------------------------------
    // Array: [3, 1, 3, 4, 2, 6, 5]
    // N = 7, values in [1, 6]
    // Value 3 appears at indices 0 and 2
    // Expected: 3
    #[test]
    fn test_example_n7() {
        let mut arr = [3, 1, 3, 4, 2, 6, 5];
        assert_eq!(find_duplicate(&mut arr), 3);
    }

    // -------------------------------------------------------------------------
    // Test 5: All same values
    // -------------------------------------------------------------------------
    // Array: [2, 2, 2, 2]
    // N = 4, values in [1, 3]
    // All elements are 2
    // Expected: 2
    #[test]
    fn test_all_same() {
        let mut arr = [2, 2, 2, 2];
        assert_eq!(find_duplicate(&mut arr), 2);
    }

    // -------------------------------------------------------------------------
    // Test 6: Duplicate at the end
    // -------------------------------------------------------------------------
    // Array: [5, 4, 3, 2, 1, 3]
    // N = 6, values in [1, 5]
    // Value 3 appears at indices 2 and 5
    // Expected: 3
    #[test]
    fn test_duplicate_at_end() {
        let mut arr = [5, 4, 3, 2, 1, 3];
        assert_eq!(find_duplicate(&mut arr), 3);
    }

    // -------------------------------------------------------------------------
    // Test 7: Large-ish array N=10
    // -------------------------------------------------------------------------
    // Array: [7, 3, 1, 8, 2, 5, 9, 7, 4, 6]
    // N = 10, values in [1, 9]
    // Value 7 appears at indices 0 and 7
    // Expected: 7
    #[test]
    fn test_n10() {
        let mut arr = [7, 3, 1, 8, 2, 5, 9, 7, 4, 6];
        assert_eq!(find_duplicate(&mut arr), 7);
    }

    // -------------------------------------------------------------------------
    // Test 8: Duplicate value 1
    // -------------------------------------------------------------------------
    // Array: [2, 3, 4, 1, 1]
    // N = 5, values in [1, 4]
    // Value 1 appears at indices 3 and 4
    // Expected: 1
    #[test]
    fn test_duplicate_value_one() {
        let mut arr = [2, 3, 4, 1, 1];
        assert_eq!(find_duplicate(&mut arr), 1);
    }

    // -------------------------------------------------------------------------
    // Test 9: Duplicate at consecutive positions
    // -------------------------------------------------------------------------
    // Array: [1, 2, 3, 3, 4]
    // N = 5, values in [1, 4]
    // Value 3 appears at indices 2 and 3
    // Expected: 3
    #[test]
    fn test_consecutive_duplicate() {
        let mut arr = [1, 2, 3, 3, 4];
        assert_eq!(find_duplicate(&mut arr), 3);
    }

    // -------------------------------------------------------------------------
    // Test 10: Self-pointing value case
    // -------------------------------------------------------------------------
    // Array: [2, 1, 3, 4, 3, 5]
    // N = 6, values in [1, 5]
    // a[1] = 1 (self-pointing)
    // Value 3 appears at indices 2 and 4
    // Expected: 3
    #[test]
    fn test_self_pointing() {
        let mut arr = [2, 1, 3, 4, 3, 5];
        assert_eq!(find_duplicate(&mut arr), 3);
    }
}
