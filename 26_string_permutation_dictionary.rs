// 26. String Permutation Dictionary Lookup
// Find all permutations of input string that exist in dictionary
//
// COUNTING (do by hand):
// |s|=3 → 3!=6, |s|=4 → 4!=24, |s|=5 → 5!=120, |s|=6 → 6!=720
// |s|=7 → 7!=5040, |s|=8 → 8!=40320, |s|=10 → 10!=3628800
//
// DUPLICATE CHARS s="aab":
// 3!/2! = 6/2 = 3 unique perms: {aab, aba, baa}
// s="aaab" → 4!/3! = 24/6 = 4 unique perms
// s="aabb" → 4!/(2!×2!) = 24/4 = 6 unique perms
//
// SWAP TRACE s="ab":
// depth=0: swap(0,0)→"ab"→emit, swap(0,1)→"ba"→emit
//
// SWAP TRACE s="abc":
// depth=0,i=0: ['a','b','c'] → depth=1,i=1: ['a','b','c'] → depth=2: emit "abc"
//                                       i=2: ['a','c','b'] → depth=2: emit "acb"
//        i=1: ['b','a','c'] → depth=1,i=1: ['b','a','c'] → emit "bac"
//                                       i=2: ['b','c','a'] → emit "bca"
//        i=2: ['c','b','a'] → depth=1,i=1: ['c','b','a'] → emit "cba"
//                                       i=2: ['c','a','b'] → emit "cab"
//
// MEMORY TRACE:
// 0x100: ['a','b','c']
// call(0): swap(0,1) → 0x100: ['b','a','c']
// call(1): swap(1,2) → 0x100: ['b','c','a'] → emit "bca"
// return:  swap(1,2) → 0x100: ['b','a','c'] ← MUST RESTORE
// return:  swap(0,1) → 0x100: ['a','b','c'] ← MUST RESTORE
//
// FAILURE F10: You will forget swap_back after recursive call

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads dictionary from file into HashSet for O(1) lookup
/// Cost: O(W×L) where W=word_count, L=avg_length
/// 10000 words × 10 chars = 100000 char reads
fn load_dictionary(path: &str) -> HashSet<String> {
    // File::open("dictionary.txt") → Result<File>
    // BufReader::new(file) → buffered reader
    // .lines() → iterator of Result<String>
    // .filter_map(|l| l.ok()) → keep only Ok values
    // .collect() → HashSet<String>
    let file = File::open(path).expect("dictionary file not found");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect()
}

/// Generate all permutations of chars starting at index `start`
/// When start == chars.len(), one complete permutation exists
///
/// TRACE for chars=['a','b','c'], start=1:
/// i=1: swap(1,1)→no-op, recurse(2), emit, swap back
/// i=2: swap(1,2)→['a','c','b'], recurse(2), emit, swap back→['a','b','c']
fn backtrack(
    chars: &mut Vec<char>,
    start: usize,
    dictionary: &HashSet<String>,
    results: &mut Vec<String>,
) {
    // TODO: implement
    // Base: start == chars.len() → collect chars, check dictionary, push if found
    // Recurse: for i in start..chars.len() { swap(start,i), recurse(start+1), swap(start,i) }
    if start == chars.len() {
        let s: String = chars.iter().collect();
        if dictionary.contains(&s) {
            results.push(s);
            return;
        }
    }
    for i in start..chars.len() {
        chars.swap(start, i);
        backtrack(chars, start + 1, dictionary, results);
        chars.swap(start, i);
    }
}

/// Main entry: convert string to char array, call backtrack
///
/// s="abc", dict={"abc","bca","cab","xyz"}
/// → chars=['a','b','c']
/// → backtrack generates: abc,acb,bac,bca,cab,cba
/// → filter by dictionary: abc ✓, bca ✓, cab ✓
/// → return ["abc","bca","cab"]
pub fn find_valid_permutations(s: &str, dictionary: &HashSet<String>) -> Vec<String> {
    // TODO: implement
    // let mut chars: Vec<char> = s.chars().collect();
    // let mut results = Vec::new();
    // backtrack(&mut chars, 0, dictionary, &mut results);
    // results
    let mut chars: Vec<char> = s.chars().collect();
    let mut results = Vec::new();
    backtrack(&mut chars, 0, dictionary, &mut results);
    results
}

/// Alternative: check each dictionary word of length n if it's an anagram
/// Better when |dictionary| < n!
///
/// s="abc" (n=3, 3!=6)
/// dict has 1000 words of length 3
/// Method1: 6 perms × 6 lookups = 36 ops
/// Method2: 1000 words × O(n) anagram check = 3000 ops
/// ∴ Method1 wins when n! < |dict|
pub fn find_valid_permutations_alt(s: &str, dictionary: &HashSet<String>) -> Vec<String> {
    // TODO: implement alternative approach
    // For each word in dictionary:
    //   if word.len() == s.len() && is_anagram(word, s):
    //     results.push(word)
    todo!()
}

/// Check if two strings are anagrams
/// Method: sort both, compare OR use frequency count
///
/// "abc" vs "bca":
/// sorted: "abc" == "abc" ✓
///
/// "abc" vs "abd":
/// sorted: "abc" != "abd" ✗
///
/// Frequency approach:
/// "abc": [1,1,1,0,0,...] at indices 0,1,2
/// "bca": [1,1,1,0,0,...] → same ✓
fn is_anagram(a: &str, b: &str) -> bool {
    // Sort both, compare
    // "abc" → ['a','b','c'], "bca" → ['a','b','c'] → equal ✓
    if a.len() != b.len() {
        return false;
    }
    let mut a_chars: Vec<char> = a.chars().collect();
    let mut b_chars: Vec<char> = b.chars().collect();
    a_chars.sort();
    b_chars.sort();
    a_chars == b_chars
}

