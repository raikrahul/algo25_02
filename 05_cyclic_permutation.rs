// ╔═══════════════════════════════════════════════════════════════════════════════════════════════════╗
// ║ CYCLIC PERMUTATION DETECTION                                                                       ║
// ║ Problem: Given s1 and s2, return true if s2 is a cyclic rotation of s1                            ║
// ║                                                                                                    ║
// ║ What is a cyclic rotation?                                                                         ║
// ║ s1 = "ABCD" has 4 rotations:                                                                       ║
// ║   rotation 0: "ABCD" (original)                                                                    ║
// ║   rotation 1: "BCDA" (A moved from front to back)                                                  ║
// ║   rotation 2: "CDAB" (AB moved from front to back)                                                 ║
// ║   rotation 3: "DABC" (ABC moved from front to back)                                                ║
// ║                                                                                                    ║
// ║ The trick: If you concatenate s1 with itself, all rotations appear as substrings!                  ║
// ║ s1 + s1 = "ABCDABCD"                                                                               ║
// ║   Position 0: "ABCD" ← rotation 0                                                                  ║
// ║   Position 1: "BCDA" ← rotation 1                                                                  ║
// ║   Position 2: "CDAB" ← rotation 2                                                                  ║
// ║   Position 3: "DABC" ← rotation 3                                                                  ║
// ╚═══════════════════════════════════════════════════════════════════════════════════════════════════╝

// ┌─────────────────────────────────────────────────────────────────────────────────────┐
// │ NUMERICAL TRACE 1: s1 = "ABCD", s2 = "CDAB"                                         │
// │                                                                                     │
// │ Step 1: Length check                                                                │
// │   len(s1) = how many chars? Count: A(1), B(2), C(3), D(4) = 4                       │
// │   len(s2) = how many chars? Count: C(1), D(2), A(3), B(4) = 4                       │
// │   4 == 4? This evaluates to TRUE, so we proceed.                                    │
// │   If this were FALSE, we return false immediately without any more work.            │
// │                                                                                     │
// │ Step 2: Concatenation                                                               │
// │   s1 = "ABCD" at memory addresses say 0x1000: [A, B, C, D]                          │
// │   s1 again = "ABCD"                                                                 │
// │   s1 + s1 = "ABCD" || "ABCD" = "ABCDABCD"                                           │
// │   This new string has length = 4 + 4 = 8 characters                                 │
// │   Memory layout: [A, B, C, D, A, B, C, D] at indices [0,1,2,3,4,5,6,7]               │
// │                                                                                     │
// │ Step 3: Substring search - looking for "CDAB" inside "ABCDABCD"                     │
// │   We need to find 4 consecutive chars in the 8-char string that match CDAB          │
// │   Starting position i can be 0, 1, 2, 3, 4 (not 5,6,7 because 5+4=9 > 8)            │
// │                                                                                     │
// │   i=0: concat[0..4] = chars at 0,1,2,3 = A,B,C,D = "ABCD"                           │
// │         Compare: "ABCD" vs "CDAB"                                                   │
// │         char 0: 'A' vs 'C' → 65 vs 67 in ASCII → 65 ≠ 67 → MISMATCH                 │
// │         Stop early, move to i=1                                                     │
// │                                                                                     │
// │   i=1: concat[1..5] = chars at 1,2,3,4 = B,C,D,A = "BCDA"                           │
// │         Compare: "BCDA" vs "CDAB"                                                   │
// │         char 0: 'B' vs 'C' → 66 vs 67 → 66 ≠ 67 → MISMATCH                          │
// │         Stop early, move to i=2                                                     │
// │                                                                                     │
// │   i=2: concat[2..6] = chars at 2,3,4,5 = C,D,A,B = "CDAB"                           │
// │         Compare: "CDAB" vs "CDAB"                                                   │
// │         char 0: 'C' vs 'C' → 67 = 67 → MATCH, continue                              │
// │         char 1: 'D' vs 'D' → 68 = 68 → MATCH, continue                              │
// │         char 2: 'A' vs 'A' → 65 = 65 → MATCH, continue                              │
// │         char 3: 'B' vs 'B' → 66 = 66 → MATCH, all 4 matched!                        │
// │         FOUND at position 2. Return TRUE.                                           │
// │                                                                                     │
// │ The number 2 means: s1 was rotated left by 2 positions to get s2.                   │
// │   Original: A B C D (positions 0,1,2,3)                                             │
// │   Rotate 2: C D A B (C from pos 2 → pos 0, D from 3→1, A from 0→2, B from 1→3)      │
// └─────────────────────────────────────────────────────────────────────────────────────┘

