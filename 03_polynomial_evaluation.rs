// ╔═══════════════════════════════════════════════════════════════════════════════╗
// ║                    POLYNOMIAL EVALUATION - BOILERPLATE                        ║
// ║                        NO SOLUTION IMPLEMENTED YET                            ║
// ╚═══════════════════════════════════════════════════════════════════════════════╝
//
// PROBLEM: Given coefficients [c0, c1, c2, ..., cn] and value x, compute:
//          p(x) = cn*x^n + c(n-1)*x^(n-1) + ... + c2*x^2 + c1*x + c0
//
// ┌────────────────────────────────────────────────────────────────────────────────┐
// │ ARRAY REPRESENTATION EXAMPLE:                                                 │
// │                                                                               │
// │ Polynomial: 2x^3 + 3x^2 + 4x + 5                                              │
// │                                                                               │
// │ coef array in memory:                                                         │
// │ ┌─────────┬─────────┬─────────┬─────────┐                                     │
// │ │ index 0 │ index 1 │ index 2 │ index 3 │                                     │
// │ │    5    │    4    │    3    │    2    │                                     │
// │ │   c0    │   c1    │   c2    │   c3    │                                     │
// │ │ coeff   │ coeff   │ coeff   │ coeff   │                                     │
// │ │ of x^0  │ of x^1  │ of x^2  │ of x^3  │                                     │
// │ └─────────┴─────────┴─────────┴─────────┘                                     │
// │      ↓         ↓         ↓         ↓                                          │
// │    5*x^0    4*x^1    3*x^2    2*x^3                                           │
// │   meaning:  coef[i] ALWAYS pairs with x^i                                    │
// │                                                                               │
// │ AT x = 10:                                                                    │
// │   coef[0] * 10^0 = 5 * 1     = 5                                              │
// │   coef[1] * 10^1 = 4 * 10    = 40                                             │
// │   coef[2] * 10^2 = 3 * 100   = 300                                            │
// │   coef[3] * 10^3 = 2 * 1000  = 2000                                           │
// │   ─────────────────────────────────                                           │
// │   TOTAL = 5 + 40 + 300 + 2000 = 2345                                          │
// │                                                                               │
// │ WHY coef[i] pairs with x^i:                                                   │
// │   The problem statement says the polynomial is cn*x^n + ... + c0              │
// │   The array stores [c0, c1, c2, ..., cn] from index 0 to n                    │
// │   So index 0 has c0 which multiplies x^0                                      │
// │   And index n has cn which multiplies x^n                                     │
// │   Therefore coef[i] * x^i is the correct pairing                              │
// │   NOT coef[i] * x^(n-i) which would reverse the polynomial                    │
// │                                                                               │
// │ NUMERICAL VERIFICATION for x = 10:                                            │
// │   Using paper notation: 2(10)^3 + 3(10)^2 + 4(10) + 5                         │
// │   = 2(1000) + 3(100) + 4(10) + 5                                              │
// │   = 2000 + 300 + 40 + 5                                                       │
// │   = 2345 ✓ matches our loop calculation                                       │
// └────────────────────────────────────────────────────────────────────────────────┘

// ┌────────────────────────────────────────────────────────────────────────────────┐
// │ EDGE CASE: x = 0                                                              │
// │                                                                               │
// │ Polynomial: 7x^3 + 4x^2 + 9x + 6                                              │
// │ coef = [6, 9, 4, 7]                                                           │
// │                                                                               │
// │ AT x = 0:                                                                     │
// │   coef[0] * 0^0 = 6 * 1 = 6   ← TRAP: What is 0^0?                            │
// │   coef[1] * 0^1 = 9 * 0 = 0   ← Any positive power of 0 is 0                  │
// │   coef[2] * 0^2 = 4 * 0 = 0                                                   │
// │   coef[3] * 0^3 = 7 * 0 = 0                                                   │
// │   ─────────────────────────                                                   │
// │   TOTAL = 6                                                                   │
// │                                                                               │
// │ WHY 0^0 = 1 in this context:                                                  │
// │   Mathematically, 0^0 is "indeterminate form"                                 │
// │   But for polynomials, limt→0 of x^0 = 1 for any x including 0               │
// │   In Rust: (0_i64).pow(0) = 1                                                 │
// │   So we treat x^0 = 1 always, which makes the constant term survive           │
// │                                                                               │
// │ YOUR BUG: If you manually compute power with a loop starting at 1,            │
// │   your loop for i=0 might give power=1 (correct) or power=x (wrong)           │
// │   depending on whether you multiply BEFORE or AFTER incrementing              │
// └────────────────────────────────────────────────────────────────────────────────┘

