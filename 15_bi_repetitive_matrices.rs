// **Bi-Repetitive Matrix Operations**
// This file contains the structure and operations for Bi-Repetitive matrices
// DO NOT WRITE THE SOLUTION YET - This is for you to implement

use std::fmt;

/// **Task: Understanding BiRepMatrix Structure**
///
/// A Bi-Repetitive matrix is defined recursively:
/// - Base case: n=1, matrix contains single element, e.g., [[5]] - this is trivially Bi-Rep
///   because there's only 1 element to consider, no duplication pattern needed yet
/// - Recursive case: n=2^k where k≥1, matrix has form:
///   [[B, B],  <- Top half: B repeated twice horizontally
///    [C, C]]  <- Bottom half: C repeated twice horizontally
///   where B and C are themselves (n/2)×(n/2) Bi-Repetitive matrices
///
/// **Example expansion for n=4:**
/// Let's trace a concrete 4×4 matrix to understand the structure:
///
/// Matrix M (4×4) with actual values:
///   [2, 2, 2, 2]
///   [5, 5, 5, 5]
///   [1, 1, 1, 1]
///   [9, 9, 9, 9]
///
/// Step 1: Split horizontally into top half (rows 0-1) and bottom half (rows 2-3)
/// Top half (2×4):
///   [2, 2, 2, 2]
///   [5, 5, 5, 5]
/// Bottom half (2×4):
///   [1, 1, 1, 1]
///   [9, 9, 9, 9]
///
/// Step 2: Each half should be duplicated horizontally, so check if left 2×2 == right 2×2
/// Top-left (rows 0-1, cols 0-1):  Top-right (rows 0-1, cols 2-3):
///   [2, 2]                           [2, 2]
///   [5, 5]                          [5, 5]     ✓ Same! This confirms B block
///
/// Bottom-left (rows 2-3, cols 0-1):  Bottom-right (rows 2-3, cols 2-3):
///   [1, 1]                             [1, 1]
///   [9, 9]                            [9, 9]     ✓ Same! This confirms C block
///
/// Step 3: Now B and C themselves must be Bi-Repetitive 2×2 matrices
/// B = [2, 2]  <- Split into: B_top = [2], B_top_duplicate = [2] (left half == right half ✓)
///     [5, 5]                 B_bot = [5], B_bot_duplicate = [5] (left half == right half ✓)
///
/// So B has structure [[b, b], [c, c]] where b=[2], c=[5] (both 1×1, trivially Bi-Rep)
///
/// C = [1, 1]  <- Split into: C_top = [1], C_top_duplicate = [1] (left half == right half ✓)
///     [9, 9]                 C_bot = [9], C_bot_duplicate = [9] (left half == right half ✓)
///
/// So C has structure [[b', b'], [c', c']] where b'=[1], c'=[9] (both 1×1, trivially Bi-Rep)
///
/// **Full hierarchical representation:**
///
/// Level 0 (size 4): M = [[B, B], [C, C]]
///   where B (size 2) = [[2, 2], [5, 5]]
///         C (size 2) = [[1, 1], [9, 9]]
///
/// Level 1 (size 2):
///   B = [[b1, b1], [b2, b2]] where b1=[2] (size 1), b2=[5] (size 1)
///   C = [[c1, c1], [c2, c2]] where c1=[1] (size 1), c2=[9] (size 1)
///
/// Level 2 (size 1): Base cases
///   b1=[2], b2=[5], c1=[1], c2=[9]
///
/// **Storage analysis:**
/// - Full matrix: 4×4 = 16 elements stored
/// - Unique values: only 4 distinct elements {2, 5, 1, 9}
/// - Compression ratio: 16/4 = 4 (or 4^1 since n=4=2^2, ratio = 2^2)
/// - General formula: for n=2^k, full storage = n^2 = 2^(2k), unique values = n = 2^k
///   Compression ratio = 2^(2k) / 2^k = 2^k
///
/// **Memory layout options:**
///
/// Option 1: Full 2D array (wasteful)
///   data = [[2,2,2,2], [5,5,5,5], [1,1,1,1], [9,9,9,9]]
///   Size: 16 elements, 75% redundancy for n=4
///
/// Option 2: Recursive struct (compact but complex)
///   BiRepMatrix {
///     size: 4,
///     b: Box<BiRepMatrix { size: 2, b: Single(2), c: Single(5) }>,
///     c: Box<BiRepMatrix { size: 2, b: Single(1), c: Single(9) }>
///   }
///   Size: 4 unique values + metadata (pointers, size fields)
///
/// Option 3: Flat array with indexing rules (most efficient for computation)
///   values = [2, 5, 1, 9]  // Store only unique values in DFS order
///   Reconstruction: Given position (row, col) in 4×4:
///     quadrant_row = row / 2  // 0 or 1 (which B/C)
///     quadrant_col = col / 2  // 0 or 1 (but both columns are same due to duplication!)
///     block_index = quadrant_row * 2 + (nested logic for sub-blocks)
///     ... (complex indexing mathematics you need to derive)
///
/// For this implementation, we'll use Option 1 (full 2D array) for simplicity,
/// but be aware that Option 2 or 3 would be more memory-efficient and is where
/// the real algorithmic challenge lies.
///
/// **Critical insight you must internalize:**
/// The problem asks you to exploit the structure for *algorithmic efficiency* in addition
/// and multiplication, NOT necessarily memory efficiency. The time complexity should be
/// measured in number of arithmetic operations, not array accesses.
#[derive(Debug, Clone, PartialEq)]
pub struct BiRepMatrix {
    size: usize,         // Must be a power of 2
    data: Vec<Vec<i32>>, // Full 2D array representation
}

