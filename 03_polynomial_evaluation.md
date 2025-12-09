03 Polynomial Evaluation (Horner's Method)

PROBLEM
Evaluate polynomial 2x^3 + 3x^2 + 4x + 5 at x=10.
Array storage: [5, 4, 3, 2] (coef[i] = coefficient of x^i)
Expected: 2345

FIGURE 1: Array Storage Convention
Position in array = power of x.

index:   0   1   2   3
       +---+---+---+---+
coef:  | 5 | 4 | 3 | 2 |
       +---+---+---+---+
power: x^0 x^1 x^2 x^3

Paper notation: 2x^3 + 3x^2 + 4x + 5
Array notation: [5, 4, 3, 2]
Reading order differs from storage order.

FIGURE 2: Naive Evaluation Trace

coef[0] * x^0 = 5 * 1    = 5
coef[1] * x^1 = 4 * 10   = 40
coef[2] * x^2 = 3 * 100  = 300
coef[3] * x^3 = 2 * 1000 = 2000
Sum = 5 + 40 + 300 + 2000 = 2345

Multiplications for powers: 0+0+1+2 = 3
Multiplications for coefficients: 4
Total: 7 multiplications

FIGURE 3: Horner's Factorization

2x^3 + 3x^2 + 4x + 5 = ((2x + 3)x + 4)x + 5

Expansion verification:
  (2x + 3)         = 2x + 3
  (2x + 3)x        = 2x^2 + 3x
  (2x + 3)x + 4    = 2x^2 + 3x + 4
  ((2x + 3)x + 4)x = 2x^3 + 3x^2 + 4x
  + 5              = 2x^3 + 3x^2 + 4x + 5

FIGURE 4: Horner Execution Trace (x=10)

Start with highest coefficient: result = 2
Step 1: result = 2  * 10 + 3 = 20  + 3 = 23
Step 2: result = 23 * 10 + 4 = 230 + 4 = 234
Step 3: result = 234 * 10 + 5 = 2340 + 5 = 2345

Total: 3 multiplications (vs 7 naive)

FIGURE 5: Negative x Trace (x=-2, coef=[1,1,1,1])

(-2)^0 = 1
(-2)^1 = -2
(-2)^2 = 4
(-2)^3 = -8

Terms: 1*1=1, 1*(-2)=-2, 1*4=4, 1*(-8)=-8
Sum: 1 + (-2) + 4 + (-8) = -5

Pattern: Even powers positive, odd powers negative.

FIGURE 6: Loop Boundary

Array [5,4,3,2] has 4 elements at indices 0,1,2,3.
Degree n = 3.

Loop 0..n: indices 0,1,2 (misses index 3)
Loop 0..=n: indices 0,1,2,3 (complete)

Missing index 3 causes error of coef[3]*x^3 = 2000.

IMPLEMENTATION

fn horner(coef: &[i64], x: i64) -> i64 {
    if coef.is_empty() { return 0; }

    let mut result = coef[coef.len() - 1];
    for i in (0..coef.len() - 1).rev() {
        result = result * x + coef[i];
    }
    result
}
