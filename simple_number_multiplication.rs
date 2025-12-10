// ============================================================================
// SIMPLE NUMBER MULTIPLICATION
// ============================================================================
// PROBLEM: Multiply two simple numbers of n digits each in O(n) time
// SIMPLE NUMBER: digit repeated n times → 4444 = 4×1111, 6666 = 6×1111
// CONSTRAINT: n is ALWAYS a power of 2 (1, 2, 4, 8, 16, ...)
// ============================================================================
//
// DATA STRUCTURE BEFORE CALLING multiply_simple(4, 4, 6):
//
// ┌─────────────────────────────────────────────────────────────────────────┐
// │ INPUTS:                                                                  │
// │   n = 4 (power of 2, represents 4 digits)                               │
// │   d1 = 4 (the digit '4' repeated → 4444)                                │
// │   d2 = 6 (the digit '6' repeated → 6666)                                │
// │                                                                          │
// │ WHAT THESE REPRESENT IN ACTUAL NUMBERS:                                  │
// │   first_number  = 4444 = 4 × 1111 = 4 × (10⁴-1)/9                       │
// │   second_number = 6666 = 6 × 1111 = 6 × (10⁴-1)/9                       │
// │                                                                          │
// │ EXPECTED OUTPUT:                                                         │
// │   4444 × 6666 = 29623704                                                │
// │   As Vec<u8>: [4, 0, 7, 3, 2, 6, 9, 2] (least significant first)        │
// └─────────────────────────────────────────────────────────────────────────┘
//
// ============================================================================
// WHY THIS DIAGRAM: because you WILL forget that d1=4 means "4444" not "4"
// WHY Vec<u8>: because result can have 2n digits, doesn't fit in u64 for n≥10
// WHY least-significant-first: carry propagation goes LEFT→RIGHT naturally
// ============================================================================

/// Multiply two simple numbers.
///
/// # Arguments
/// * `n` - Number of digits in each simple number (must be power of 2)
/// * `d1` - The repeated digit of first number (1-9)
/// * `d2` - The repeated digit of second number (1-9)
///
/// # Returns
/// * Digits of the product, stored least significant digit first
///
/// # Example
/// ```
/// // multiply_simple(4, 4, 6) computes 4444 × 6666 = 29623704
/// // Returns: [4, 0, 7, 3, 2, 6, 9, 2]
/// //           ↑                    ↑
/// //          10⁰                  10⁷
/// ```
///
/// # Complexity Requirements
/// * TIME: O(n) - one digit addition/multiplication = O(1)
/// * SPACE: O(n) - storing 2n intermediate values and 2n result digits
///
// ============================================================================
// DATA STRUCTURE: REPUNIT SQUARED COEFFICIENTS
// ============================================================================
//
// FOR n=4, REPUNIT = 1111:
// 1111² = 1234321
//
// COEFFICIENT ARRAY (what you must compute):
// ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┐
// │  1  │  2  │  3  │  4  │  3  │  2  │  1  │
// └─────┴─────┴─────┴─────┴─────┴─────┴─────┘
//   ↓     ↓     ↓     ↓     ↓     ↓     ↓
//  10⁰   10¹   10²   10³   10⁴   10⁵   10⁶
//  pos=0 pos=1 pos=2 pos=3 pos=4 pos=5 pos=6
//
// FORMULA FOR COEFFICIENT AT POSITION i:
//   ┌─────────────────────────────────────────────────────────────────┐
//   │ coef[i] = min(i + 1, 2n - 1 - i)                                │
//   └─────────────────────────────────────────────────────────────────┘
//
// VERIFICATION FOR n=4 (7 positions: 0 to 6):
//   pos=0: min(0+1, 7-0) = min(1, 7) = 1 ✓
//   pos=1: min(1+1, 7-1) = min(2, 6) = 2 ✓
//   pos=2: min(2+1, 7-2) = min(3, 5) = 3 ✓
//   pos=3: min(3+1, 7-3) = min(4, 4) = 4 ✓ ← PEAK
//   pos=4: min(4+1, 7-4) = min(5, 3) = 3 ✓
//   pos=5: min(5+1, 7-5) = min(6, 2) = 2 ✓
//   pos=6: min(6+1, 7-6) = min(7, 1) = 1 ✓
//
// ============================================================================
// WHY THIS FORMULA:
// When computing 1111 × 1111 at position i:
//   contributions come from (a, b) pairs where a + b = i
//   a ∈ [0, n-1], b ∈ [0, n-1]
//   count of valid pairs = min(i+1, 2n-1-i)
// ============================================================================
//
// ============================================================================
// DATA STRUCTURE: AFTER MULTIPLYING COEFFICIENTS BY (d1 × d2)
// ============================================================================
//
// FOR n=4, d1=4, d2=6:
//   d1 × d2 = 24
//   
// BEFORE CARRY PROPAGATION:
// ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┐
// │ 24  │ 48  │ 72  │ 96  │ 72  │ 48  │ 24  │
// └─────┴─────┴─────┴─────┴─────┴─────┴─────┘
//  pos=0 pos=1 pos=2 pos=3 pos=4 pos=5 pos=6
//
// AFTER CARRY PROPAGATION (right to left, i.e., pos=0 first):
//   pos=0: val=24, carry_in=0 → 24/10=2 rem 4 → digit=4, carry=2
//   pos=1: val=48, carry_in=2 → 50/10=5 rem 0 → digit=0, carry=5
//   pos=2: val=72, carry_in=5 → 77/10=7 rem 7 → digit=7, carry=7
//   pos=3: val=96, carry_in=7 → 103/10=10 rem 3 → digit=3, carry=10
//   pos=4: val=72, carry_in=10 → 82/10=8 rem 2 → digit=2, carry=8
//   pos=5: val=48, carry_in=8 → 56/10=5 rem 6 → digit=6, carry=5
//   pos=6: val=24, carry_in=5 → 29/10=2 rem 9 → digit=9, carry=2
//   pos=7: val=0, carry_in=2 → digit=2, carry=0
//
// FINAL RESULT:
// ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐
// │  4  │  0  │  7  │  3  │  2  │  6  │  9  │  2  │
// └─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘
//  10⁰   10¹   10²   10³   10⁴   10⁵   10⁶   10⁷
//
// READ LEFT→RIGHT in reverse: 29623704 ✓
//
// ============================================================================