// ┌────────────────────────────────────────────────────────────────────────────────┐
// │ EDGE CASE: x = -2                                                             │
// │                                                                               │
// │ Polynomial: x^3 + x^2 + x + 1                                                 │
// │ coef = [1, 1, 1, 1]                                                           │
// │                                                                               │
// │ AT x = -2:                                                                    │
// │   (-2)^0 = 1              ← any number to power 0 is 1                        │
// │   (-2)^1 = -2             ← odd power, negative result CRITICAL               │
// │   (-2)^2 = (-2)*(-2) = 4  ← even power, positive result                       │
// │   (-2)^3 = 4*(-2) = -8    ← odd power, negative result CRITICAL               │
// │                                                                               │
// │ TERM BY TERM:                                                                 │
// │   coef[0] * (-2)^0 = 1 * 1    = 1                                             │
// │   coef[1] * (-2)^1 = 1 * (-2) = -2                                            │
// │   coef[2] * (-2)^2 = 1 * 4    = 4                                             │
// │   coef[3] * (-2)^3 = 1 * (-8) = -8                                            │
// │   ─────────────────────────────────                                           │
// │   TOTAL = 1 + (-2) + 4 + (-8) = -5                                            │
// │                                                                               │
// │ YOUR BUG: If you compute power with x.abs() and then "fix" the sign,          │
// │   you might apply the sign incorrectly or forget that the parity of i         │
// │   determines the sign, not some other logic                                   │
// │                                                                               │
// │ SIGN RULE: (-2)^i = { positive if i is even, negative if i is odd }           │
// │            i=0: even → positive 1                                             │
// │            i=1: odd → negative -2                                             │
// │            i=2: even → positive 4                                             │
// │            i=3: odd → negative -8                                             │
// └────────────────────────────────────────────────────────────────────────────────┘

// ┌────────────────────────────────────────────────────────────────────────────────┐
// │ MIDDLE CALCULATION: n = 5, x = 10                                             │
// │                                                                               │
// │ Polynomial: 1x^5 + 2x^4 + 3x^3 + 4x^2 + 5x + 6                                │
// │ coef = [6, 5, 4, 3, 2, 1]  ← NOTICE: coef.len() = 6, but n = 5                │
// │                                                                               │
// │ THE OFF-BY-ONE TRAP:                                                          │
// │   n = degree = highest power = 5                                              │
// │   array length = n + 1 = 6                                                    │
// │   array indices = 0, 1, 2, 3, 4, 5 (six of them)                              │
// │                                                                               │
// │ LOOP OPTIONS:                                                                 │
// │   for i in 0..n   → goes 0,1,2,3,4 (5 iterations, MISSING index 5!)           │
// │   for i in 0..=n  → goes 0,1,2,3,4,5 (6 iterations, CORRECT)                  │
// │   for i in 0..n+1 → goes 0,1,2,3,4,5 (6 iterations, CORRECT but ugly)         │
// │                                                                               │
// │ CALCULATION WITH CORRECT LOOP (0..=5):                                        │
// │   i=0: coef[0] * 10^0 = 6 * 1         = 6                                     │
// │   i=1: coef[1] * 10^1 = 5 * 10        = 50                                    │
// │   i=2: coef[2] * 10^2 = 4 * 100       = 400                                   │
// │   i=3: coef[3] * 10^3 = 3 * 1000      = 3000                                  │
// │   i=4: coef[4] * 10^4 = 2 * 10000     = 20000                                 │
// │   i=5: coef[5] * 10^5 = 1 * 100000    = 100000                                │
// │   ─────────────────────────────────────────────                               │
// │   TOTAL = 6 + 50 + 400 + 3000 + 20000 + 100000 = 123456                       │
// │                                                                               │
// │ CALCULATION WITH WRONG LOOP (0..5):                                           │
// │   i=0: 6 * 1 = 6                                                              │
// │   i=1: 5 * 10 = 50                                                            │
// │   i=2: 4 * 100 = 400                                                          │
// │   i=3: 3 * 1000 = 3000                                                        │
// │   i=4: 2 * 10000 = 20000                                                      │
// │   (i=5 NEVER RUNS)                                                            │
// │   TOTAL = 23456 ← WRONG! Missing 100000                                       │
// │                                                                               │
// │ DIFFERENCE: 123456 - 23456 = 100000 = coef[5] * 10^5 = the missing term       │
// └────────────────────────────────────────────────────────────────────────────────┘