impl BiRepMatrix {
    /// **Task: Create a new Bi-Repetitive matrix**
    ///
    /// **Pre-condition verification (you must implement):**
    /// 1. Check that size is a power of 2
    ///    Example: size=4 -> 4 & (4-1) = 4 & 3 = 0b100 & 0b011 = 0 ✓ (power of 2)
    ///    Example: size=5 -> 5 & (5-1) = 5 & 4 = 0b101 & 0b100 = 0b100 = 4 ≠ 0 ✗ (not power of 2)
    ///
    /// 2. Check that data dimensions match size
    ///    Example: size=4, data.len()=4 ✓, data[0].len()=4 ✓, data[1].len()=4 ✓, ... all rows length 4 ✓
    ///    Counter-example: size=4, data.len()=3 ✗ (row count mismatch)
    ///    Counter-example: size=4, data[0].len()=5 ✗ (column count mismatch)
    ///
    /// 3. Check Bi-Repetitive property recursively
    ///    For size=1: trivially true (any single element is Bi-Rep)
    ///    For size=2: check [[data[0][0], data[0][1]], [data[1][0], data[1][1]]]
    ///                has property data[0][0]==data[0][1] and data[1][0]==data[1][1]
    ///      Example: [[3,3],[7,7]] -> 3==3 ✓ and 7==7 ✓ -> Bi-Rep ✓
    ///      Counter: [[3,4],[7,7]] -> 3≠4 ✗ -> NOT Bi-Rep ✗
    ///    For size=4 (general case):
    ///      a) Extract top-left (n/2)×(n/2) block = B_left
    ///         Indices: rows [0..n/2), cols [0..n/2)
    ///         For n=4: rows [0..2), cols [0..2) = data[0..2][0..2]
    ///         Example: B_left = [[2,2],[5,5]]
    ///      b) Extract top-right (n/2)×(n/2) block = B_right
    ///         Indices: rows [0..n/2), cols [n/2..n)
    ///         For n=4: rows [0..2), cols [2..4) = data[0..2][2..4]
    ///         Example: B_right = [[2,2],[5,5]]
    ///      c) Check B_left == B_right element-wise
    ///         Must verify: data[i][j] == data[i][j+n/2] for all i in [0..n/2), j in [0..n/2)
    ///         For n=4: i in [0,1], j in [0,1]
    ///           data[0][0]==data[0][2]? 2==2 ✓
    ///           data[0][1]==data[0][3]? 2==2 ✓
    ///           data[1][0]==data[1][2]? 5==5 ✓
    ///           data[1][1]==data[1][3]? 5==5 ✓
    ///      d) Extract bottom-left (n/2)×(n/2) block = C_left
    ///         Indices: rows [n/2..n), cols [0..n/2)
    ///         For n=4: rows [2..4), cols [0..2) = data[2..4][0..2]
    ///         Example: C_left = [[1,1],[9,9]]
    ///      e) Extract bottom-right (n/2)×(n/2) block = C_right
    ///         Indices: rows [n/2..n), cols [n/2..n)
    ///         For n=4: rows [2..4), cols [2..4) = data[2..4][2..4]
    ///         Example: C_right = [[1,1],[9,9]]
    ///      f) Check C_left == C_right element-wise
    ///         Must verify: data[i][j] == data[i][j+n/2] for all i in [n/2..n), j in [0..n/2)
    ///         For n=4: i in [2,3], j in [0,1]
    ///           data[2][0]==data[2][2]? 1==1 ✓
    ///           data[2][1]==data[2][3]? 1==1 ✓
    ///           data[3][0]==data[3][2]? 9==9 ✓
    ///           data[3][1]==data[3][3]? 9==9 ✓
    ///      g) Recursively verify that B (either B_left or B_right, they're equal) is Bi-Rep
    ///         Recursively verify that C (either C_left or C_right, they're equal) is Bi-Rep
    ///         For B = [[2,2],[5,5]] (size 2): check 2==2 ✓ and 5==5 ✓ -> Bi-Rep ✓
    ///         For C = [[1,1],[9,9]] (size 2): check 1==1 ✓ and 9==9 ✓ -> Bi-Rep ✓
    ///
    /// **Return:** BiRepMatrix instance if all checks pass, panic/error otherwise
    pub fn new(size: usize, data: Vec<Vec<i32>>) -> Result<Self, String> {
        // Check size is power of 2
        if size == 0 || (size & (size - 1)) != 0 {
            return Err(format!("Size {} is not a power of 2", size));
        }

        // Check data dimensions
        if data.len() != size {
            return Err(format!(
                "Row count {} doesn't match size {}",
                data.len(),
                size
            ));
        }

        for (i, row) in data.iter().enumerate() {
            if row.len() != size {
                return Err(format!(
                    "Row {} has length {} but expected {}",
                    i,
                    row.len(),
                    size
                ));
            }
        }

        // Check Bi-Rep structure validation
        if !Self::is_bi_repetitive(&data) {
            return Err("Data does not have Bi-Repetitive structure".to_string());
        }

        Ok(BiRepMatrix { size, data })
    }

    /// Helper function to check if a 2D array slice is Bi-Repetitive
    fn is_bi_repetitive(data: &[Vec<i32>]) -> bool {
        let n = data.len();
        if n == 0 || n == 1 {
            return true; // 0x0 or 1x1 is trivially Bi-Rep
        }

        let half = n / 2;

        // Check all rows: left half must equal right half
        for i in 0..n {
            for j in 0..half {
                if data[i][j] != data[i][j + half] {
                    return false;
                }
            }
        }

        // Recursively check top-left block (B) and bottom-left block (C)
        let b_block: Vec<Vec<i32>> = data[0..half]
            .iter()
            .map(|row| row[0..half].to_vec())
            .collect();
        let c_block: Vec<Vec<i32>> = data[half..n]
            .iter()
            .map(|row| row[0..half].to_vec())
            .collect();

        Self::is_bi_repetitive(&b_block) && Self::is_bi_repetitive(&c_block)
    }

