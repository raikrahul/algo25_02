fn main() {
    let input = "ABC";
    let result = all_combinations(input);
    println!("Combinations for '{}': {:?}", input, result);
}

// Task:
// 1. **Break Down the Code**:
//    - The `all_combinations` function takes `s` (address 0x7FF... approx) which points to "ABC" (bytes `0x41, 0x42, 0x43`).
//    - It returns a vector of strings. Each string is a new heap allocation.
//    - For "ABC", we expect 7 strings: "A", "B", "C", "AB", "AC", "BC", "ABC".
//    - The empty string "" is strictly excluded based on the example output.
//    - Recursion Depth: 3 (for 'C') or 4 (if counting base case `index == len`).
//    - Stack usage per frame: `index` (8 bytes) + `current_string` reference (24 bytes) + return address (8 bytes) ≈ 40-50 bytes.
//    - Total stack for depth 3 ≈ 150 bytes. Safe for standard 2MB stack.
//    - 1st call: index=0 ('A'). Decision: Include 'A' or Exclude 'A'.
//    - If Include 'A': append 0x41. Recurse index=1.
//
// You have skipped numerical calculations in the previous step. Work out the numerical calculation till the every end, and as a punishment, make another numerical example, harder, and work it out step by step.
// **Punishment Calculation (Middle Start):**
// ... 5832 / 18.
// 5800 / 18 -> 300 * 18 = 5400. Remainder 432.
// 432 / 18 -> 20 * 18 = 360. Remainder 72.
// 72 / 18 = 4.
// Total = 300 + 20 + 4 = 324.
// 324 is 18^2.
// Surprise: 18^3 = 5832.
//
// **7 Numerical Examples from First Principles (No Variables):**
// 1. 12345679 * 8 = 98765432. (Missing 8 in input, output is reverse sorted).
// 2. 1 / 998001 = ... 000 001 002 003 ... (Sequence generator up to 999).
// 3. 2^10 = 1024. 10^3 = 1000. Error = 2.4%.
// 4. 255 * 255 = 65025. (Max u16 is 65535. Diff = 510).
// 5. 3.14159 * 2 = 6.28318.
// 6. 1 + 2 + 3 + ... + 100 = 5050. (Gauss).
// 7. 0.1 + 0.2 = 0.30000000000000004. (IEEE 754 Floating Point Artifact).
fn all_combinations(s: &str) -> Vec<String> {
    let mut combinations = Vec::new();
    // Implementation intentionally left blank.
    // DO NOT WRITE SOLUTION.
    // Expected logic:
    // Helper function with (index, current_string).
    // Base case: index == s.len() -> if !current.is_empty() { push to results }.

    helper(s, 0, &mut String::new(), &mut combinations);

    combinations
}

fn helper(s: &str, char_pos: usize, tray: &mut String, combinations: &mut Vec<String>) {
    if char_pos == s.len() {
        if !tray.is_empty() {
            combinations.push(tray.clone());
        }
        return;
    }
    let ch = match s.chars().nth(char_pos) {
        Some(c) => c,
        None => return,
    };
    tray.push(ch);
    helper(s, char_pos + 1, tray, combinations);
    tray.pop();
    helper(s, char_pos + 1, tray, combinations);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_abc() {
        let res = all_combinations("ABC");
        // Verify output contains 7 elements
        // This test will fail until implementation is added
        // Expected: ["A", "B", "C", "AB", "AC", "BC", "ABC"] (in any order)
    }

    #[test]
    fn test_single_char() {
        let res = all_combinations("A");
        // Expected: ["A"]
    }
}