// ┌────────────────────────────────────────────────────────────────────────────────┐
// │ HARDER CALCULATION: n = 4, x = 3, with negative coefficients                  │
// │                                                                               │
// │ Polynomial: 2x^4 - 3x^3 + x^2 - 5x + 7                                        │
// │ coef = [7, -5, 1, -3, 2]                                                      │
// │                                                                               │
// │ POWER TABLE for x = 3:                                                        │
// │   3^0 = 1                                                                     │
// │   3^1 = 3                                                                     │
// │   3^2 = 3 * 3 = 9                                                             │
// │   3^3 = 9 * 3 = 27                                                            │
// │   3^4 = 27 * 3 = 81                                                           │
// │                                                                               │
// │ TERM BY TERM:                                                                 │
// │   coef[0] * 3^0 = 7 * 1     = 7                                               │
// │   coef[1] * 3^1 = (-5) * 3  = -15                                             │
// │   coef[2] * 3^2 = 1 * 9     = 9                                               │
// │   coef[3] * 3^3 = (-3) * 27 = -81                                             │
// │   coef[4] * 3^4 = 2 * 81    = 162                                             │
// │   ─────────────────────────────────                                           │
// │   TOTAL = 7 + (-15) + 9 + (-81) + 162                                         │
// │         = 7 - 15 + 9 - 81 + 162                                               │
// │         = (7 + 9 + 162) - (15 + 81)                                           │
// │         = 178 - 96                                                            │
// │         = 82                                                                  │
// │                                                                               │
// │ VERIFICATION using paper notation:                                            │
// │   2(81) - 3(27) + 1(9) - 5(3) + 7                                             │
// │   = 162 - 81 + 9 - 15 + 7                                                     │
// │   = 162 + 9 + 7 - 81 - 15                                                     │
// │   = 178 - 96 = 82 ✓                                                           │
// └────────────────────────────────────────────────────────────────────────────────┘

// ┌────────────────────────────────────────────────────────────────────────────────┐
// │ OVERFLOW TRAP: n = 9, x = 10                                                  │
// │                                                                               │
// │ 10^9 = 1,000,000,000 = 1 billion                                              │
// │ i32::MAX = 2,147,483,647 ≈ 2.1 billion                                        │
// │                                                                               │
// │ So 10^9 fits in i32, but barely.                                              │
// │                                                                               │
// │ If coefficient is 3:                                                          │
// │   3 * 10^9 = 3,000,000,000 > i32::MAX                                         │
// │   OVERFLOW!                                                                   │
// │                                                                               │
// │ 10^10 = 10,000,000,000 > i32::MAX                                             │
// │ So for n >= 10, x = 10, even x^n alone overflows i32                          │
// │                                                                               │
// │ SOLUTION: Use i64 or f64 for intermediate calculations                        │
// │   i64::MAX = 9,223,372,036,854,775,807 ≈ 9.2 * 10^18                          │
// │   This handles 10^18 comfortably                                              │
// │                                                                               │
// │ YOUR BUG: You declared everything as i32 because the function signature said  │
// │   int, and now 10^10 wraps around to garbage                                  │
// └────────────────────────────────────────────────────────────────────────────────┘