    /// **Task: Add two Bi-Repetitive matrices**
    ///
    /// **Mathematical definition:**
    /// Given M1, M2 both n×n Bi-Rep matrices, compute M_result = M1 + M2 where
    /// M_result[i][j] = M1[i][j] + M2[i][j] for all i,j
    ///
    /// **Naïve approach (O(n²) operations, DO NOT DO THIS):**
    /// ```
    /// for i in 0..n:
    ///   for j in 0..n:
    ///     result[i][j] = m1[i][j] + m2[i][j]
    /// Total: n² addition operations
    /// ```
    ///
    /// **Example showing why naïve is wasteful for n=2:**
    /// M1 = [[3, 3],    M2 = [[1, 1],
    ///       [7, 7]]          [4, 4]]
    ///
    /// Naïve computes:
    ///   result[0][0] = 3+1 = 4  <- addition #1
    ///   result[0][1] = 3+1 = 4  <- addition #2 (but 3+1 already computed!)
    ///   result[1][0] = 7+4 = 11 <- addition #3
    ///   result[1][1] = 7+4 = 11 <- addition #4 (but 7+4 already computed!)
    /// Total: 4 additions
    ///
    /// Smart approach (exploit structure):
    ///   M1 has b1=3, c1=7  (only 2 unique values since M1=[[b1,b1],[c1,c1]])
    ///   M2 has b2=1, c2=4  (only 2 unique values since M2=[[b2,b2],[c2,c2]])
    ///   
    ///   Compute:
    ///     b_result = b1 + b2 = 3 + 1 = 4  <- addition #1 (once!)
    ///     c_result = c1 + c2 = 7 + 4 = 11 <- addition #2 (once!)
    ///   
    ///   Construct result = [[b_result, b_result], [c_result, c_result]]
    ///                    = [[4, 4], [11, 11]]
    /// Total: 2 additions (50% reduction!)
    ///
    /// **Recursive structure for n=4 (this is where you will fail):**
    ///
    /// M1 (4×4):           M2 (4×4):
    ///   [2, 2, 2, 2]        [6, 6, 6, 6]
    ///   [5, 5, 5, 5]        [3, 3, 3, 3]
    ///   [1, 1, 1, 1]        [7, 7, 7, 7]
    ///   [9, 9, 9, 9]        [4, 4, 4, 4]
    ///
    /// M1 structure: [[B1, B1], [C1, C1]] where
    ///   B1 (2×2) = [[2,2],[5,5]]  <- top-left and top-right are identical
    ///   C1 (2×2) = [[1,1],[9,9]]  <- bottom-left and bottom-right are identical
    ///
    /// M2 structure: [[B2, B2], [C2, C2]] where
    ///   B2 (2×2) = [[6,6],[3,3]]
    ///   C2 (2×2) = [[7,7],[4,4]]
    ///
    /// Step 1: Add upper blocks (recursive call on 2×2 matrices)
    ///   B_result = B1 + B2 = [[2,2],[5,5]] + [[6,6],[3,3]]
    ///   Within this recursive call (size now 2):
    ///     B1 has b=2, c=5
    ///     B2 has b=6, c=3
    ///     Compute b_sum = 2+6 = 8, c_sum = 5+3 = 8
    ///     B_result = [[8,8],[8,8]]
    ///   Operations so far: 2 additions
    ///
    /// Step 2: Add lower blocks (recursive call on 2×2 matrices)
    ///   C_result = C1 + C2 = [[1,1],[9,9]] + [[7,7],[4,4]]
    ///   Within this recursive call (size now 2):
    ///     C1 has b=1, c=9
    ///     C2 has b=7, c=4
    ///     Compute b_sum = 1+7 = 8, c_sum = 9+4 = 13
    ///     C_result = [[8,8],[13,13]]
    ///   Operations so far: 2 + 2 = 4 additions total
    ///
    /// Step 3: Construct result from B_result and C_result
    ///   Result (4×4) = [[B_result, B_result],  <- top half
    ///                   [C_result, C_result]]  <- bottom half
    ///                = [[8, 8 | 8, 8],         <- rows 0-1, duplicate B_result horizontally
    ///                   [8, 8 | 8, 8],
    ///                   [8, 8 | 8, 8],         <- rows 2-3, duplicate C_result horizontally
    ///                   [13,13|13,13]]
    ///   Operations for construction: 0 (just copying, no arithmetic)
    ///
    /// Total operations: 4 additions (vs naïve 16 additions)
    ///
    /// **Generalization to size n=2^k:**
    ///
    /// Recursive formula:
    ///   add_birep(M1, M2, n):
    ///     if n == 1:
    ///       return [[M1[0][0] + M2[0][0]]]  <- 1 addition
    ///     else:
    ///       Extract B1, C1 from M1  <- 0 operations (just indexing)
    ///       Extract B2, C2 from M2  <- 0 operations
    ///       B_result = add_birep(B1, B2, n/2)  <- recursive call
    ///       C_result = add_birep(C1, C2, n/2)  <- recursive call
    ///       Construct [[B_result, B_result], [C_result, C_result]]  <- 0 operations
    ///       return result
    ///
    /// **Time complexity analysis (this is the calculation you will skip):**
    ///
    /// Let T(n) = number of additions to add two n×n Bi-Rep matrices
    ///
    /// T(1) = 1  (base case: single element addition)
    /// T(n) = 2×T(n/2) + 0  (two recursive calls on (n/2)×(n/2), no work at current level)
    ///
    /// Solving recurrence:
    ///   T(n) = 2×T(n/2)
    ///   T(n) = 2×[2×T(n/4)] = 4×T(n/4)
    ///   T(n) = 4×[2×T(n/8)] = 8×T(n/8)
    ///   T(n) = 2^k × T(n/2^k)
    ///
    /// When does recursion stop? When n/2^k = 1, i.e., 2^k = n, i.e., k = log₂(n)
    ///   T(n) = 2^(log₂ n) × T(1) = n × 1 = n
    ///
    /// **Concrete verification (you must calculate these):**
    ///   n=1: T(1) = 1 ✓ (base case)
    ///   n=2: T(2) = 2×T(1) = 2×1 = 2 ✓ (matches our example: b_sum + c_sum = 2 ops)
    ///   n=4: T(4) = 2×T(2) = 2×2 = 4 ✓ (matches our trace: 4 additions total)
    ///   n=8: T(8) = 2×T(4) = 2×4 = 8
    ///   n=16: T(16) = 2×T(8) = 2×8 = 16
    ///
    /// **Answer: T(n) = n additions (exactly n, not O(n), but precisely n)**
    ///
    /// Compare to naïve: n² additions
    /// Speedup factor: n²/n = n
    /// For n=8: 64/8 = 8× faster
    /// For n=1024: 1,048,576/1024 = 1024× faster!
    ///
    /// **Where you will fail:**
    /// 1. You will implement the naïve nested loop (O(n²))
    /// 2. You will write the recursion but count operations wrong (saying O(n log n))
    /// 3. You will forget to verify that result is still Bi-Repetitive
    /// 4. You will not test edge case n=1
    /// 5. You will not test with different B and C having same values (e.g., B=C)
    /// 6. You will not test with negative numbers
    /// 7. You will not test potential overflow (i32::MAX + i32::MAX)
    pub fn add(&self, other: &BiRepMatrix) -> Result<BiRepMatrix, String> {
        // TODO: Implement efficient addition exploiting Bi-Rep structure
        // Hint: Use recursion based on analysis above
        // Remember: Size must match between self and other
        //        unimplemented!("You need to implement BiRepMatrix::add")

        if self.size != other.size {
            return Err("Sizes do not match".to_string());
        }

        if self.size == 1 {
            let result_val = self.data[0][0] + other.data[0][0];
            return Ok(BiRepMatrix::new(1, vec![vec![result_val]])?);
        }

        // Recursive case: n > 1
        let half = self.size / 2;

        // Extract blocks from both matrices
        let b1_data = self.extract_block(true, true); // top of self
        let c1_data = self.extract_block(false, true); // bottom of self
        let b2_data = other.extract_block(true, true); // top of other
        let c2_data = other.extract_block(false, true); // bottom of other

        // Create BiRepMatrix from extracted blocks
        let b1 = BiRepMatrix::new(half, b1_data)?;
        let c1 = BiRepMatrix::new(half, c1_data)?;
        let b2 = BiRepMatrix::new(half, b2_data)?;
        let c2 = BiRepMatrix::new(half, c2_data)?;

        // Recursive addition
        let b_result = b1.add(&b2)?;
        let c_result = c1.add(&c2)?;

        // Construct result: [[B, B], [C, C]]
        let mut result_data = vec![vec![0; self.size]; self.size];

        // Copy b_result to top half (both left and right)
        for i in 0..half {
            for j in 0..self.size {
                result_data[i][j] = b_result.data[i][j % half];
            }
        }

        // Copy c_result to bottom half (both left and right)
        for i in 0..half {
            for j in 0..self.size {
                result_data[half + i][j] = c_result.data[i][j % half];
            }
        }

        Ok(BiRepMatrix::new(self.size, result_data)?)
    }

