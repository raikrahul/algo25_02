// ============================================================================
// ANAGRAM CHECK - TEST CASES ONLY - NO SOLUTION - NO HINTS
// ============================================================================

/// YOU IMPLEMENT THIS.
/// 
/// INPUTS: s1, s2 (two string slices)
/// OUTPUT: true if anagrams, false otherwise
/// 
/// NO HINTS PROVIDED. FIGURE IT OUT FROM THE EXERCISES IN anagram_check.md
pub fn is_anagram(s1: &str, s2: &str) -> bool {
    match s1.len() == s2.len()
    {
        false => return false,
        true  =>
        {
            let mut counter_array : [i32 ; 256] = [0; 256];

            for (chara, charb) in s1.chars().zip(s2.chars())
            {
                counter_array[chara as usize] +=1;
                counter_array[charb as usize ] -=1;
            }
            
            for c in counter_array
            {
                if (c != 0)
                {
                    return false;
                }
            }
            return true;

            }

        }   
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listen_silent() {
        assert_eq!(is_anagram("listen", "silent"), true);
    }

    #[test]
    fn test_anagram_nagaram() {
        assert_eq!(is_anagram("anagram", "nagaram"), true);
    }

    #[test]
    fn test_rat_tar() {
        assert_eq!(is_anagram("rat", "tar"), true);
    }

    #[test]
    fn test_rat_car() {
        assert_eq!(is_anagram("rat", "car"), false);
    }

    #[test]
    fn test_aab_abb() {
        assert_eq!(is_anagram("aab", "abb"), false);
    }

    #[test]
    fn test_abc_ab() {
        assert_eq!(is_anagram("abc", "ab"), false);
    }

    #[test]
    fn test_empty_empty() {
        assert_eq!(is_anagram("", ""), true);
    }

    #[test]
    fn test_empty_a() {
        assert_eq!(is_anagram("", "a"), false);
    }

    #[test]
    fn test_a_a() {
        assert_eq!(is_anagram("a", "a"), true);
    }

    #[test]
    fn test_a_b() {
        assert_eq!(is_anagram("a", "b"), false);
    }

    #[test]
    fn test_aaaa_aaaa() {
        assert_eq!(is_anagram("aaaa", "aaaa"), true);
    }

    #[test]
    fn test_aaabbbccc_abcabcabc() {
        assert_eq!(is_anagram("aaabbbccc", "abcabcabc"), true);
    }

    #[test]
    fn test_aabb_aaab() {
        assert_eq!(is_anagram("aabb", "aaab"), false);
    }

    #[test]
    fn test_case_sensitive() {
        assert_eq!(is_anagram("ABC", "abc"), false);
    }

    #[test]
    fn test_alphabet_reversed() {
        assert_eq!(
            is_anagram(
                "abcdefghijklmnopqrstuvwxyz",
                "zyxwvutsrqponmlkjihgfedcba"
            ),
            true
        );
    }
}