// ┌────────────────────────────────────────────────────────────────────────────────┐
// │ HORNER'S METHOD TRACE: The Efficient Approach                                 │
// │                                                                               │
// │ Polynomial: 2x^3 + 3x^2 + 4x + 5 at x = 10                                    │
// │ coef = [5, 4, 3, 2]  (n = 3)                                                  │
// │                                                                               │
// │ FACTORED FORM:                                                                │
// │   p(x) = ((2x + 3)x + 4)x + 5                                                 │
// │                                                                               │
// │ Start from the inside (highest power coefficient):                            │
// │   result = coef[3] = 2                                                        │
// │   ─────────────────────────────────────────────                               │
// │   i = 2: result = result * x + coef[2]                                        │
// │          result = 2 * 10 + 3 = 20 + 3 = 23                                    │
// │   ─────────────────────────────────────────────                               │
// │   i = 1: result = result * x + coef[1]                                        │
// │          result = 23 * 10 + 4 = 230 + 4 = 234                                 │
// │   ─────────────────────────────────────────────                               │
// │   i = 0: result = result * x + coef[0]                                        │
// │          result = 234 * 10 + 5 = 2340 + 5 = 2345                              │
// │                                                                               │
// │ FINAL RESULT = 2345 ✓ (matches naive approach)                                │
// │                                                                               │
// │ LOOP DIRECTION:                                                               │
// │   Start at i = n (coef[n])                                                    │
// │   End at i = 0 (coef[0])                                                      │
// │   So: for i in (0..=n).rev() skipping the first iteration                     │
// │   Or: initialize with coef[n], then for i in (0..n).rev()                     │
// │                                                                               │
// │ YOUR CONFUSION:                                                               │
// │   (0..n).rev() = n-1, n-2, ..., 0 (not including n!)                          │
// │   (0..=n).rev() = n, n-1, ..., 0 (including n)                                │
// │                                                                               │
// │ If you initialize result = coef[n] first, then loop (0..n).rev():             │
// │   result starts as coef[3] = 2                                                │
// │   i=2: result = 2*10 + coef[2] = 20 + 3 = 23                                  │
// │   i=1: result = 23*10 + coef[1] = 230 + 4 = 234                               │
// │   i=0: result = 234*10 + coef[0] = 2340 + 5 = 2345 ✓                          │
// │                                                                               │
// │ WHY HORNER IS EFFICIENT:                                                      │
// │   Naive: computes x^0, x^1, x^2, x^3 separately = O(n^2) multiplications      │
// │          x^0=1 (0 mult), x^1=x (0), x^2=x*x (1), x^3=x*x*x (2)                │
// │          Total: 0+0+1+2 = 3 power multiplications + 4 coef multiplications    │
// │   Horner: one multiply-and-add per coefficient = O(n) multiplications         │
// │          3 mult + 3 add = 6 operations total for degree 3                     │
// └────────────────────────────────────────────────────────────────────────────────┘