fn multiply_simple(n: usize, d1: u8, d2: u8) -> Vec<u8> {
    // ========================================================================
    // STEP 1: COMPUTE d1 × d2
    // ========================================================================
    // EXAMPLE: d1=4, d2=6 → 4×6=24
    // EXAMPLE: d1=9, d2=9 → 9×9=81 (maximum possible)
    // EXAMPLE: d1=1, d2=1 → 1×1=1 (minimum non-zero)
    //
    // WHY u16: because 9×9=81 fits in u8, but later we add carries
    //          and 81×4 + carry could exceed 255
    // ========================================================================
    
    // TODO: YOUR CODE HERE
    // TRAP: What if d1=0 or d2=0? Is 0000 a valid simple number?
    
    // ========================================================================
    // STEP 2: GENERATE REPUNIT² COEFFICIENTS
    // ========================================================================
    // FOR n=4: generate [1, 2, 3, 4, 3, 2, 1] (length = 2n-1 = 7)
    // FOR n=8: generate [1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1] (length = 15)
    //
    // FORMULA: coef[i] = min(i + 1, 2n - 1 - i)
    //
    // NUMERICAL WALK:
    //   i=0: min(1, 2n-1) = 1 (always 1 at start)
    //   i=n-1: min(n, n) = n (peak)
    //   i=2n-2: min(2n-1, 1) = 1 (always 1 at end)
    //
    // TRAP: Array has 2n-1 elements, NOT n elements!
    // TRAP: For n=16, coef[15]=16, which causes carry during final multiply
    // ========================================================================
    
    // TODO: YOUR CODE HERE
    // TRAP: off-by-one in loop bounds
    // TRAP: using n instead of 2n-1 as array size
    
    // ========================================================================
    // STEP 3: MULTIPLY EACH COEFFICIENT BY (d1 × d2)
    // ========================================================================
    // FOR n=4, d1×d2=24:
    //   [1, 2, 3, 4, 3, 2, 1] × 24 = [24, 48, 72, 96, 72, 48, 24]
    //
    // NUMERICAL CALCULATION:
    //   1 × 24 = 24
    //   2 × 24 = 48
    //   3 × 24 = 72
    //   4 × 24 = 96
    //   3 × 24 = 72
    //   2 × 24 = 48
    //   1 × 24 = 24
    //
    // TRAP: 96 > 99 means TWO-DIGIT intermediate, needs carry handling
    // TRAP: For n=16, d1=9, d2=9: max_coef × (d1×d2) = 16 × 81 = 1296
    //       This is a 4-digit number at one position!
    // ========================================================================
    
    // TODO: YOUR CODE HERE
    
    // ========================================================================
    // STEP 4: CARRY PROPAGATION
    // ========================================================================
    // BEFORE: [24, 48, 72, 96, 72, 48, 24]
    // PROCESS from position 0 to position 2n-1 (left to right):
    //
    // DETAILED TRACE:
    //   pos=0: total = 24 + 0 = 24
    //          digit = 24 % 10 = 4
    //          carry = 24 / 10 = 2
    //   pos=1: total = 48 + 2 = 50
    //          digit = 50 % 10 = 0
    //          carry = 50 / 10 = 5
    //   pos=2: total = 72 + 5 = 77
    //          digit = 77 % 10 = 7
    //          carry = 77 / 10 = 7
    //   pos=3: total = 96 + 7 = 103
    //          digit = 103 % 10 = 3
    //          carry = 103 / 10 = 10
    //   pos=4: total = 72 + 10 = 82
    //          digit = 82 % 10 = 2
    //          carry = 82 / 10 = 8
    //   pos=5: total = 48 + 8 = 56
    //          digit = 56 % 10 = 6
    //          carry = 56 / 10 = 5
    //   pos=6: total = 24 + 5 = 29
    //          digit = 29 % 10 = 9
    //          carry = 29 / 10 = 2
    //   pos=7: total = 0 + 2 = 2
    //          digit = 2 % 10 = 2
    //          carry = 2 / 10 = 0  ← STOP when carry=0
    //
    // RESULT: [4, 0, 7, 3, 2, 6, 9, 2] (8 digits, which is 2n for n=4)
    //
    // TRAP: Final carry may add extra digits beyond 2n-1
    // TRAP: Must continue while carry > 0, not just until position 2n-1
    // ========================================================================
    
    // TODO: YOUR CODE HERE
    
    let product : u16 = (d1 as u16) * (d2 as u16);
    let num_coffs = 2*n -1;
    let mut vals: Vec<u32>  = vec![0; num_coffs];
    for i in 0..num_coffs {
        let coef = std::cmp::min(i+1, num_coffs - i );
        vals[i] = (coef as u32) * (product as u32);
    }

    let mut result :Vec<u8> = Vec::new();
    let mut carry :u32 = 0;

    for &val in vals.iter()
    {
        let total = val + carry; 
        let digit = (total %10) as u8;
        carry = total /10; 
        result.push(digit);
    }
    while carry > 0 
    {
        let digit = ( carry % 10) as u8;
        carry = carry /10; 
        result.push(digit);
    }

    result

}