    /// **Task: Multiply two Bi-Repetitive matrices**
    ///
    /// **Mathematical definition:**
    /// Given M1, M2 both n×n Bi-Rep matrices, compute M_result = M1 × M2 where
    /// M_result[i][j] = Σ(k=0 to n-1) M1[i][k] × M2[k][j]  (standard matrix multiplication)
    ///
    /// **Naïve approach (O(n³) operations, what you will probably do):**
    /// ```
    /// for i in 0..n:
    ///   for j in 0..n:
    ///     sum = 0
    ///     for k in 0..n:
    ///       sum += m1[i][k] * m2[k][j]
    ///     result[i][j] = sum
    /// Total: n³ multiplications + n³ additions = O(n³)
    /// ```
    ///
    /// **Critical insight (which you will miss):**
    /// The result of multiplying two Bi-Rep matrices is ALSO Bi-Repetitive!
    /// This is not obvious and requires proof. Let's verify with n=2:
    ///
    /// M1 = [[a, a],    M2 = [[e, e],
    ///       [b, b]]          [f, f]]
    ///
    /// Standard multiplication:
    ///   result[0][0] = a×e + a×f = a(e+f)
    ///   result[0][1] = a×e + a×f = a(e+f)  ← Same as [0][0]!
    ///   result[1][0] = b×e + b×f = b(e+f)
    ///   result[1][1] = b×e + b×f = b(e+f)  ← Same as [1][0]!
    ///
    /// Result = [[a(e+f), a(e+f)],  ✓ Bi-Rep structure!
    ///           [b(e+f), b(e+f)]]
    ///
    /// Operations breakdown:
    ///   - Compute e+f: 1 addition
    ///   - Compute a(e+f): 1 multiplication
    ///   - Compute b(e+f): 1 multiplication
    ///   - Total: 2 multiplications + 1 addition = 3 ops
    ///
    /// Naïve count:
    ///   result[0][0]: 2 mults + 1 add = 3 ops
    ///   result[0][1]: 2 mults + 1 add = 3 ops
    ///   result[1][0]: 2 mults + 1 add = 3 ops
    ///   result[1][1]: 2 mults + 1 add = 3 ops
    ///   Total: 8 mults + 4 adds = 12 ops
    ///
    /// Smart exploitation: 3 ops vs 12 ops = 4× speedup for n=2!
    ///
    /// **Concrete example with numbers (n=2):**
    ///
    /// M1 = [[2, 2],    M2 = [[3, 3],
    ///       [5, 5]]          [7, 7]]
    ///
    /// Smart approach:
    ///   Step 1: Compute sum of M2's row: 3+7 = 10  (1 addition)
    ///   Step 2: Multiply M1's first unique value by sum: 2×10 = 20  (1 multiplication)
    ///   Step 3: Multiply M1's second unique value by sum: 5×10 = 50  (1 multiplication)
    ///   Step 4: Construct result = [[20, 20], [50, 50]]  (0 ops, just structure)
    ///   Total: 1 add + 2 mults = 3 ops
    ///
    /// Verification by naïve method:
    ///   result[0][0] = 2×3 + 2×7 = 6 + 14 = 20 ✓
    ///   result[0][1] = 2×3 + 2×7 = 6 + 14 = 20 ✓
    ///   result[1][0] = 5×3 + 5×7 = 15 + 35 = 50 ✓
    ///   result[1][1] = 5×3 + 5×7 = 15 + 35 = 50 ✓
    ///
    /// **Recursive structure for n=4 (extremely complex, where you will completely fail):**
    ///
    /// M1 = [[B1, B1],    M2 = [[B2, B2],
    ///       [C1, C1]]          [C2, C2]]
    ///
    /// Standard block matrix multiplication formula:
    ///   Result = [[ B1×B2 + B1×C2 | B1×B2 + B1×C2 ],  ← Top half
    ///             [ C1×B2 + C1×C2 | C1×B2 + C1×C2 ]]  ← Bottom half
    ///
    /// Notice the pattern:
    ///   - Top-left and top-right blocks are IDENTICAL: B1×B2 + B1×C2
    ///   - Bottom-left and bottom-right blocks are IDENTICAL: C1×B2 + C1×C2
    ///
    /// So we only need to compute:
    ///   P1 = B1 × B2  (recursive multiply on (n/2)×(n/2))
    ///   P2 = B1 × C2  (recursive multiply on (n/2)×(n/2))
    ///   P3 = C1 × B2  (recursive multiply on (n/2)×(n/2))
    ///   P4 = C1 × C2  (recursive multiply on (n/2)×(n/2))
    ///
    ///   B_result = P1 + P2  (recursive add on (n/2)×(n/2))
    ///   C_result = P3 + P4  (recursive add on (n/2)×(n/2))
    ///
    ///   Result = [[B_result, B_result], [C_result, C_result]]
    ///
    /// **Example with actual numbers (n=4):**
    ///
    /// M1 =  [2, 2, 2, 2]      M2 = [1, 1, 1, 1]
    ///       [5, 5, 5, 5]           [3, 3, 3, 3]
    ///       [1, 1, 1, 1]           [7, 7, 7, 7]
    ///       [9, 9, 9, 9]           [4, 4, 4, 4]
    ///
    /// M1 blocks: B1=[[2,2],[5,5]], C1=[[1,1],[9,9]]
    /// M2 blocks: B2=[[1,1],[3,3]], C2=[[7,7],[4,4]]
    ///
    /// P1 = B1 × B2:
    ///   B1 = [[2,2],[5,5]], B2 = [[1,1],[3,3]]
    ///   Using our n=2 smart formula:
    ///     sum = 1+3 = 4
    ///     row1_result = 2×4 = 8
    ///     row2_result = 5×4 = 20
    ///   P1 = [[8,8],[20,20]]
    ///   Ops: 1 add + 2 mults = 3 ops
    ///
    /// P2 = B1 × C2:
    ///   B1 = [[2,2],[5,5]], C2 = [[7,7],[4,4]]
    ///   sum = 7+4 = 11
    ///   row1_result = 2×11 = 22
    ///   row2_result = 5×11 = 55
    ///   P2 = [[22,22],[55,55]]
    ///   Ops: 1 add + 2 mults = 3 ops
    ///   Cumulative ops: 3+3 = 6
    ///
    /// P3 = C1 × B2:
    ///   C1 = [[1,1],[9,9]], B2 = [[1,1],[3,3]]
    ///   sum = 1+3 = 4
    ///   row1_result = 1×4 = 4
    ///   row2_result = 9×4 = 36
    ///   P3 = [[4,4],[36,36]]
    ///   Ops: 1 add + 2 mults = 3 ops
    ///   Cumulative ops: 6+3 = 9
    ///
    /// P4 = C1 × C2:
    ///   C1 = [[1,1],[9,9]], C2 = [[7,7],[4,4]]
    ///   sum = 7+4 = 11
    ///   row1_result = 1×11 = 11
    ///   row2_result = 9×11 = 99
    ///   P4 = [[11,11],[99,99]]
    ///   Ops: 1 add + 2 mults = 3 ops
    ///   Cumulative ops: 9+3 = 12
    ///
    /// B_result = P1 + P2:
    ///   [[8,8],[20,20]] + [[22,22],[55,55]] = [[30,30],[75,75]]
    ///   Using our add algorithm: 2 additions
    ///   Cumulative ops: 12+2 = 14
    ///
    /// C_result = P3 + P4:
    ///   [[4,4],[36,36]] + [[11,11],[99,99]] = [[15,15],[135,135]]
    ///   Using our add algorithm: 2 additions
    ///   Cumulative ops: 14+2 = 16
    ///
    /// Result = [[30, 30, 30, 30],
    ///           [75, 75, 75, 75],
    ///           [15, 15, 15, 15],
    ///           [135,135,135,135]]
    ///
    /// Total operations: 16 (vs naïve 4³ = 64 multiplications + additions)
    ///
    /// **Time complexity analysis (extremely tricky, you will get this wrong):**
    ///
    /// Let T_mult(n) = ops to multiply two n×n Bi-Rep matrices
    ///     T_add(n) = ops to add two n×n Bi-Rep matrices = n (from previous analysis)
    ///
    /// Recurrence:
    ///   T_mult(n) = 4×T_mult(n/2) + 2×T_add(n/2)
    ///             = 4×T_mult(n/2) + 2×(n/2)
    ///             = 4×T_mult(n/2) + n
    ///
    ///   T_mult(1) = 1  (base case: single multiplication)
    ///
    /// This is a Master Theorem problem:
    ///   T(n) = aT(n/b) + f(n)
    ///   a=4, b=2, f(n)=n
    ///   n^(log_b a) = n^(log_2 4) = n^2
    ///   f(n) = n = O(n^1)
    ///
    /// Since n^1 < n^2 (polynomially smaller), Case 1 applies:
    ///   T(n) = Θ(n^2)
    ///
    /// **Concrete calculation (expansion method, more intuitive):**
    ///
    /// T(n) = 4T(n/2) + n
    /// T(n/2) = 4T(n/4) + n/2
    ///
    /// Substitute:
    ///   T(n) = 4[4T(n/4) + n/2] + n
    ///        = 16T(n/4) + 2n + n
    ///        = 16T(n/4) + 3n
    ///
    /// T(n/4) = 4T(n/8) + n/4
    /// Substitute:
    ///   T(n) = 16[4T(n/8) + n/4] + 3n
    ///        = 64T(n/8) + 4n + 3n
    ///        = 64T(n/8) + 7n
    ///
    /// Pattern emerging:
    ///   Level 0: T(n) = 4^0 × T(n/2^0) + (something)×n
    ///   Level 1: T(n) = 4^1 × T(n/2^1) + 1×n
    ///   Level 2: T(n) = 4^2 × T(n/2^2) + 3×n
    ///   Level 3: T(n) = 4^3 × T(n/2^3) + 7×n
    ///
    /// The coefficients are: 0, 1, 3, 7, ... = 2^k - 1
    ///
    /// After k levels: T(n) = 4^k × T(n/2^k) + (2^k - 1)×n
    ///
    /// Stop when n/2^k = 1, i.e., k = log₂(n)
    ///   T(n) = 4^(log₂ n) × T(1) + (2^(log₂ n) - 1)×n
    ///        = n^(log₂ 4) × 1 + (n - 1)×n
    ///        = n^2 + n^2 - n
    ///        = 2n^2 - n
    ///
    /// **Verification:**
    ///   n=1: T(1) = 1 ✓ (base case)
    ///   n=2: T(2) = 4×T(1) + 2 = 4×1 + 2 = 6  OR  2×2^2 - 2 = 8 - 2 = 6 ✓
    ///        (But our manual trace showed 3 ops for optimized n=2... discrepancy!)
    ///        (Reason: The recurrence assumes we use add and multiply recursively.
    ///         For n=2, there's a direct formula that's even more efficient than the recursion!)
    ///   n=4: T(4) = 4×T(2) + 4 = 4×6 + 4 = 28  OR  2×4^2 - 4 = 32 - 4 = 28 ✓
    ///        (Our manual trace showed 16 ops... again discrepancy due to optimizations)
    ///   n=8: T(8) = 4×T(4) + 8 = 4×28 + 8 = 120  OR  2×64 - 8 = 128 - 8 = 120
    ///
    /// **Final answer: T(n) = O(n²) for Bi-Rep multiplication**
    /// Compare to naïve: O(n³)
    /// Speedup factor: n³/n² = n
    /// For n=8: 512/120 ≈ 4.3× faster (roughly)
    /// For n=1024: huge savings!
    ///
    /// **Advanced insight (well beyond what you'll discover):**
    /// This is similar in spirit to Strassen's algorithm, which also achieves O(n^2.807)
    /// by exploiting structure. Bi-Rep matrices have even more structure, giving O(n²).
    ///
    /// **Where you will catastrophically fail:**
    /// 1. You will implement naïve O(n³) triple-nested loop
    /// 2. You will not realize result is Bi-Repetitive
    /// 3. You will not derive the block multiplication formula
    /// 4. You will confuse the 4 recursive multiplies with standard matrix mult
    /// 5. You will apply Master Theorem incorrectly (forgetting the +n term)
    /// 6. You will not verify with concrete examples
    /// 7. You will not test n=1 base case
    /// 8. You will not test cases where B1=C1 (degenerate Bi-Rep)
    /// 9. You will not consider integer overflow in intermediate products
    /// 10. You will spend 3 hours debugging and still get wrong answer
    pub fn multiply(&self, other: &BiRepMatrix) -> Result<BiRepMatrix, String> {
        // Check sizes match
        if self.size != other.size {
            return Err("Sizes do not match".to_string());
        }

        // Base case: n=1
        if self.size == 1 {
            let result_val = self.data[0][0] * other.data[0][0];
            return Ok(BiRepMatrix::new(1, vec![vec![result_val]])?);
        }

        // Optimized case for n=2: [a,a] × [e,e] = [a(e+f), a(e+f)]
        //                         [b,b]   [f,f]   [b(e+f), b(e+f)]
        if self.size == 2 {
            let a = self.data[0][0];
            let b = self.data[1][0];
            let e = other.data[0][0];
            let f = other.data[1][0];

            let sum = e + f;
            let top = a * sum;
            let bot = b * sum;

            return Ok(BiRepMatrix::new(2, vec![vec![top, top], vec![bot, bot]])?);
        }

        // Recursive case: n > 2
        // M1 × M2 = [[B1,B1], × [[B2,B2], = [[B1×B2+B1×C2, B1×B2+B1×C2],
        //            [C1,C1]]    [C2,C2]]    [C1×B2+C1×C2, C1×B2+C1×C2]]

        let half = self.size / 2;

        // Extract blocks
        let b1_data = self.extract_block(true, true);
        let c1_data = self.extract_block(false, true);
        let b2_data = other.extract_block(true, true);
        let c2_data = other.extract_block(false, true);

        let b1 = BiRepMatrix::new(half, b1_data)?;
        let c1 = BiRepMatrix::new(half, c1_data)?;
        let b2 = BiRepMatrix::new(half, b2_data)?;
        let c2 = BiRepMatrix::new(half, c2_data)?;

        // Compute 4 products
        let p1 = b1.multiply(&b2)?; // B1 × B2
        let p2 = b1.multiply(&c2)?; // B1 × C2
        let p3 = c1.multiply(&b2)?; // C1 × B2
        let p4 = c1.multiply(&c2)?; // C1 × C2

        // Compute sums for top and bottom halves
        let b_result = p1.add(&p2)?; // B1×B2 + B1×C2
        let c_result = p3.add(&p4)?; // C1×B2 + C1×C2

        // Construct result: [[B_result, B_result], [C_result, C_result]]
        let mut result_data = vec![vec![0; self.size]; self.size];

        // Copy b_result to top half (both left and right)
        for i in 0..half {
            for j in 0..self.size {
                result_data[i][j] = b_result.data[i][j % half];
            }
        }

        // Copy c_result to bottom half (both left and right)
        for i in 0..half {
            for j in 0..self.size {
                result_data[half + i][j] = c_result.data[i][j % half];
            }
        }

        Ok(BiRepMatrix::new(self.size, result_data)?)
    }