// ┌─────────────────────────────────────────────────────────────────────────────────────┐
// │ NUMERICAL TRACE 2: s1 = "ABCD", s2 = "ACBD" (NOT a rotation!)                       │
// │                                                                                     │
// │ Step 1: Length check                                                                │
// │   len("ABCD") = 4, len("ACBD") = 4                                                  │
// │   4 == 4 → TRUE, so we don't return early. Proceed to concatenation.                │
// │                                                                                     │
// │ Step 2: Concatenation                                                               │
// │   s1 + s1 = "ABCDABCD" (8 chars)                                                    │
// │                                                                                     │
// │ Step 3: Substring search                                                            │
// │   Looking for "ACBD" in "ABCDABCD"                                                  │
// │                                                                                     │
// │   i=0: "ABCD" vs "ACBD"                                                             │
// │         'A' vs 'A' → 65 = 65 → match                                                │
// │         'B' vs 'C' → 66 vs 67 → 66 ≠ 67 → MISMATCH at position 1                    │
// │                                                                                     │
// │   i=1: "BCDA" vs "ACBD"                                                             │
// │         'B' vs 'A' → 66 vs 65 → MISMATCH at position 0                              │
// │                                                                                     │
// │   i=2: "CDAB" vs "ACBD"                                                             │
// │         'C' vs 'A' → 67 vs 65 → MISMATCH at position 0                              │
// │                                                                                     │
// │   i=3: "DABC" vs "ACBD"                                                             │
// │         'D' vs 'A' → 68 vs 65 → MISMATCH at position 0                              │
// │                                                                                     │
// │   i=4: "ABCD" vs "ACBD"                                                             │
// │         'A' vs 'A' → match                                                          │
// │         'B' vs 'C' → MISMATCH at position 1                                         │
// │                                                                                     │
// │   All positions exhausted. NOT FOUND. Return FALSE.                                 │
// │                                                                                     │
// │ Why is "ACBD" not a rotation of "ABCD"?                                             │
// │   Both have same characters: A, B, C, D (one each)                                  │
// │   But the ORDER is wrong. In any rotation:                                          │
// │     - A is always followed by B (with wraparound)                                   │
// │     - B is always followed by C                                                     │
// │     - C is always followed by D                                                     │
// │     - D is always followed by A                                                     │
// │   In "ACBD": A is followed by C (wrong!), so it's not a rotation.                   │
// └─────────────────────────────────────────────────────────────────────────────────────┘

// ┌─────────────────────────────────────────────────────────────────────────────────────┐
// │ NUMERICAL TRACE 3: Edge case - s1 = "ABCD", s2 = "CD" (different lengths)           │
// │                                                                                     │
// │ Step 1: Length check                                                                │
// │   len("ABCD") = 4                                                                   │
// │   len("CD") = 2                                                                     │
// │   4 == 2? This is 4 == 2 → FALSE                                                    │
// │                                                                                     │
// │   RETURN FALSE IMMEDIATELY.                                                         │
// │                                                                                     │
// │   We do NOT concatenate. We do NOT search.                                          │
// │   Why? Because cyclic permutation requires SAME LENGTH.                             │
// │   "CD" has only 2 characters, "ABCD" has 4.                                         │
// │   You cannot rotate a 4-character string to get a 2-character string.               │
// │                                                                                     │
// │ CRITICAL BUG PREVENTION:                                                            │
// │   If you skip this check and just do contains():                                    │
// │     s1 + s1 = "ABCDABCD"                                                            │
// │     "ABCDABCD".contains("CD") → TRUE because "CD" appears at position 2!            │
// │   But "CD" is NOT a cyclic permutation of "ABCD"!                                   │
// │   Your buggy code would return TRUE. Correct answer is FALSE.                       │
// └─────────────────────────────────────────────────────────────────────────────────────┘

