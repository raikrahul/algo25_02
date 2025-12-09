07 Remove Length From Alphanumeric String

PROBLEM
Input string contains content followed by its length as suffix.
Remove the length suffix and return only the content.

Input:  "JamesBond00712"
        Content: "JamesBond007" (length 12)
        Suffix:  "12"
Output: "JamesBond007"

FIGURE 1: Input Structure

Index:   0   1   2   3   4   5   6   7   8   9  10  11  12  13
Char:  [ J | a | m | e | s | B | o | n | d | 0 | 0 | 7 | 1 | 2 ]
       [=========== content ===========][suffix]

total_length = 14
content_length = ?
suffix_length = ?

Constraint: content_length = suffix_value

FIGURE 2: Why Greedy Digit Collection Fails

Input: "JamesBond00712"

Greedy approach: Collect all trailing digits.
  Index 13: '2' digit
  Index 12: '1' digit
  Index 11: '7' digit
  Index 10: '0' digit
  Index 9:  '0' digit
  Index 8:  'd' not digit, stop

Collected suffix: "00712" (5 digits)
Remaining: "JamesB" (6 chars)
Suffix as integer: 712
6 == 712? NO. Greedy fails.

Content contains digits ("007"). Cannot greedily collect.

FIGURE 3: Check-Based Approach

Attempt 1: Assume suffix is 1 digit.
  Remaining: s[0..13] = "JamesBond0071" (13 chars)
  Suffix: s[13..14] = "2"
  Parse "2" = 2
  13 == 2? NO. Reject.

Attempt 2: Assume suffix is 2 digits.
  Remaining: s[0..12] = "JamesBond007" (12 chars)
  Suffix: s[12..14] = "12"
  Parse "12" = 12
  12 == 12? YES. Accept.

Output: "JamesBond007"

FIGURE 4: Leading Zeros in Suffix

Input: "Abc003"

Attempt 3: Assume suffix is 3 digits.
  Remaining: s[0..3] = "Abc" (3 chars)
  Suffix: s[3..6] = "003"
  Parse "003" = 3
  3 == 3? YES. Accept.

Leading zeros vanish when parsed as integer.

FIGURE 5: Empty String Output

Input: "0"

Attempt 1: Assume suffix is 1 digit.
  Remaining: s[0..0] = "" (0 chars)
  Suffix: s[0..1] = "0"
  Parse "0" = 0
  0 == 0? YES. Accept.

Output: "" (empty string)
Empty string is valid content when suffix is "0".

FIGURE 6: Efficiency

For total_length = N:
  Suffix length <= log10(N) + 1
  Maximum attempts = log10(N) + 1

Example: N = 1000000
  Suffix length <= 7 digits
  Maximum 7 attempts
  O(log N) attempts. Efficient.

IMPLEMENTATION

fn remove_length(s: &str) -> &str {
    let n = s.len();
    for suffix_len in 1..=n {
        let content_len = n - suffix_len;
        let suffix = &s[content_len..];
        
        // Parse suffix as integer
        if let Ok(parsed) = suffix.parse::<usize>() {
            if parsed == content_len {
                return &s[0..content_len];
            }
        }
    }
    s // No valid split found
}
