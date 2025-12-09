// ============================================================================
// DISTINCT ANAGRAM GROUPS IN DICTIONARY
// ============================================================================
//
// PROBLEM: Read dictionary file, count distinct anagram groups.
//
// EXAMPLE:
//   File: ["listen", "silent", "enlist", "cat", "act", "dog"]
//   Groups:
//     "eilnst" → {listen, silent, enlist}
//     "act"    → {cat, act}
//     "dgo"    → {dog}
//   Answer: 3
//
// ============================================================================

use std::collections::HashMap;
use std::fs;

/// Count distinct anagram groups in dictionary.
///
/// INPUTS:
///   - file_path: path to dictionary file (one word per line)
///
/// OUTPUT:
///   - usize: count of distinct anagram groups
///
/// ALGORITHM (YOU IMPLEMENT):
///   1. Read file into Vec<String>
///   2. For each word:
///      - Normalize: lowercase, trim whitespace
///      - Sort characters → canonical key
///      - Insert into HashMap<key, Vec<words>>
///   3. Return hashmap.len()
///
/// TRAPS:
///   - Empty lines in file → filter them
///   - Case sensitivity → lowercase before sorting
///   - Whitespace → trim before processing
///
/// NUMERICAL TRACE:
///   words = ["Eat", "Tea", "Ate"]
///
///   word "Eat":
///     normalize → "eat"
///     sort chars: ['e','a','t'] → ['a','e','t']
///     key = "aet"
///     hashmap.entry("aet").or_insert(vec![]).push("eat")
///     state: {"aet": ["eat"]}
///
///   word "Tea":
///     normalize → "tea"
///     sort chars: ['t','e','a'] → ['a','e','t']
///     key = "aet"
///     hashmap["aet"].push("tea")
///     state: {"aet": ["eat", "tea"]}
///
///   word "Ate":
///     normalize → "ate"
///     sort chars: ['a','t','e'] → ['a','e','t']
///     key = "aet"
///     hashmap["aet"].push("ate")
///     state: {"aet": ["eat", "tea", "ate"]}
///
///   hashmap.len() = 1
///   RETURN 1
///
pub fn count_distinct_anagram_groups(file_path: &str) -> usize {

    let contents = match fs::read_to_string(file_path)
    {
        Err(_) => {
            panic!("error in file reading");
        }
        Ok(c) =>
        {
            c
        }
    };
    let lines:Vec<&str>  = contents.lines().filter(|line| !line.trim().is_empty())
    .collect();
    let mut keys : Vec<String> = lines.iter().
                map(|line| canonical_key(line))
                .collect();

    keys.sort();

    let mut count = 0 ;
    
    if keys.is_empty() {
        return 0;
    }

    for i in 0..keys.len() - 1 {
        if keys[i] != keys[i + 1] {
            count += 1;
        }
    }

    count + 1
}

/// Helper: compute canonical key for a word.
///
/// TRACE for "listen":
///   input: "listen"
///   chars: ['l','i','s','t','e','n']
///   ASCII: [108, 105, 115, 116, 101, 110]
///   sorted ASCII: [101, 105, 108, 110, 115, 116]
///   sorted chars: ['e','i','l','n','s','t']
///   output: "eilnst"
///
/// TRACE for "SILENT":
///   input: "SILENT"
///   lowercase: "silent"
///   chars: ['s','i','l','e','n','t']
///   ASCII: [115, 105, 108, 101, 110, 116]
///   sorted ASCII: [101, 105, 108, 110, 115, 116]
///   output: "eilnst"
///
/// NOTE: "listen" and "SILENT" produce same key "eilnst" → same group
///
pub fn canonical_key(word: &str) -> String {
    let lower:String = word.to_lowercase();
    let mut chars:Vec<char> = lower.chars().collect();
    chars.sort();
    let sorted:String = chars.into_iter().collect();
    sorted

}

// ============================================================================
// TEST CASES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_temp_file(words: &[&str]) -> String {
        let path = "/tmp/test_dictionary.txt";
        let mut file = fs::File::create(path).unwrap();
        for word in words {
            writeln!(file, "{}", word).unwrap();
        }
        path.to_string()
    }

    #[test]
    fn test_canonical_key_listen() {
        // "listen" → sort → "eilnst"
        assert_eq!(canonical_key("listen"), "eilnst");
    }

    #[test]
    fn test_canonical_key_silent() {
        // "silent" → sort → "eilnst"
        assert_eq!(canonical_key("silent"), "eilnst");
    }

    #[test]
    fn test_canonical_key_case_insensitive() {
        // "LISTEN" → lowercase → "listen" → sort → "eilnst"
        assert_eq!(canonical_key("LISTEN"), "eilnst");
    }

    #[test]
    fn test_canonical_key_cat() {
        // "cat" → sort → "act"
        assert_eq!(canonical_key("cat"), "act");
    }

    #[test]
    fn test_three_groups() {
        // listen, silent, enlist → 1 group
        // cat, act → 1 group
        // dog → 1 group
        // Total: 3 groups
        let path = create_temp_file(&["listen", "silent", "enlist", "cat", "act", "dog"]);
        assert_eq!(count_distinct_anagram_groups(&path), 3);
    }

    #[test]
    fn test_all_same_anagram() {
        // eat, tea, ate → all same group
        // Total: 1 group
        let path = create_temp_file(&["eat", "tea", "ate"]);
        assert_eq!(count_distinct_anagram_groups(&path), 1);
    }

    #[test]
    fn test_no_anagrams() {
        // apple, banana, cherry → 3 separate groups
        let path = create_temp_file(&["apple", "banana", "cherry"]);
        assert_eq!(count_distinct_anagram_groups(&path), 3);
    }

    #[test]
    fn test_single_word() {
        // Only "hello" → 1 group
        let path = create_temp_file(&["hello"]);
        assert_eq!(count_distinct_anagram_groups(&path), 1);
    }

    #[test]
    fn test_empty_file() {
        // Empty file → 0 groups
        let path = create_temp_file(&[]);
        assert_eq!(count_distinct_anagram_groups(&path), 0);
    }

    #[test]
    fn test_case_variations() {
        // "Listen", "SILENT", "EnLiSt" → all same group (case-insensitive)
        let path = create_temp_file(&["Listen", "SILENT", "EnLiSt"]);
        assert_eq!(count_distinct_anagram_groups(&path), 1);
    }

    #[test]
    fn test_with_empty_lines() {
        // File has empty lines mixed in
        // Should ignore empty lines
        let path = "/tmp/test_with_empty.txt";
        let mut file = fs::File::create(path).unwrap();
        writeln!(file, "cat").unwrap();
        writeln!(file, "").unwrap();  // empty line
        writeln!(file, "act").unwrap();
        writeln!(file, "   ").unwrap();  // whitespace only
        writeln!(file, "tac").unwrap();
        
        // cat, act, tac → 1 group
        assert_eq!(count_distinct_anagram_groups(path), 1);
    }

    #[test]
    fn test_larger_dictionary() {
        // 10 words, multiple groups
        // eat, tea, ate → group 1
        // tan, nat → group 2
        // bat, tab → group 3
        // god, dog → group 4
        // cat → group 5
        // Total: 5 groups
        let path = create_temp_file(&[
            "eat", "tea", "ate",
            "tan", "nat",
            "bat", "tab",
            "god", "dog",
            "cat"
        ]);
        assert_eq!(count_distinct_anagram_groups(&path), 5);
    }
}