// ┌─────────────────────────────────────────────────────────────────────────────────────┐
// │ NUMERICAL TRACE 4: Edge case - empty strings s1 = "", s2 = ""                       │
// │                                                                                     │
// │ Step 1: Length check                                                                │
// │   len("") = 0 (zero characters)                                                     │
// │   len("") = 0                                                                       │
// │   0 == 0 → TRUE, proceed.                                                           │
// │                                                                                     │
// │ Step 2: Concatenation                                                               │
// │   "" + "" = "" (still empty)                                                        │
// │                                                                                     │
// │ Step 3: Substring search                                                            │
// │   Does "" contain ""?                                                               │
// │   In Rust: "".contains("") → TRUE                                                   │
// │   The empty string is considered to be "found" in every string, including itself.   │
// │   This is by definition: empty substring match succeeds at position 0.              │
// │                                                                                     │
// │ Return TRUE.                                                                        │
// │                                                                                     │
// │ Is this correct? YES.                                                               │
// │   The empty string is the trivial cyclic permutation of itself.                     │
// │   Rotating nothing by 0 positions gives nothing.                                    │
// └─────────────────────────────────────────────────────────────────────────────────────┘

// ┌─────────────────────────────────────────────────────────────────────────────────────┐
// │ NUMERICAL TRACE 5: Single character - s1 = "A", s2 = "A"                            │
// │                                                                                     │
// │ Step 1: Length check                                                                │
// │   len("A") = 1, len("A") = 1                                                        │
// │   1 == 1 → TRUE, proceed.                                                           │
// │                                                                                     │
// │ Step 2: Concatenation                                                               │
// │   "A" + "A" = "AA" (2 characters)                                                   │
// │   Indices: [0, 1] → ['A', 'A']                                                      │
// │                                                                                     │
// │ Step 3: Substring search                                                            │
// │   Looking for "A" (1 char) in "AA" (2 chars)                                        │
// │   Starting positions: 0, 1                                                          │
// │                                                                                     │
// │   i=0: concat[0..1] = "A" vs "A" → 'A' vs 'A' → 65 = 65 → MATCH!                    │
// │   FOUND at position 0. Return TRUE.                                                 │
// │                                                                                     │
// │ Single character rotated by 0 is itself. Correct.                                   │
// └─────────────────────────────────────────────────────────────────────────────────────┘

// ┌─────────────────────────────────────────────────────────────────────────────────────┐
// │ NUMERICAL TRACE 6: Repeated characters - s1 = "AABB", s2 = "BBAA"                   │
// │                                                                                     │
// │ Step 1: Length check                                                                │
// │   len("AABB") = 4, len("BBAA") = 4                                                  │
// │   4 == 4 → TRUE, proceed.                                                           │
// │                                                                                     │
// │ Step 2: Concatenation                                                               │
// │   "AABB" + "AABB" = "AABBAABB" (8 characters)                                       │
// │   Indices: 0=A, 1=A, 2=B, 3=B, 4=A, 5=A, 6=B, 7=B                                   │
// │                                                                                     │
// │ Step 3: Substring search                                                            │
// │   Looking for "BBAA" in "AABBAABB"                                                  │
// │                                                                                     │
// │   i=0: concat[0..4] = "AABB" vs "BBAA"                                              │
// │         'A' vs 'B' → 65 vs 66 → MISMATCH                                            │
// │                                                                                     │
// │   i=1: concat[1..5] = "ABBA" vs "BBAA"                                              │
// │         'A' vs 'B' → MISMATCH                                                       │
// │                                                                                     │
// │   i=2: concat[2..6] = "BBAA" vs "BBAA"                                              │
// │         'B' vs 'B' → 66 = 66 → match                                                │
// │         'B' vs 'B' → 66 = 66 → match                                                │
// │         'A' vs 'A' → 65 = 65 → match                                                │
// │         'A' vs 'A' → 65 = 65 → match                                                │
// │         All 4 matched! FOUND at position 2. Return TRUE.                            │
// │                                                                                     │
// │ Verification: "AABB" rotated left by 2:                                             │
// │   Take first 2 chars: "AA"                                                          │
// │   Remaining: "BB"                                                                   │
// │   Result: "BB" + "AA" = "BBAA" ✓                                                    │
// └─────────────────────────────────────────────────────────────────────────────────────┘