/// Helper function to convert Vec<u8> (LSB first) to readable string
fn digits_to_string(digits: &[u8]) -> String {
    // ========================================================================
    // EXAMPLE: [4, 0, 7, 3, 2, 6, 9, 2] → "29623704"
    // 
    // STEP 1: Reverse to get MSB first: [2, 9, 6, 2, 3, 7, 0, 4]
    // STEP 2: Convert each digit to char: ['2', '9', '6', '2', '3', '7', '0', '4']
    // STEP 3: Collect to string: "29623704"
    //
    // TRAP: Leading zeros! [0, 0, 1] should become "100" not "001"
    //       After reverse: [1, 0, 0] → "100" ✓ (no issue here)
    //
    // TRAP: Empty vec! [] should become "0" not ""
    // TRAP: All zeros! [0, 0, 0] after reverse is [0, 0, 0] → strip to "0"
    // ========================================================================
    
    if digits.is_empty() {
        return "0".to_string();
    }
    
    digits.iter()
        .rev()
        .map(|&d| char::from_digit(d as u32, 10).unwrap())
        .collect::<String>()
        .trim_start_matches('0')
        .to_string()
        .chars()
        .next()
        .map_or("0".to_string(), |_| {
            digits.iter()
                .rev()
                .map(|&d| char::from_digit(d as u32, 10).unwrap())
                .collect::<String>()
                .trim_start_matches('0')
                .to_string()
        })
}