fn main() {
    // Example usage
    let dict: HashSet<String> = ["abc", "bca", "cab", "xyz", "dog"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let results = find_valid_permutations("abc", &dict);
    println!("Valid permutations: {:?}", results);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dict(words: &[&str]) -> HashSet<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    // ========== EDGE CASES ==========

    #[test]
    fn test_empty_string() {
        // s="" → 0!=1 → "" is the only permutation
        // F5: you will return 0 results instead of checking "" ∈ dict
        let dict = make_dict(&[""]);
        let result = find_valid_permutations("", &dict);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_empty_string_not_in_dict() {
        // s="" → "" is only perm, but "" ∉ dict
        let dict = make_dict(&["a", "b"]);
        let result = find_valid_permutations("", &dict);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_char() {
        // s="a" → 1!=1 → only "a"
        // F6: you will overcomplicate this
        let dict = make_dict(&["a", "b", "c"]);
        let result = find_valid_permutations("a", &dict);
        assert_eq!(result, vec!["a"]);
    }

    #[test]
    fn test_single_char_not_in_dict() {
        let dict = make_dict(&["b", "c"]);
        let result = find_valid_permutations("a", &dict);
        assert!(result.is_empty());
    }

    // ========== BASIC CASES ==========

    #[test]
    fn test_two_chars() {
        // s="ab" → 2!=2 → "ab","ba"
        let dict = make_dict(&["ab", "ba", "cd"]);
        let mut result = find_valid_permutations("ab", &dict);
        result.sort();
        assert_eq!(result, vec!["ab", "ba"]);
    }

    #[test]
    fn test_three_chars_partial_match() {
        // s="abc" → 3!=6 perms
        // dict has 3 of them
        let dict = make_dict(&["abc", "bca", "cab", "xyz", "dog"]);
        let mut result = find_valid_permutations("abc", &dict);
        result.sort();
        assert_eq!(result, vec!["abc", "bca", "cab"]);
    }

    #[test]
    fn test_three_chars_all_match() {
        // All 6 permutations in dictionary
        // F8: you will not test this case
        let dict = make_dict(&["abc", "acb", "bac", "bca", "cab", "cba"]);
        let mut result = find_valid_permutations("abc", &dict);
        result.sort();
        assert_eq!(result, vec!["abc", "acb", "bac", "bca", "cab", "cba"]);
    }

    #[test]
    fn test_three_chars_no_match() {
        // F7: empty result case
        let dict = make_dict(&["xyz", "dog", "cat"]);
        let result = find_valid_permutations("abc", &dict);
        assert!(result.is_empty());
    }

    // ========== DUPLICATE CHARACTERS ==========

    #[test]
    fn test_duplicate_chars_aab() {
        // s="aab" → 3!/2! = 3 unique perms: aab, aba, baa
        // F3: you will generate duplicates without HashSet
        let dict = make_dict(&["aab", "aba", "baa"]);
        let mut result = find_valid_permutations("aab", &dict);
        result.sort();
        result.dedup(); // Remove duplicates if any
        assert_eq!(result, vec!["aab", "aba", "baa"]);
    }

    #[test]
    fn test_duplicate_chars_aaa() {
        // s="aaa" → 3!/3! = 1 unique perm: aaa
        let dict = make_dict(&["aaa"]);
        let mut result = find_valid_permutations("aaa", &dict);
        result.dedup();
        assert_eq!(result, vec!["aaa"]);
    }

    #[test]
    fn test_duplicate_chars_aabb() {
        // s="aabb" → 4!/(2!×2!) = 24/4 = 6 unique perms
        // aabb, abab, abba, baab, baba, bbaa
        let dict = make_dict(&["aabb", "abab", "abba", "baab", "baba", "bbaa"]);
        let mut result = find_valid_permutations("aabb", &dict);
        result.sort();
        result.dedup();
        assert_eq!(result.len(), 6);
    }

    // ========== LARGER INPUTS ==========

    #[test]
    fn test_four_chars() {
        // s="abcd" → 4!=24 perms
        let dict = make_dict(&["abcd", "dcba", "badc"]);
        let mut result = find_valid_permutations("abcd", &dict);
        result.sort();
        assert_eq!(result, vec!["abcd", "badc", "dcba"]);
    }

    // ========== ANAGRAM CHECK TESTS ==========

    #[test]
    fn test_is_anagram_true() {
        assert!(is_anagram("abc", "bca"));
        assert!(is_anagram("abc", "cab"));
        assert!(is_anagram("listen", "silent"));
    }

    #[test]
    fn test_is_anagram_false() {
        assert!(!is_anagram("abc", "abd"));
        assert!(!is_anagram("abc", "abcd"));
        assert!(!is_anagram("aab", "abb"));
    }

    #[test]
    fn test_is_anagram_empty() {
        assert!(is_anagram("", ""));
    }

    // ========== DICTIONARY FILE TESTS ==========

    #[test]
    #[ignore] // Run with: cargo test --bin 26_string_permutation_dictionary -- --ignored
    fn test_with_real_dictionary() {
        let dict = load_dictionary("dictionary.txt");
        let result = find_valid_permutations("dog", &dict);
        println!("Permutations of 'dog' in dictionary: {:?}", result);
        // Expected: "dog", "god" (if in dictionary)
    }
}