// ┌─────────────────────────────────────────────────────────────────────────────────────┐
// │ NUMERICAL TRACE 7: All same characters - s1 = "AAA", s2 = "AAA"                     │
// │                                                                                     │
// │ Step 1: Length check                                                                │
// │   len("AAA") = 3, len("AAA") = 3                                                    │
// │   3 == 3 → TRUE, proceed.                                                           │
// │                                                                                     │
// │ Step 2: Concatenation                                                               │
// │   "AAA" + "AAA" = "AAAAAA" (6 characters)                                           │
// │   All A's at every position.                                                        │
// │                                                                                     │
// │ Step 3: Substring search                                                            │
// │   Looking for "AAA" in "AAAAAA"                                                     │
// │                                                                                     │
// │   i=0: concat[0..3] = "AAA" vs "AAA" → all match → FOUND!                           │
// │   Return TRUE immediately.                                                          │
// │                                                                                     │
// │ Note: Every starting position 0,1,2,3 would give "AAA" and match.                   │
// │       But we only need to find ONE match, so return at first success.               │
// └─────────────────────────────────────────────────────────────────────────────────────┘

// ═══════════════════════════════════════════════════════════════════════════════════════
// YOUR TASK: IMPLEMENT THIS FUNCTION
// ═══════════════════════════════════════════════════════════════════════════════════════
//
// fn is_cyclic_perm(s1: &str, s2: &str) -> bool
//
// INPUT:
//   s1: The original string, e.g., "ABCD" which lives at some memory address say 0x7FFE1234
//       and contains bytes [65, 66, 67, 68] in ASCII encoding.
//   s2: The candidate rotation, e.g., "CDAB" at 0x7FFE5678 with bytes [67, 68, 65, 66].
//
// OUTPUT:
//   true if s2 is a cyclic rotation of s1
//   false otherwise
//
// THE ALGORITHM YOU NEED TO IMPLEMENT:
//   1. Check if len(s1) == len(s2). If not, return false.
//   2. Concatenate s1 with itself: let concat = s1 + s1
//   3. Check if s2 is a substring of concat: return concat.contains(s2)
//
// But DO NOT just copy this! Think about:
//   - What if s1 or s2 is empty?
//   - What type should concat be? String? &str?
//   - How do you concatenate strings in Rust?
//   - Does contains() work on String or &str?
//
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Checks if s2 is a cyclic permutation (rotation) of s1.
///
/// # Arguments
/// * `s1` - The original string to check against.
///          For example, "ABCD" is stored as bytes [0x41, 0x42, 0x43, 0x44] in memory.
///          Length is measured in bytes (for ASCII) or UTF-8 code units.
///          "ABCD" has length 4 because: A(1) + B(1) + C(1) + D(1) = 4 bytes.
///
/// * `s2` - The candidate string to check if it's a rotation.
///          For example, "CDAB" stored as [0x43, 0x44, 0x41, 0x42].
///          This is "ABCD" rotated left by 2: take "AB" from front, append to back.
///          AB goes to the back: CD + AB = CDAB.
///
/// # Returns
/// * `true` if s2 can be obtained by rotating s1 some number of positions.
/// * `false` otherwise.
///
/// # Examples from numerical traces:
/// - is_cyclic_perm("ABCD", "CDAB") → true (rotation by 2)
/// - is_cyclic_perm("ABCD", "ACBD") → false (same chars, wrong order)
/// - is_cyclic_perm("ABCD", "CD") → false (different lengths: 4 ≠ 2)
/// - is_cyclic_perm("", "") → true (empty rotated is empty)
/// - is_cyclic_perm("A", "A") → true (single char rotated is same)
fn is_cyclic_perm(s1: &str, s2: &str) -> bool {
    // ┌──────────────────────────────────────────────────────────────┐
    // │ IMPLEMENTATION START                                         │
    // └──────────────────────────────────────────────────────────────┘

    // Step 1: Check lengths
    // If lengths differ, a rotation is impossible strictly by definition.
    // Example: "ABCD" (4) vs "ABC" (3) -> false
    if s1.len() != s2.len() {
        return false;
    }

    // Step 2: Handle empty strings
    // If both are empty, length check passed (0 == 0).
    // Empty is cyclic perm of empty.
    if s1.is_empty() {
        // SAFETY: We do NOT need to check s2.is_empty().
        // Why? Because Step 1 already verified s1.len() == s2.len().
        // If s1 is empty (len 0), then s2 MUST be empty (len 0).
        // If s2 was not empty, Step 1 would have returned false (0 != N).
        // Therefore, we know s2 is empty here.
        return true;
    }

    // Step 3: Concatenate s1 + s1
    // We need a new String that owns the data "s1s1".
    // "ABCD" + "ABCD" = "ABCDABCD"
    // We use repeat(2) or manually format.
    let concatenated = s1.repeat(2); 

    // Step 4: Check if s2 is a substring of concatenated
    // s1="ABCD", s2="CDAB", concatenated="ABCDABCD"
    // "CDAB" is found in "ABCDABCD" -> true
    concatenated.contains(s2)

    // ┌──────────────────────────────────────────────────────────────┐
    // │ IMPLEMENTATION END                                           │
    // └──────────────────────────────────────────────────────────────┘
}