/// Helper to construct the actual simple number for verification
fn build_simple_number(n: usize, d: u8) -> u64 {
    // ========================================================================
    // EXAMPLE: build_simple_number(4, 4) → 4444
    //
    // CALCULATION:
    //   result = 0
    //   iteration 1: result = 0 × 10 + 4 = 4
    //   iteration 2: result = 4 × 10 + 4 = 44
    //   iteration 3: result = 44 × 10 + 4 = 444
    //   iteration 4: result = 444 × 10 + 4 = 4444
    //
    // TRAP: Overflow for n ≥ 20 with d=9: 99999999999999999999 > u64::MAX
    // ========================================================================
    
    let mut result = 0u64;
    for _ in 0..n {
        result = result * 10 + d as u64;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // TEST CASE 1: Basic multiplication n=2
    // ========================================================================
    // 22 × 33 = 726
    //
    // BREAKDOWN:
    //   d1 = 2, d2 = 3, d1×d2 = 6
    //   REPUNIT = 11, REPUNIT² = 121
    //   6 × 121 = 726
    //
    // VERIFY: 22 × 33 = 726 ✓
    // ========================================================================
    #[test]
    fn test_n2_basic() {
        let result = multiply_simple(2, 2, 3);
        let result_str = digits_to_string(&result);
        assert_eq!(result_str, "726");
        
        // Cross-verify with direct multiplication
        let expected = build_simple_number(2, 2) * build_simple_number(2, 3);
        assert_eq!(expected, 726);
    }

    // ========================================================================
    // TEST CASE 2: The example from problem statement n=4
    // ========================================================================
    // 4444 × 6666 = 29623704
    //
    // BREAKDOWN:
    //   d1 = 4, d2 = 6, d1×d2 = 24
    //   REPUNIT = 1111, REPUNIT² = 1234321
    //   24 × 1234321 = 29623704
    //
    // DETAILED CARRY TRACE:
    //   coefs after ×24: [24, 48, 72, 96, 72, 48, 24]
    //   pos0: 24→4 carry=2
    //   pos1: 48+2=50→0 carry=5
    //   pos2: 72+5=77→7 carry=7
    //   pos3: 96+7=103→3 carry=10
    //   pos4: 72+10=82→2 carry=8
    //   pos5: 48+8=56→6 carry=5
    //   pos6: 24+5=29→9 carry=2
    //   pos7: 0+2=2→2 carry=0
    //   RESULT: [4,0,7,3,2,6,9,2] → 29623704
    // ========================================================================
    #[test]
    fn test_n4_problem_example() {
        let result = multiply_simple(4, 4, 6);
        let result_str = digits_to_string(&result);
        assert_eq!(result_str, "29623704");
        
        let expected = build_simple_number(4, 4) * build_simple_number(4, 6);
        assert_eq!(expected, 29623704);
    }

    // ========================================================================
    // TEST CASE 3: Perfect square n=4
    // ========================================================================
    // 5555 × 5555 = 30858025
    //
    // BREAKDOWN:
    //   d1 = 5, d2 = 5, d1×d2 = 25
    //   REPUNIT² = 1234321
    //   25 × 1234321 = 30858025
    //
    // VERIFY: 5555² = 30858025 ✓
    // ========================================================================
    #[test]
    fn test_n4_perfect_square() {
        let result = multiply_simple(4, 5, 5);
        let result_str = digits_to_string(&result);
        assert_eq!(result_str, "30858025");
    }

    // ========================================================================
    // TEST CASE 4: Maximum digits n=4
    // ========================================================================
    // 9999 × 9999 = 99980001
    //
    // BREAKDOWN:
    //   d1 = 9, d2 = 9, d1×d2 = 81
    //   coefs: [1, 2, 3, 4, 3, 2, 1]
    //   ×81:   [81, 162, 243, 324, 243, 162, 81]
    //
    // CARRY TRACE:
    //   pos0: 81→1 carry=8
    //   pos1: 162+8=170→0 carry=17
    //   pos2: 243+17=260→0 carry=26
    //   pos3: 324+26=350→0 carry=35
    //   pos4: 243+35=278→8 carry=27
    //   pos5: 162+27=189→9 carry=18
    //   pos6: 81+18=99→9 carry=9
    //   pos7: 0+9=9→9 carry=0
    //   RESULT: [1,0,0,0,8,9,9,9] → 99980001
    //
    // VERIFY: 9999² = 99980001 ✓
    // ========================================================================
    #[test]
    fn test_n4_max_digits() {
        let result = multiply_simple(4, 9, 9);
        let result_str = digits_to_string(&result);
        assert_eq!(result_str, "99980001");
    }

    // ========================================================================
    // TEST CASE 5: Minimum n=1
    // ========================================================================
    // 7 × 8 = 56
    //
    // BREAKDOWN:
    //   n = 1, so REPUNIT = 1, REPUNIT² = 1
    //   d1×d2 = 56
    //   56 × 1 = 56
    // ========================================================================
    #[test]
    fn test_n1_single_digit() {
        let result = multiply_simple(1, 7, 8);
        let result_str = digits_to_string(&result);
        assert_eq!(result_str, "56");
    }

    // ========================================================================
    // TEST CASE 6: n=1 with carry
    // ========================================================================
    // 9 × 9 = 81
    //
    // BREAKDOWN:
    //   REPUNIT = 1, REPUNIT² = 1
    //   81 × 1 = 81
    //   Single position value 81 → digit=1, carry=8 → final [1, 8] → "81"
    // ========================================================================
    #[test]
    fn test_n1_max() {
        let result = multiply_simple(1, 9, 9);
        let result_str = digits_to_string(&result);
        assert_eq!(result_str, "81");
    }

    // ========================================================================
    // TEST CASE 7: n=8 larger case
    // ========================================================================
    // 22222222 × 33333333 = 740740725925926
    //
    // THIS IS THE TEST THAT WILL BREAK NAIVE O(n²) IMPLEMENTATIONS
    //
    // BREAKDOWN:
    //   d1 = 2, d2 = 3, d1×d2 = 6
    //   REPUNIT = 11111111 (8 ones)
    //   REPUNIT² = 123456787654321 (15 digits)
    //   6 × 123456787654321 = 740740725925926
    //
    // REPUNIT² COEFFICIENTS for n=8:
    //   [1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1]
    //
    // VERIFY: 22222222 × 33333333 = 740740725925926 ✓
    // ========================================================================
    #[test]
    fn test_n8_large() {
        let result = multiply_simple(8, 2, 3);
        let result_str = digits_to_string(&result);
        assert_eq!(result_str, "740740725925926");
        
        let expected = build_simple_number(8, 2) * build_simple_number(8, 3);
        assert_eq!(expected, 740740725925926);
    }

    // ========================================================================
    // TEST CASE 8: n=8 maximum
    // ========================================================================
    // 99999999 × 99999999 = 9999999800000001
    //
    // BREAKDOWN:
    //   d1×d2 = 81
    //   REPUNIT² coefs: [1,2,3,4,5,6,7,8,7,6,5,4,3,2,1]
    //   ×81: [81,162,243,324,405,486,567,648,567,486,405,324,243,162,81]
    //
    // This test will expose carry propagation bugs in larger scale!
    // ========================================================================
    #[test]
    fn test_n8_max() {
        let result = multiply_simple(8, 9, 9);
        let result_str = digits_to_string(&result);
        assert_eq!(result_str, "9999999800000001");
        
        let expected = build_simple_number(8, 9) * build_simple_number(8, 9);
        assert_eq!(expected, 9999999800000001);
    }

    // ========================================================================
    // TEST CASE 9: Asymmetric digits n=4
    // ========================================================================
    // 1111 × 9999 = 11108889
    //
    // BREAKDOWN:
    //   d1 = 1, d2 = 9, d1×d2 = 9
    //   REPUNIT² = 1234321
    //   9 × 1234321 = 11108889
    //
    // VERIFY: 1111 × 9999 = 11108889 ✓
    // ========================================================================
    #[test]
    fn test_n4_asymmetric() {
        let result = multiply_simple(4, 1, 9);
        let result_str = digits_to_string(&result);
        assert_eq!(result_str, "11108889");
        
        let expected = build_simple_number(4, 1) * build_simple_number(4, 9);
        assert_eq!(expected, 11108889);
    }

    // ========================================================================
    // TEST CASE 10: n=2 all combinations stress test
    // ========================================================================
    // Tests all 81 combinations of d1, d2 from 1-9 for n=2
    // Verifies correctness against naive multiplication
    // ========================================================================
    #[test]
    fn test_n2_all_combinations() {
        for d1 in 1..=9u8 {
            for d2 in 1..=9u8 {
                let result = multiply_simple(2, d1, d2);
                let result_str = digits_to_string(&result);
                
                let expected = build_simple_number(2, d1) * build_simple_number(2, d2);
                
                assert_eq!(
                    result_str, 
                    expected.to_string(),
                    "Failed for {}{}×{}{}: got {}, expected {}",
                    d1, d1, d2, d2, result_str, expected
                );
            }
        }
    }

    // ========================================================================
    // TEST CASE 11: n=4 all combinations stress test
    // ========================================================================
    #[test]
    fn test_n4_all_combinations() {
        for d1 in 1..=9u8 {
            for d2 in 1..=9u8 {
                let result = multiply_simple(4, d1, d2);
                let result_str = digits_to_string(&result);
                
                let expected = build_simple_number(4, d1) * build_simple_number(4, d2);
                
                assert_eq!(
                    result_str, 
                    expected.to_string(),
                    "Failed for {}{}{}{}×{}{}{}{}: got {}, expected {}",
                    d1, d1, d1, d1, d2, d2, d2, d2, result_str, expected
                );
            }
        }
    }
}