/// Evaluates a polynomial at a given point x.
/// 
/// # Arguments
/// * `coef` - Slice of coefficients where coef[i] is the coefficient of x^i
///            
///            ┌─────────┬─────────┬─────────┬─────────┐
///            │ coef[0] │ coef[1] │ coef[2] │ coef[n] │
///            │   c0    │   c1    │   c2    │   cn    │
///            │  x^0    │  x^1    │  x^2    │  x^n    │
///            └─────────┴─────────┴─────────┴─────────┘
///            
///            Example: For 2x^3 + 3x^2 + 4x + 5
///            coef = [5, 4, 3, 2]
///            coef[0]=5 is coefficient of x^0 (constant term)
///            coef[3]=2 is coefficient of x^3 (highest power term)
///            
/// * `x` - The point at which to evaluate the polynomial
///            
///            Example values and their trap potential:
///            x = 0  → all x^i terms die except x^0 = 1, result = coef[0]
///            x = 1  → all x^i = 1, result = sum of all coefficients
///            x = -2 → odd powers negative, even powers positive
///            x = 10 → easy to verify, each term is obvious
///            
/// * `n` - The degree of the polynomial (highest power)
///            
///            CRITICAL: If n = 3, there are 4 coefficients (indices 0,1,2,3)
///            coef.len() should be n + 1
///            Loop should run from 0 to n INCLUSIVE: for i in 0..=n
///            
/// # Returns
/// The value of the polynomial: cn*x^n + c(n-1)*x^(n-1) + ... + c1*x + c0
/// 
/// # Example Calculation (manual trace for verification)
/// 
/// For coef = [5, 4, 3, 2], x = 10, n = 3:
/// 
/// p(10) = 2*10^3 + 3*10^2 + 4*10^1 + 5*10^0
///       = 2*1000 + 3*100 + 4*10 + 5*1
///       = 2000 + 300 + 40 + 5
///       = 2345
///
pub fn eval_polynom(coef: &[i64], x: i64, n: usize) -> i64 {
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ YOUR IMPLEMENTATION GOES HERE                                               │
    // │                                                                             │
    // │ BEFORE YOU WRITE ANYTHING, TRACE THESE EXAMPLES BY HAND:                    │
    // │                                                                             │
    // │ Example 1: coef = [5, 4, 3, 2], x = 10, n = 3                                │
    // │   Expected: 2345                                                            │
    // │   Your trace:                                                               │
    // │   i=0: coef[0] * x^0 = 5 * 1 = ?                                            │
    // │   i=1: coef[1] * x^1 = 4 * 10 = ?                                           │
    // │   i=2: coef[2] * x^2 = 3 * 100 = ?                                          │
    // │   i=3: coef[3] * x^3 = 2 * 1000 = ?                                         │
    // │   Sum = ?                                                                   │
    // │                                                                             │
    // │ Example 2: coef = [6, 9, 4, 7], x = 0, n = 3                                 │
    // │   Expected: 6 (only constant term survives)                                 │
    // │   Your trace: ...                                                           │
    // │                                                                             │
    // │ Example 3: coef = [1, 1, 1, 1], x = -2, n = 3                                │
    // │   Expected: -5                                                              │
    // │   Your trace (watch the signs!): ...                                        │
    // └──────────────────────────────────────────────────────────────────────────────┘
    
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ HORNER'S METHOD IMPLEMENTATION (The Efficient Loop)                         │
    // └──────────────────────────────────────────────────────────────────────────────┘

    let mut res = coef[n];     // INITIALIZATION STEP: Start with the highest power coefficient.
                               // -----------------------------------------------------------
                               // TRACE (Example: coef = [5, 4, 3, 2], x = 10, n = 3)
                               // We grab coef[3] because n=3. 
                               // coef[3] is the number 2.
                               // So 'res' starts as 2.
                               //
                               // MATHEMATICAL MEANING:
                               // We are holding the coefficient for x^3.
                               // Right now, it's just "2". We haven't multiplied it by x yet.
                               // The "snowball" is at the top of the hill, size 2.

    for i in (0..n).rev()      // LOOP DIRECTION: DOWNWARDS (n-1 down to 0)
    {                          // ----------------------------------------
                               // TRACE INDICES:
                               // n=3, so range is 0..3 (0, 1, 2)
                               // rev() flips it: 2, then 1, then 0.
                               //
                               // ITERATION 1 (i = 2):
                               //   Target: process x^2 term.
                               //   res (old) = 2
                               //   coef[2] = 3
                               //   res = res * x + coef[i]
                               //       = 2 * 10 + 3
                               //       = 20 + 3 = 23
                               //   Math: We formed (2x + 3)
                               //
                               // ITERATION 2 (i = 1):
                               //   Target: process x^1 term.
                               //   res (old) = 23
                               //   coef[1] = 4
                               //   res = res * x + coef[i]
                               //       = 23 * 10 + 4
                               //       = 230 + 4 = 234
                               //   Math: We formed ((2x + 3)x + 4) = 2x^2 + 3x + 4
                               //
                               // ITERATION 3 (i = 0):
                               //   Target: process x^0 (constant) term.
                               //   res (old) = 234
                               //   coef[0] = 5
                               //   res = res * x + coef[i]
                               //       = 234 * 10 + 5
                               //       = 2340 + 5 = 2345
                               //   Math: We formed (((2x + 3)x + 4)x + 5) = 2x^3 + 3x^2 + 4x + 5
                               //
                               // -----------------------------------------------------------
                               // PUNISHMENT TRACE (Harder Example: x = -2)
                               // coef = [5, 4, 3, 2]
                               //
                               // Init: res = 2
                               //
                               // i=2: res = 2 * (-2) + 3 
                               //          = -4 + 3 = -1
                               //
                               // i=1: res = (-1) * (-2) + 4
                               //          = 2 + 4 = 6
                               //      (Notice: Negative * Negative = Positive 2)
                               //
                               // i=0: res = 6 * (-2) + 5
                               //          = -12 + 5 = -7
                               //
                               // Result: -7  (Correct)
                               // -----------------------------------------------------------

        res = res * x + coef[i];
    }

    res

}