// ─────────────────────────────────────────────────────────────────────────────
// BRUTE FORCE SOLUTION (The "Annoying Loop" Approach)
// ─────────────────────────────────────────────────────────────────────────────
// This is how you solve it WITHOUT the concatenation trick.
// It uses nested loops and modulo arithmetic.
// Time Complexity: O(N^2)
fn is_cyclic_perm_brute_force(s1: &str, s2: &str) -> bool {
    // 1. Length check is still required
    if s1.len() != s2.len() {
        return false;
    }
    if s1.is_empty() {
        return true;
    }

    let n = s1.len();
    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    // Outer loop: Try every possible rotation shift 'k' from 0 to N-1
    // let n = 4 (for example "ABCD")
    // Loop runs for k = 0, 1, 2, 3
    // k represents the "shift": how many steps we rotate left.
    // If k=0: A B C D (Shift 0)
    // If k=2: C D A B (Shift 2)
    'rotation_check: for k in 0..n {
        
        // Inner loop: Check if this rotation matches s2 completely
        // We need to verify N characters. 
        // Example: s1="ABCD", s2="CDAB", n=4, current k=2
        // We need to check indices j = 0, 1, 2, 3
        for j in 0..n {
            
            // Compare s2[j] with s1 rotated by k
            // Formula: s1[(k + j) % n]
            
            // ──────────────────────────────────────────────────────────────────────────
            // NUMERICAL CALCULATION DETOUR (Punishment Trace)
            // ──────────────────────────────────────────────────────────────────────────
            // Let s1="ABCDE" (Indices: 0=A, 1=B, 2=C, 3=D, 4=E), n=5
            // Let s2="CDEAB"
            // Current k=2 (Testing rotation by 2)
            //
            // Iteration j=0: 
            //   LHS = s2[0] = 'C' (ASCII 67)
            //   RHS Index = (k + j) % n = (2 + 0) % 5 = 2 % 5 = 2
            //   RHS = s1[2] = 'C' (ASCII 67)
            //   'C' == 'C' -> MATCH. Continue.
            //
            // Iteration j=1:
            //   LHS = s2[1] = 'D' (ASCII 68)
            //   RHS Index = (k + j) % n = (2 + 1) % 5 = 3 % 5 = 3
            //   RHS = s1[3] = 'D' (ASCII 68)
            //   'D' == 'D' -> MATCH. Continue.
            //
            // Iteration j=2:
            //   LHS = s2[2] = 'E' (ASCII 69)
            //   RHS Index = (k + j) % n = (2 + 2) % 5 = 4 % 5 = 4
            //   RHS = s1[4] = 'E' (ASCII 69)
            //   'E' == 'E' -> MATCH. Continue.
            //
            // Iteration j=3: (THE WRAP AROUND HAPPENS HERE)
            //   LHS = s2[3] = 'A' (ASCII 65)
            //   RHS Index = (k + j) % n = (2 + 3) % 5 = 5 % 5 = 0
            //   RHS = s1[0] = 'A' (ASCII 65)
            //   'A' == 'A' -> MATCH. Continue.
            //
            // Iteration j=4:
            //   LHS = s2[4] = 'B' (ASCII 66)
            //   RHS Index = (k + j) % n = (2 + 4) % 5 = 6 % 5 = 1
            //   RHS = s1[1] = 'B' (ASCII 66)
            //   'B' == 'B' -> MATCH. Continue.
            //
            // Result: All j matched for k=2. Return TRUE.
            // ──────────────────────────────────────────────────────────────────────────
            
            // ──────────────────────────────────────────────────────────────────────────
            // NUMERICAL CALCULATION HARDER TRACE (Punishment 2)
            // ──────────────────────────────────────────────────────────────────────────
            // Let s1="XYZ", s2="XZY" (Indices: 0=X, 1=Y, 2=Z), n=3
            // Testing k=1 (Rotation "YZX")
            //
            // Iteration j=0:
            //   LHS = s2[0] = 'X'
            //   RHS Index = (1 + 0) % 3 = 1
            //   RHS = s1[1] = 'Y'
            //   'X' != 'Y' -> MISMATCH! 
            //   Break inner loop immediately. Continue 'rotation_check with next k=2.
            // ──────────────────────────────────────────────────────────────────────────
            
            if s2_bytes[j] != s1_bytes[(k + j) % n] {
                // Mismatch found for this 'k'.
                // Stop checking this rotation, try next 'k'.
                continue 'rotation_check;
            }
        }
        
        // If we finished the inner loop without breaking, ALL characters matched!
        // We found a valid rotation.
        return true;
    }

    // Checked all N rotations, none matched.
    false
}