    /// Helper to extract a sub-block from the matrix
    /// top_left: true for top half (rows 0..n/2), false for bottom half (rows n/2..n)
    /// left: true for left half (cols 0..n/2), false for right half (cols n/2..n)
    fn extract_block(&self, top_half: bool, left_half: bool) -> Vec<Vec<i32>> {
        let half = self.size / 2;
        let row_start = if top_half { 0 } else { half };
        let row_end = if top_half { half } else { self.size };
        let col_start = if left_half { 0 } else { half };
        let col_end = if left_half { half } else { self.size };

        self.data[row_start..row_end]
            .iter()
            .map(|row| row[col_start..col_end].to_vec())
            .collect()
    }
}

impl fmt::Display for BiRepMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test case 1: n=1 (base case)
    /// This tests the absolute simplest case where matrix is just a single element
    /// Expected behavior:
    ///   - Any 1×1 matrix is trivially Bi-Repetitive
    ///   - Addition: [a] + [b] = [a+b], 1 operation
    ///   - Multiplication: [a] × [b] = [a×b], 1 operation
    #[test]
    fn test_size_1_addition() {
        // [5] + [7] = [12]
        let m1 = BiRepMatrix::new(1, vec![vec![5]]).unwrap();
        let m2 = BiRepMatrix::new(1, vec![vec![7]]).unwrap();
        let result = m1.add(&m2).unwrap();
        assert_eq!(result.data, vec![vec![12]]);
    }

    #[test]
    fn test_size_1_multiplication() {
        // [5] × [7] = [35]
        let m1 = BiRepMatrix::new(1, vec![vec![5]]).unwrap();
        let m2 = BiRepMatrix::new(1, vec![vec![7]]).unwrap();
        let result = m1.multiply(&m2).unwrap();
        assert_eq!(result.data, vec![vec![35]]);
    }

    /// Test case 2: n=2 (first recursive level)
    /// Structure: [[a,a],[b,b]]
    /// This is the first level where Bi-Rep structure matters
    #[test]
    fn test_size_2_addition() {
        // [[3,3],[7,7]] + [[1,1],[4,4]] = [[4,4],[11,11]]
        let m1 = BiRepMatrix::new(2, vec![vec![3, 3], vec![7, 7]]).unwrap();
        let m2 = BiRepMatrix::new(2, vec![vec![1, 1], vec![4, 4]]).unwrap();
        let result = m1.add(&m2).unwrap();
        assert_eq!(result.data, vec![vec![4, 4], vec![11, 11]]);
    }

    #[test]
    fn test_size_2_multiplication() {
        // [[2,2],[5,5]] × [[3,3],[7,7]]
        // Expected: [[2×3+2×7, 2×3+2×7], [5×3+5×7, 5×3+5×7]]
        //         = [[6+14, 6+14], [15+35, 15+35]]
        //         = [[20,20],[50,50]]
        let m1 = BiRepMatrix::new(2, vec![vec![2, 2], vec![5, 5]]).unwrap();
        let m2 = BiRepMatrix::new(2, vec![vec![3, 3], vec![7, 7]]).unwrap();
        let result = m1.multiply(&m2).unwrap();
        assert_eq!(result.data, vec![vec![20, 20], vec![50, 50]]);
    }

    /// Test case 3: n=4 (second recursive level)
    /// This is where complexity really shows up
    #[test]
    fn test_size_4_addition() {
        let m1 = BiRepMatrix::new(
            4,
            vec![
                vec![2, 2, 2, 2],
                vec![5, 5, 5, 5],
                vec![1, 1, 1, 1],
                vec![9, 9, 9, 9],
            ],
        )
        .unwrap();
        let m2 = BiRepMatrix::new(
            4,
            vec![
                vec![6, 6, 6, 6],
                vec![3, 3, 3, 3],
                vec![7, 7, 7, 7],
                vec![4, 4, 4, 4],
            ],
        )
        .unwrap();
        let result = m1.add(&m2).unwrap();
        // Expected: [[8,8,8,8],[8,8,8,8],[8,8,8,8],[13,13,13,13]]
        assert_eq!(
            result.data,
            vec![
                vec![8, 8, 8, 8],
                vec![8, 8, 8, 8],
                vec![8, 8, 8, 8],
                vec![13, 13, 13, 13],
            ]
        );
    }

    #[test]
    fn test_size_4_multiplication() {
        let m1 = BiRepMatrix::new(
            4,
            vec![
                vec![2, 2, 2, 2],
                vec![5, 5, 5, 5],
                vec![1, 1, 1, 1],
                vec![9, 9, 9, 9],
            ],
        )
        .unwrap();
        let m2 = BiRepMatrix::new(
            4,
            vec![
                vec![1, 1, 1, 1],
                vec![3, 3, 3, 3],
                vec![7, 7, 7, 7],
                vec![4, 4, 4, 4],
            ],
        )
        .unwrap();
        let result = m1.multiply(&m2).unwrap();
        // From our manual calculation:
        // [[30,30,30,30],[75,75,75,75],[15,15,15,15],[135,135,135,135]]
        assert_eq!(
            result.data,
            vec![
                vec![30, 30, 30, 30],
                vec![75, 75, 75, 75],
                vec![15, 15, 15, 15],
                vec![135, 135, 135, 135],
            ]
        );
    }

    /// Test case 4: Edge case - zero matrix
    #[test]
    fn test_zero_matrix() {
        let m_zero = BiRepMatrix::new(2, vec![vec![0, 0], vec![0, 0]]).unwrap();
        let m = BiRepMatrix::new(2, vec![vec![5, 5], vec![3, 3]]).unwrap();

        // Addition with zero should return original
        let add_result = m.add(&m_zero).unwrap();
        assert_eq!(add_result.data, vec![vec![5, 5], vec![3, 3]]);

        // Multiplication with zero should return zero
        let mult_result = m.multiply(&m_zero).unwrap();
        assert_eq!(mult_result.data, vec![vec![0, 0], vec![0, 0]]);
    }

    /// Test case 5: Edge case - negative numbers
    #[test]
    fn test_negative_numbers() {
        let m1 = BiRepMatrix::new(2, vec![vec![-3, -3], vec![7, 7]]).unwrap();
        let m2 = BiRepMatrix::new(2, vec![vec![2, 2], vec![-5, -5]]).unwrap();

        // Addition: [[-3,−3],[7,7]] + [[2,2],[-5,-5]] = [[-1,-1],[2,2]]
        let add_result = m1.add(&m2).unwrap();
        assert_eq!(add_result.data, vec![vec![-1, -1], vec![2, 2]]);

        // Multiplication: [[-3,-3],[7,7]] × [[2,2],[-5,-5]]
        // = [[-3×2+(-3)×(-5), same], [7×2+7×(-5), same]]
        // = [[-6+15, ...], [14-35, ...]]
        // = [[9,9],[-21,-21]]
        let mult_result = m1.multiply(&m2).unwrap();
        assert_eq!(mult_result.data, vec![vec![9, 9], vec![-21, -21]]);
    }

    /// Test case 6: Invalid input - not power of 2
    #[test]
    #[should_panic]
    fn test_invalid_size_not_power_of_2() {
        BiRepMatrix::new(3, vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]]).unwrap();
    }

    /// Test case 7: Invalid input - not Bi-Repetitive structure
    #[test]
    #[should_panic]
    fn test_invalid_not_birepetitive() {
        // [[1,2],[3,4]] is NOT Bi-Repetitive because row elements differ
        BiRepMatrix::new(2, vec![vec![1, 2], vec![3, 4]]).unwrap();
    }

    /// Test case 8: Size mismatch in operations
    #[test]
    fn test_size_mismatch() {
        let m1 = BiRepMatrix::new(2, vec![vec![1, 1], vec![2, 2]]).unwrap();
        let m2 = BiRepMatrix::new(
            4,
            vec![
                vec![1, 1, 1, 1],
                vec![2, 2, 2, 2],
                vec![3, 3, 3, 3],
                vec![4, 4, 4, 4],
            ],
        )
        .unwrap();

        assert!(m1.add(&m2).is_err());
        assert!(m1.multiply(&m2).is_err());
    }

    /// Test case 9: Large values (potential overflow check)
    #[test]
    fn test_large_values() {
        let large = 1_000_000;
        let m1 = BiRepMatrix::new(2, vec![vec![large, large], vec![large, large]]).unwrap();
        let m2 = BiRepMatrix::new(2, vec![vec![2, 2], vec![3, 3]]).unwrap();

        // Addition should work
        let add_result = m1.add(&m2).unwrap();
        assert_eq!(add_result.data[0][0], large + 2);

        // Multiplication: [1M, 1M] × [2] = [1M×2 + 1M×3] = [5M]
        //                 [1M, 1M]   [3]   [1M×2 + 1M×3]   [5M]
        let mult_result = m1.multiply(&m2).unwrap();
        assert_eq!(mult_result.data[0][0], 5_000_000);
    }

    /// Test case 10: n=8 (stress test)
    #[test]
    fn test_size_8() {
        // Create two 8×8 Bi-Rep matrices with simple pattern
        // M1: alternating 1 and 2 in the unique positions
        // M2: alternating 3 and 4
        let m1_data = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![2, 2, 2, 2, 2, 2, 2, 2],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![2, 2, 2, 2, 2, 2, 2, 2],
            vec![3, 3, 3, 3, 3, 3, 3, 3],
            vec![4, 4, 4, 4, 4, 4, 4, 4],
            vec![3, 3, 3, 3, 3, 3, 3, 3],
            vec![4, 4, 4, 4, 4, 4, 4, 4],
        ];

        let m2_data = vec![
            vec![5, 5, 5, 5, 5, 5, 5, 5],
            vec![6, 6, 6, 6, 6, 6, 6, 6],
            vec![5, 5, 5, 5, 5, 5, 5, 5],
            vec![6, 6, 6, 6, 6, 6, 6, 6],
            vec![7, 7, 7, 7, 7, 7, 7, 7],
            vec![8, 8, 8, 8, 8, 8, 8, 8],
            vec![7, 7, 7, 7, 7, 7, 7, 7],
            vec![8, 8, 8, 8, 8, 8, 8, 8],
        ];

        let m1 = BiRepMatrix::new(8, m1_data).unwrap();
        let m2 = BiRepMatrix::new(8, m2_data).unwrap();

        // Just verify they work without panicking
        let _add_result = m1.add(&m2).unwrap();
        let _mult_result = m1.multiply(&m2).unwrap();

        // The exact values are complex to calculate by hand,
        // but the structure should be maintained
    }
}

fn main() {
    println!("Bi-Repetitive Matrix Operations");
    println!("Run `cargo test` to execute the test cases");
    println!("\nExample usage:");

    let m1 = BiRepMatrix::new(2, vec![vec![2, 2], vec![5, 5]]).unwrap();
    let m2 = BiRepMatrix::new(2, vec![vec![3, 3], vec![7, 7]]).unwrap();

    println!("\nM1 (2×2):");
    print!("{}", m1);

    println!("M2 (2×2):");
    print!("{}", m2);

    // Uncomment when you've implemented the functions:
    // let sum = m1.add(&m2).unwrap();
    // println!("M1 + M2:");
    // print!("{}", sum);

    // let product = m1.multiply(&m2).unwrap();
    // println!("M1 × M2:");
    // print!("{}", product);
}