#[cfg(test)]
mod tests {
    use super::*;
    
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ TEST 1: Basic polynomial 2x^3 + 3x^2 + 4x + 5 at x = 10                     │
    // │                                                                             │
    // │ CALCULATION TRACE:                                                          │
    // │   coef = [5, 4, 3, 2]  meaning:                                             │
    // │     index 0 → coefficient 5 → term 5*x^0 = 5*1 = 5                          │
    // │     index 1 → coefficient 4 → term 4*x^1 = 4*10 = 40                        │
    // │     index 2 → coefficient 3 → term 3*x^2 = 3*100 = 300                      │
    // │     index 3 → coefficient 2 → term 2*x^3 = 2*1000 = 2000                    │
    // │   Sum = 5 + 40 + 300 + 2000 = 2345                                          │
    // └──────────────────────────────────────────────────────────────────────────────┘
    #[test]
    fn test_basic_polynomial_at_10() {
        let coef = vec![5, 4, 3, 2];
        let x = 10;
        let n = 3;
        assert_eq!(eval_polynom(&coef, x, n), 2345);
    }
    
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ TEST 2: x = 0 means only constant term survives                             │
    // │                                                                             │
    // │ Polynomial: 7x^3 + 4x^2 + 9x + 6                                            │
    // │ coef = [6, 9, 4, 7]                                                         │
    // │                                                                             │
    // │ AT x = 0:                                                                   │
    // │   0^0 = 1 (by convention for polynomials)                                   │
    // │   0^1 = 0                                                                   │
    // │   0^2 = 0                                                                   │
    // │   0^3 = 0                                                                   │
    // │                                                                             │
    // │ TERMS:                                                                      │
    // │   coef[0] * 0^0 = 6 * 1 = 6                                                 │
    // │   coef[1] * 0^1 = 9 * 0 = 0                                                 │
    // │   coef[2] * 0^2 = 4 * 0 = 0                                                 │
    // │   coef[3] * 0^3 = 7 * 0 = 0                                                 │
    // │   Sum = 6                                                                   │
    // │                                                                             │
    // │ BUG TRAP: If your power function handles 0^0 incorrectly, this test fails  │
    // └──────────────────────────────────────────────────────────────────────────────┘
    #[test]
    fn test_x_equals_zero() {
        let coef = vec![6, 9, 4, 7];
        let x = 0;
        let n = 3;
        assert_eq!(eval_polynom(&coef, x, n), 6);
    }
    
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ TEST 3: x = 1 means result is sum of all coefficients                       │
    // │                                                                             │
    // │ Polynomial: 2x^4 + 3x^3 + 1x^2 + 5x + 7                                     │
    // │ coef = [7, 5, 1, 3, 2]                                                      │
    // │                                                                             │
    // │ AT x = 1:                                                                   │
    // │   1^0 = 1, 1^1 = 1, 1^2 = 1, 1^3 = 1, 1^4 = 1                               │
    // │                                                                             │
    // │ TERMS:                                                                      │
    // │   coef[0] * 1 = 7                                                           │
    // │   coef[1] * 1 = 5                                                           │
    // │   coef[2] * 1 = 1                                                           │
    // │   coef[3] * 1 = 3                                                           │
    // │   coef[4] * 1 = 2                                                           │
    // │   Sum = 7 + 5 + 1 + 3 + 2 = 18                                              │
    // │                                                                             │
    // │ SANITY CHECK: Quick way to verify your implementation                       │
    // └──────────────────────────────────────────────────────────────────────────────┘
    #[test]
    fn test_x_equals_one() {
        let coef = vec![7, 5, 1, 3, 2];
        let x = 1;
        let n = 4;
        assert_eq!(eval_polynom(&coef, x, n), 18);
    }
    
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ TEST 4: Negative x = -2 with alternating sign powers                        │
    // │                                                                             │
    // │ Polynomial: x^3 + x^2 + x + 1                                               │
    // │ coef = [1, 1, 1, 1]                                                         │
    // │                                                                             │
    // │ AT x = -2:                                                                  │
    // │   (-2)^0 = 1      (even power 0 → positive)                                 │
    // │   (-2)^1 = -2     (odd power 1 → negative)                                  │
    // │   (-2)^2 = 4      (even power 2 → positive)                                 │
    // │   (-2)^3 = -8     (odd power 3 → negative)                                  │
    // │                                                                             │
    // │ TERMS:                                                                      │
    // │   coef[0] * 1 = 1                                                           │
    // │   coef[1] * (-2) = -2                                                       │
    // │   coef[2] * 4 = 4                                                           │
    // │   coef[3] * (-8) = -8                                                       │
    // │   Sum = 1 + (-2) + 4 + (-8) = 1 - 2 + 4 - 8 = -5                            │
    // │                                                                             │
    // │ BUG TRAP: If you lose the sign in power computation, you get +5 not -5     │
    // └──────────────────────────────────────────────────────────────────────────────┘
    #[test]
    fn test_negative_x() {
        let coef = vec![1, 1, 1, 1];
        let x = -2;
        let n = 3;
        assert_eq!(eval_polynom(&coef, x, n), -5);
    }
    
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ TEST 5: off-by-one detection with n = 5 (6 coefficients)                   │
    // │                                                                             │
    // │ Polynomial: 1x^5 + 2x^4 + 3x^3 + 4x^2 + 5x + 6                              │
    // │ coef = [6, 5, 4, 3, 2, 1]  (length = 6)                                     │
    // │                                                                             │
    // │ AT x = 10:                                                                  │
    // │   coef[0] * 10^0 = 6 * 1         = 6                                        │
    // │   coef[1] * 10^1 = 5 * 10        = 50                                       │
    // │   coef[2] * 10^2 = 4 * 100       = 400                                      │
    // │   coef[3] * 10^3 = 3 * 1000      = 3000                                     │
    // │   coef[4] * 10^4 = 2 * 10000     = 20000                                    │
    // │   coef[5] * 10^5 = 1 * 100000    = 100000                                   │
    // │   Sum = 6 + 50 + 400 + 3000 + 20000 + 100000 = 123456                       │
    // │                                                                             │
    // │ BUG TRAP: If your loop is for i in 0..n instead of 0..=n,                   │
    // │   you miss coef[5] and get 23456 instead of 123456                          │
    // │   Difference = 100000 (exactly the missing term coef[5] * 10^5)             │
    // └──────────────────────────────────────────────────────────────────────────────┘
    #[test]
    fn test_off_by_one_detection() {
        let coef = vec![6, 5, 4, 3, 2, 1];
        let x = 10;
        let n = 5;
        assert_eq!(eval_polynom(&coef, x, n), 123456);
    }
    
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ TEST 6: Negative coefficients mixed with positive                           │
    // │                                                                             │
    // │ Polynomial: 2x^4 - 3x^3 + x^2 - 5x + 7                                      │
    // │ coef = [7, -5, 1, -3, 2]                                                    │
    // │                                                                             │
    // │ AT x = 3:                                                                   │
    // │   3^0 = 1, 3^1 = 3, 3^2 = 9, 3^3 = 27, 3^4 = 81                             │
    // │                                                                             │
    // │ TERMS:                                                                      │
    // │   coef[0] * 1  = 7 * 1 = 7                                                  │
    // │   coef[1] * 3  = (-5) * 3 = -15                                             │
    // │   coef[2] * 9  = 1 * 9 = 9                                                  │
    // │   coef[3] * 27 = (-3) * 27 = -81                                            │
    // │   coef[4] * 81 = 2 * 81 = 162                                               │
    // │   Sum = 7 - 15 + 9 - 81 + 162 = 82                                          │
    // │                                                                             │
    // │ VERIFICATION:                                                               │
    // │   Positive terms: 7 + 9 + 162 = 178                                         │
    // │   Negative terms: 15 + 81 = 96                                              │
    // │   178 - 96 = 82 ✓                                                           │
    // └──────────────────────────────────────────────────────────────────────────────┘
    #[test]
    fn test_negative_coefficients() {
        let coef = vec![7, -5, 1, -3, 2];
        let x = 3;
        let n = 4;
        assert_eq!(eval_polynom(&coef, x, n), 82);
    }
    
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ TEST 7: Constant polynomial (degree 0)                                      │
    // │                                                                             │
    // │ Polynomial: 42 (just a constant)                                            │
    // │ coef = [42]                                                                 │
    // │                                                                             │
    // │ AT ANY x:                                                                   │
    // │   coef[0] * x^0 = 42 * 1 = 42                                               │
    // │                                                                             │
    // │ BUG TRAP: Your loop might not run at all if n = 0 and you wrote             │
    // │   for i in 0..n (which is 0..0, an empty range)                             │
    // └──────────────────────────────────────────────────────────────────────────────┘
    #[test]
    fn test_constant_polynomial() {
        let coef = vec![42];
        let x = 999;
        let n = 0;
        assert_eq!(eval_polynom(&coef, x, n), 42);
    }
    
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ TEST 8: Linear polynomial (degree 1)                                        │
    // │                                                                             │
    // │ Polynomial: 3x + 7                                                          │
    // │ coef = [7, 3]                                                               │
    // │                                                                             │
    // │ AT x = 5:                                                                   │
    // │   coef[0] * 5^0 = 7 * 1 = 7                                                 │
    // │   coef[1] * 5^1 = 3 * 5 = 15                                                │
    // │   Sum = 7 + 15 = 22                                                         │
    // │                                                                             │
    // │ VERIFICATION: 3(5) + 7 = 15 + 7 = 22 ✓                                      │
    // └──────────────────────────────────────────────────────────────────────────────┘
    #[test]
    fn test_linear_polynomial() {
        let coef = vec![7, 3];
        let x = 5;
        let n = 1;
        assert_eq!(eval_polynom(&coef, x, n), 22);
    }
}

fn main() {
    // ┌──────────────────────────────────────────────────────────────────────────────┐
    // │ MANUAL TESTING GROUND                                                       │
    // │                                                                             │
    // │ Run: cargo run                                                              │
    // │ Test: cargo test                                                            │
    // │                                                                             │
    // │ When you implement solution, trace through here:                            │
    // └──────────────────────────────────────────────────────────────────────────────┘
    
    println!("Polynomial Evaluation - Implement eval_polynom and run cargo test");
    
    // Example trace you should verify manually:
    // coef = [5, 4, 3, 2] represents 2x^3 + 3x^2 + 4x + 5
    // At x = 10: expected 2345
}