#[cfg(test)]
mod tests_brute_force {
    use super::*;

    // Re-run the key tests using the brute force function to verify it works exactly the same
    
    #[test]
    fn test_brute_force_positive() {
        assert_eq!(is_cyclic_perm_brute_force("ABCD", "CDAB"), true);
    }

    #[test]
    fn test_brute_force_negative() {
        assert_eq!(is_cyclic_perm_brute_force("ABCD", "ACBD"), false);
    }
    
    #[test]
    fn test_brute_force_trap() {
        assert_eq!(is_cyclic_perm_brute_force("ABCD", "CD"), false);
    }
    
    #[test]
    fn test_brute_force_empty() {
        assert_eq!(is_cyclic_perm_brute_force("", ""), true);
    }

    #[test]
    fn test_brute_force_wraparound() {
         // Rotation that explicitly wraps around
        assert_eq!(is_cyclic_perm_brute_force("ABCDE", "BCDEA"), true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ─────────────────────────────────────────────────────────────────
    // Test 1: Basic positive case from problem statement
    // s1 = "ABCD", s2 = "CDAB"
    // 
    // Why should this return true?
    //   "ABCD" rotated left by 2 positions:
    //     - Remove "AB" from front
    //     - Append "AB" to back
    //     - Result: "CD" + "AB" = "CDAB"
    //   So "CDAB" IS a rotation of "ABCD"
    // 
    // The concatenation approach:
    //   "ABCD" + "ABCD" = "ABCDABCD"
    //   Does "ABCDABCD" contain "CDAB"?
    //     Position 2: C-D-A-B → YES!
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_basic_positive() {
        assert_eq!(is_cyclic_perm("ABCD", "CDAB"), true);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 2: Basic negative case from problem statement
    // s1 = "ABCD", s2 = "ACBD"
    // 
    // Why should this return false?
    //   "ACBD" has the same letters as "ABCD" (A, B, C, D each once)
    //   But NO rotation of "ABCD" produces "ACBD"
    //   
    //   All rotations of "ABCD":
    //     rotate 0: ABCD (A at pos 0, B at pos 1, C at pos 2, D at pos 3)
    //     rotate 1: BCDA (B at pos 0, C at pos 1, D at pos 2, A at pos 3)
    //     rotate 2: CDAB (C at pos 0, D at pos 1, A at pos 2, B at pos 3)
    //     rotate 3: DABC (D at pos 0, A at pos 1, B at pos 2, C at pos 3)
    //   
    //   None of these equals "ACBD"
    //   "ACBD" is an ANAGRAM but not a ROTATION
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_basic_negative() {
        assert_eq!(is_cyclic_perm("ABCD", "ACBD"), false);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 3: Same string (rotation by 0)
    // s1 = "HELLO", s2 = "HELLO"
    // 
    // Why should this return true?
    //   Rotation by 0 means: take 0 chars from front, put at back.
    //   Result: original string unchanged.
    //   Every string is a rotation of itself (by 0 positions).
    // 
    // Check:
    //   "HELLO" + "HELLO" = "HELLOHELLO"
    //   Does "HELLOHELLO" contain "HELLO"?
    //   Position 0: H-E-L-L-O → YES!
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_same_string() {
        assert_eq!(is_cyclic_perm("HELLO", "HELLO"), true);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 4: Different lengths - must return false
    // s1 = "ABCD", s2 = "ABC"
    // 
    // Why should this return false?
    //   len("ABCD") = 4
    //   len("ABC") = 3
    //   4 ≠ 3
    //   
    //   A rotation preserves length. You cannot rotate a 4-char string
    //   to get a 3-char string. Impossible.
    // 
    // TRAP: If you skip the length check:
    //   "ABCD" + "ABCD" = "ABCDABCD"
    //   Does "ABCDABCD" contain "ABC"?
    //   Position 0: A-B-C → YES!
    //   Buggy code returns true. WRONG!
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_different_lengths() {
        assert_eq!(is_cyclic_perm("ABCD", "ABC"), false);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 5: Empty strings
    // s1 = "", s2 = ""
    // 
    // Why should this return true?
    //   len("") = 0, len("") = 0 → lengths match
    //   "" + "" = ""
    //   Does "" contain ""? YES (empty is substring of empty)
    //   
    //   Philosophically: rotating nothing gives nothing.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_empty_strings() {
        assert_eq!(is_cyclic_perm("", ""), true);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 6: Single character same
    // s1 = "A", s2 = "A"
    // 
    // Why should this return true?
    //   Only 1 rotation possible for single char: rotate by 0.
    //   "A" rotated by 0 = "A"
    // 
    // Check:
    //   "A" + "A" = "AA"
    //   Does "AA" contain "A"? YES at position 0.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_single_char_same() {
        assert_eq!(is_cyclic_perm("A", "A"), true);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 7: Single character different
    // s1 = "A", s2 = "B"
    // 
    // Why should this return false?
    //   len("A") = 1, len("B") = 1 → lengths match (can't short-circuit)
    //   "A" + "A" = "AA"
    //   Does "AA" contain "B"? NO!
    //   'B' (ASCII 66) is not in "AA" which only has 'A' (ASCII 65).
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_single_char_different() {
        assert_eq!(is_cyclic_perm("A", "B"), false);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 8: Rotation by 1
    // s1 = "ABCDE", s2 = "BCDEA"
    // 
    // Why should this return true?
    //   "ABCDE" rotated left by 1:
    //     - Remove 'A' from front
    //     - Put 'A' at back
    //     - Result: "BCDE" + "A" = "BCDEA"
    // 
    // Check:
    //   "ABCDE" + "ABCDE" = "ABCDEABCDE"
    //   Looking for "BCDEA" at position 1:
    //     chars 1,2,3,4,5 = B,C,D,E,A → "BCDEA" → MATCH!
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_rotation_by_one() {
        assert_eq!(is_cyclic_perm("ABCDE", "BCDEA"), true);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 9: Repeated characters - valid rotation
    // s1 = "AABB", s2 = "BBAA"
    // 
    // Why should this return true?
    //   "AABB" rotated left by 2:
    //     - Remove "AA" from front
    //     - Put "AA" at back
    //     - Result: "BB" + "AA" = "BBAA"
    // 
    // Check:
    //   "AABB" + "AABB" = "AABBAABB"
    //   Looking for "BBAA":
    //     Position 2: chars 2,3,4,5 = B,B,A,A = "BBAA" → MATCH!
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_repeated_chars_valid() {
        assert_eq!(is_cyclic_perm("AABB", "BBAA"), true);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 10: Repeated characters - invalid rotation
    // s1 = "AABB", s2 = "ABAB"
    // 
    // Why should this return false?
    //   All rotations of "AABB":
    //     rotate 0: AABB
    //     rotate 1: ABBA
    //     rotate 2: BBAA
    //     rotate 3: BAAB
    //   
    //   None equals "ABAB".
    //   "ABAB" has same chars (2 A's, 2 B's) but wrong order.
    // 
    // Check:
    //   "AABB" + "AABB" = "AABBAABB"
    //   Searching for "ABAB": not found.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_repeated_chars_invalid() {
        assert_eq!(is_cyclic_perm("AABB", "ABAB"), false);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 11: All same characters
    // s1 = "AAAA", s2 = "AAAA"
    // 
    // Why should this return true?
    //   All rotations of "AAAA" produce "AAAA".
    //   rotate 0: AAAA
    //   rotate 1: AAAA
    //   rotate 2: AAAA
    //   rotate 3: AAAA
    // 
    // Check:
    //   "AAAA" + "AAAA" = "AAAAAAAA"
    //   "AAAA" found at position 0. TRUE.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_all_same() {
        assert_eq!(is_cyclic_perm("AAAA", "AAAA"), true);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 12: s2 is substring but different length (TRAP!)
    // s1 = "ABCD", s2 = "CD"
    // 
    // Why should this return false?
    //   len("ABCD") = 4
    //   len("CD") = 2
    //   4 ≠ 2 → return false BEFORE doing any contains check!
    // 
    // TRAP:
    //   "ABCD" + "ABCD" = "ABCDABCD"
    //   "CD" IS found in "ABCDABCD" at position 2.
    //   If you skip length check, you return true. WRONG!
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_substring_trap() {
        assert_eq!(is_cyclic_perm("ABCD", "CD"), false);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 13: Two characters, valid rotation
    // s1 = "AB", s2 = "BA"
    // 
    // Why should this return true?
    //   "AB" rotated by 1:
    //     Remove 'A', put at back.
    //     Result: "B" + "A" = "BA"
    // 
    // Check:
    //   "AB" + "AB" = "ABAB"
    //   "BA" at position 1: chars 1,2 = B,A → MATCH!
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_two_char_valid() {
        assert_eq!(is_cyclic_perm("AB", "BA"), true);
    }

    // ─────────────────────────────────────────────────────────────────
    // Test 14: Rotation to end position
    // s1 = "XYZ", s2 = "ZXY"
    // 
    // Why should this return true?
    //   "XYZ" rotated by 2:
    //     Remove "XY", put at back.
    //     Result: "Z" + "XY" = "ZXY"
    // 
    // Check:
    //   "XYZ" + "XYZ" = "XYZXYZ"
    //   "ZXY" at position 2: chars 2,3,4 = Z,X,Y → MATCH!
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn test_rotation_to_end() {
        assert_eq!(is_cyclic_perm("XYZ", "ZXY"), true);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// To run tests: cargo test --bin 05_cyclic_permutation
// ═══════════════════════════════════════════════════════════════════════════════════════

fn main() {
    // Example usage:
    let s1 = "ABCD";
    let s2 = "CDAB";
    
    println!("Is '{}' a cyclic permutation of '{}'? {}", s2, s1, is_cyclic_perm(s1, s2));
    
    let s3 = "ACBD";
    println!("Is '{}' a cyclic permutation of '{}'? {}", s3, s1, is_cyclic_perm(s1, s3));
}
