// =============================================================================
// PROBLEM: Remove Length From Alphanumeric String
// =============================================================================
// Given: "JamesBond00712" where "12" is the length of "JamesBond007"
// Output: "JamesBond007"
// =============================================================================

/// YOUR FUNCTION. NO HINTS. FILL IN THE BLANKS IN THE MARKDOWN FIRST.
///
/// ## TRACE PUNISHMENT: "JamesBond00712"
///
/// ```text
/// MEMORY:
/// Address:  0x1000  0x1001  0x1002  0x1003  0x1004  0x1005  0x1006  0x1007
/// Index:       0       1       2       3       4       5       6       7
/// Byte:       74      97     109     101     115      66     111     110
/// Char:       'J'     'a'     'm'     'e'     's'     'B'     'o'     'n'
///
/// Address:  0x1008  0x1009  0x100A  0x100B  0x100C  0x100D
/// Index:       8       9      10      11      12      13
/// Byte:      100      48      48      55      49      50
/// Char:       'd'     '0'     '0'     '7'     '1'     '2'
///
/// s.len() = ???? (count the boxes yourself)
///
/// YOUR NAIVE APPROACH #1: Collect all trailing digits.
///   Start from index 13, go left until non-digit.
///   Index 13: byte 50. Is 50 >= 48 AND 50 <= 57? ____
///   Index 12: byte 49. Is 49 >= 48 AND 49 <= 57? ____
///   Index 11: byte 55. Is 55 >= 48 AND 55 <= 57? ____
///   Index 10: byte 48. Is 48 >= 48 AND 48 <= 57? ____
///   Index  9: byte 48. Is 48 >= 48 AND 48 <= 57? ____
///   Index  8: byte 100. Is 100 >= 48 AND 100 <= 57? ____
///
///   Collected bytes from 9 to 13: [48, 48, 55, 49, 50]
///   As string: "00712"
///   As integer: ????
///   Remaining after removal: indices 0..9 = "JamesBond" (wait, is 'd' at 8 included?)
///   
///   RECALCULATE: If collected starts at 9, remaining is 0..9 which is 9 chars.
///   Length = 9. Suffix as int = 712.
///   9 == 712? ____
///
///   CONCLUSION ABOUT NAIVE APPROACH #1: ____
///
/// YOUR NAIVE APPROACH #2: Just remove last 2 chars always.
///   Why 2? You don't know it's 2. You cheated by looking at the answer.
///   What if input was "Hello5"? Removing 2 gives "Hell" (length 4).
///   Is "o5" the length? "o5" is not even a valid int.
///   
///   CONCLUSION ABOUT NAIVE APPROACH #2: ____
///
/// NOW WHAT? Fill in the markdown blanks. Discover the pattern yourself.
/// ```
pub fn rem_length(s: &mut Vec<u8>) {
    // DO NOT WRITE ANYTHING HERE UNTIL YOU FILLED THE MARKDOWN.
    // DO NOT WRITE ANYTHING HERE UNTIL YOU TRACED 7 EXAMPLES BY HAND.
    // DO NOT WRITE ANYTHING HERE UNTIL YOU ANSWERED THE 3 QUESTIONS.

    let mut p : usize  = 0; 
    let mut curr_sum : usize = 0;
    for &c in s.iter().rev()
    {
        let cur_num = c - b'0';
        curr_sum        += (cur_num as usize) * 10_usize.pow(p as u32);
        p                +=1;
        if curr_sum == s.len() - p
        {
            s.truncate(curr_sum);
            return;
        }
    }
}

// =============================================================================
// TEST CASES - You can run these AFTER you implement.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_jamesbond() {
        // "JamesBond007" has 12 chars. Suffix "12". Total 14.
        let mut s: Vec<u8> = b"JamesBond00712".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"JamesBond007".to_vec());
    }

    #[test]
    fn test_02_hello5() {
        // "Hello" has 5 chars. Suffix "5". Total 6.
        let mut s: Vec<u8> = b"Hello5".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"Hello".to_vec());
    }

    #[test]
    fn test_03_zero() {
        // "" has 0 chars. Suffix "0". Total 1.
        let mut s: Vec<u8> = b"0".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"".to_vec());
    }

    #[test]
    fn test_04_two_digit_suffix() {
        // "abcdefghij" has 10 chars. Suffix "10". Total 12.
        let mut s: Vec<u8> = b"abcdefghij10".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"abcdefghij".to_vec());
    }

    #[test]
    fn test_05_one_one() {
        // "1" has 1 char. Suffix "1". Total 2.
        let mut s: Vec<u8> = b"11".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"1".to_vec());
    }

    #[test]
    fn test_06_two_one() {
        // "2" has 1 char. Suffix "1". Total 2.
        let mut s: Vec<u8> = b"21".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"2".to_vec());
    }

    #[test]
    fn test_07_abc003() {
        // "Abc" has 3 chars. Suffix "003" parses to 3. Total 6.
        let mut s: Vec<u8> = b"Abc003".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"Abc".to_vec());
    }

    #[test]
    fn test_08_agent007() {
        // "Agent007" has 8 chars. Suffix "8". Total 9.
        let mut s: Vec<u8> = b"Agent0078".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"Agent007".to_vec());
    }

    #[test]
    fn test_09_nine_x() {
        // "xxxxxxxxx" has 9 chars. Suffix "9". Total 10.
        let mut s: Vec<u8> = b"xxxxxxxxx9".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"xxxxxxxxx".to_vec());
    }

    #[test]
    fn test_10_ten_x() {
        // "xxxxxxxxxx" has 10 chars. Suffix "10". Total 12.
        let mut s: Vec<u8> = b"xxxxxxxxxx10".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"xxxxxxxxxx".to_vec());
    }

    #[test]
    fn test_11_single_char() {
        // "A" has 1 char. Suffix "1". Total 2.
        let mut s: Vec<u8> = b"A1".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"A".to_vec());
    }

    #[test]
    fn test_12_mixed_case() {
        // "HeLLo" has 5 chars. Suffix "5". Total 6.
        let mut s: Vec<u8> = b"HeLLo5".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"HeLLo".to_vec());
    }

    #[test]
    fn test_13_spaces() {
        // "Hi " has 3 chars. Suffix "3". Total 4.
        let mut s: Vec<u8> = b"Hi 3".to_vec();
        rem_length(&mut s);
        assert_eq!(s, b"Hi ".to_vec());
    }

    #[test]
    fn test_14_ninety_nine() {
        // 99 y's. Suffix "99". Total 101.
        let content = "y".repeat(99);
        let mut s: Vec<u8> = format!("{}99", content).into_bytes();
        rem_length(&mut s);
        assert_eq!(s, content.into_bytes());
    }

    #[test]
    fn test_15_hundred() {
        // 100 z's. Suffix "100". Total 103.
        let content = "z".repeat(100);
        let mut s: Vec<u8> = format!("{}100", content).into_bytes();
        rem_length(&mut s);
        assert_eq!(s, content.into_bytes());
    }
}
