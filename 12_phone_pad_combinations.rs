// phone_pad_combinations.rs
//
// ┌─────────────────────────────────────────────────────────────────────────────────────────┐
// │ BEFORE YOU WRITE ANY CODE:                                                             │
// │                                                                                        │
// │ 1. Open phone_pad_combinations.md                                                      │
// │ 2. Fill in ALL blanks with your own calculations                                       │
// │ 3. Draw ALL trees by hand                                                              │
// │ 4. List ALL outputs for "199" by hand (12 strings)                                     │
// │ 5. Answer ALL 7 questions at the bottom                                                │
// │                                                                                        │
// │ DO NOT PROCEED UNTIL YOU HAVE DONE THE ABOVE.                                          │
// └─────────────────────────────────────────────────────────────────────────────────────────┘

// ═══════════════════════════════════════════════════════════════════════════════════════════════════════
// generate_helper: recursive function that builds all combinations
// ═══════════════════════════════════════════════════════════════════════════════════════════════════════
//
// EXAMPLE TRACE FOR input = "19":
//   CALL #0: digit_position=0, current_buffer="", results=[]
//   CALL #1: digit_position=1, current_buffer="a", results=[]
//   CALL #2: digit_position=2, current_buffer="ay", results=[] → BASE CASE → results=["ay"]
//   CALL #3: digit_position=2, current_buffer="az", results=["ay"] → BASE CASE → results=["ay","az"]
//   CALL #4: digit_position=1, current_buffer="b", results=["ay","az"]
//   ...
//   CALL #9: digit_position=2, current_buffer="cz" → results=["ay","az","by","bz","cy","cz"]
//
// TOTAL CALLS: 1 + 3 + 6 = 10
//   - 1 call at digit_position=0
//   - 3 calls at digit_position=1 (one per letter in mappings[1]=['a','b','c'])
//   - 6 calls at digit_position=2 (one per output, 3×2=6)
//
// ═══════════════════════════════════════════════════════════════════════════════════════════════════════
fn generate_helper(
    phone_number: &str,           // "19" at address 0x7fff0000, len=2, chars=['1','9']
                                  // "199" at address 0x7fff0000, len=3, chars=['1','9','9']
                                  // "12345" at address 0x7fff0000, len=5 → 3^5=243 outputs
    mappings: &[Vec<char>],       // 10 Vecs: mappings[0]=[], mappings[1]=['a','b','c'], ..., mappings[9]=['y','z']
                                  // mappings[1].len()=3, mappings[9].len()=2 ← TRAP: NOT ALL HAVE 3!
    digit_position: usize,        // 0 → processing phone_number[0]
                                  // 1 → processing phone_number[1]
                                  // 2 → phone_number.len()=2 → BASE CASE
                                  // for "199": 0,1,2,3 where 3=BASE CASE
    current_buffer: &mut String,  // "" → "a" → "ay" → "a" → "az" → "" → "b" → "by" → ...
                                  // address 0x5555_0000, capacity grows as needed
                                  // max len = phone_number.len() (2 for "19", 3 for "199")
    results: &mut Vec<String>     // starts [], ends ["ay","az","by","bz","cy","cz"] for "19"
                                  // len goes 0 → 1 → 2 → 3 → 4 → 5 → 6
                                  // for "199": len goes 0 → 1 → 2 → ... → 12
)
{
    // ═══════════════════════════════════════════════════════════════════════════════════════════════════
    // BASE CASE CHECK
    // ═══════════════════════════════════════════════════════════════════════════════════════════════════
    // 
    // digit_position vs phone_number.len():
    //   for "19": phone_number.len() = 2
    //     digit_position=0: 0 == 2 → false → continue
    //     digit_position=1: 1 == 2 → false → continue
    //     digit_position=2: 2 == 2 → TRUE → base case!
    //
    //   for "199": phone_number.len() = 3
    //     digit_position=0: 0 == 3 → false
    //     digit_position=1: 1 == 3 → false
    //     digit_position=2: 2 == 3 → false
    //     digit_position=3: 3 == 3 → TRUE → base case!
    //
    // BASE CASE = we've processed ALL digits, current_buffer is complete
    //
    if digit_position == phone_number.len()
    {
        // ═══════════════════════════════════════════════════════════════════════════════════════════════
        // current_buffer.clone():
        //   current_buffer = "ay" at address 0x5555_0000
        //   .clone() creates NEW String at address 0x5555_1000 with same content "ay"
        //   WHY CLONE? results.push() takes ownership, but we need current_buffer for pop() later
        //   WAIT: we return immediately, so no pop() needed here. Clone still needed because
        //         results takes String, not &String
        //
        // results state progression for "19":
        //   results.push("ay") → results = ["ay"], len=1
        //   results.push("az") → results = ["ay","az"], len=2
        //   results.push("by") → results = ["ay","az","by"], len=3
        //   results.push("bz") → results = ["ay","az","by","bz"], len=4
        //   results.push("cy") → results = ["ay","az","by","bz","cy"], len=5
        //   results.push("cz") → results = ["ay","az","by","bz","cy","cz"], len=6
        //
        // ═══════════════════════════════════════════════════════════════════════════════════════════════
        results.push(current_buffer.clone()); 
        return;  // exit this call, go back to caller (the recursive call site)
    }

    // ═══════════════════════════════════════════════════════════════════════════════════════════════════
    // GET CURRENT DIGIT CHARACTER
    // ═══════════════════════════════════════════════════════════════════════════════════════════════════
    //
    // phone_number.chars().nth(digit_position):
    //   phone_number = "19"
    //   phone_number.chars() → iterator over ['1', '9']
    //   .nth(0) → Some('1')
    //   .nth(1) → Some('9')
    //   .nth(2) → None (past end)
    //
    // BUT: we already checked digit_position == phone_number.len() above
    //      so digit_position < phone_number.len() here
    //      so nth() will always return Some(...), never None
    //      still use if-let to avoid unwrap()
    //
    if let Some(digit_char) = phone_number.chars().nth(digit_position)
    {
        // ═══════════════════════════════════════════════════════════════════════════════════════════════
        // CONVERT CHAR TO INDEX
        // ═══════════════════════════════════════════════════════════════════════════════════════════════
        //
        // digit_char = '1' (character, not number)
        // digit_char as u8 = 49 (ASCII code for '1')
        // b'0' = 48 (ASCII code for '0')
        // 49 - 48 = 1
        // 1 as usize = 1
        // digit_index = 1
        //
        // digit_char = '9'
        // digit_char as u8 = 57
        // 57 - 48 = 9
        // digit_index = 9
        //
        // ASCII TABLE EXTRACT:
        //   '0' = 48, '1' = 49, '2' = 50, '3' = 51, '4' = 52
        //   '5' = 53, '6' = 54, '7' = 55, '8' = 56, '9' = 57
        //
        let digit_index = (digit_char as u8 - b'0') as usize;

        // ═══════════════════════════════════════════════════════════════════════════════════════════════
        // GET LETTERS FOR THIS DIGIT
        // ═══════════════════════════════════════════════════════════════════════════════════════════════
        //
        // digit_index = 1 → mappings[1] = ['a','b','c'] → letters = &['a','b','c']
        // digit_index = 9 → mappings[9] = ['y','z'] → letters = &['y','z'] ← ONLY 2!
        //
        // letters.len():
        //   mappings[1].len() = 3
        //   mappings[2].len() = 3
        //   ...
        //   mappings[9].len() = 2 ← TRAP!
        //
        // if you hardcode `for i in 0..3` → crash for digit 9 at i=2
        //
        let letters = &mappings[digit_index];

        // ═══════════════════════════════════════════════════════════════════════════════════════════════
        // LOOP THROUGH EACH LETTER
        // ═══════════════════════════════════════════════════════════════════════════════════════════════
        //
        // for "19" at digit_position=0:
        //   letters = ['a','b','c']
        //   ITER 1: l = &'a' → *l = 'a'
        //   ITER 2: l = &'b' → *l = 'b'
        //   ITER 3: l = &'c' → *l = 'c'
        //
        // for "19" at digit_position=1:
        //   letters = ['y','z']
        //   ITER 1: l = &'y' → *l = 'y'
        //   ITER 2: l = &'z' → *l = 'z'
        //   (NO ITER 3 - only 2 letters!)
        //
        for l in letters {
            // ═══════════════════════════════════════════════════════════════════════════════════════════
            // PUSH LETTER TO BUFFER
            // ═══════════════════════════════════════════════════════════════════════════════════════════
            //
            // current_buffer BEFORE push:
            //   at digit_position=0, ITER 1: current_buffer = "", len=0
            //   at digit_position=1, ITER 1: current_buffer = "a", len=1
            //
            // current_buffer.push(*l):
            //   *l = dereference: l is &char, *l is char
            //   push appends char to end of String
            //
            // current_buffer AFTER push:
            //   at digit_position=0, ITER 1: current_buffer = "a", len=1
            //   at digit_position=1, ITER 1: current_buffer = "ay", len=2
            //
            // MEMORY:
            //   current_buffer at 0x5555_0000: [97, 121, _, _, _, _, _, _]
            //                                   'a'  'y'
            //   len = 2, capacity = 8
            //
            current_buffer.push(*l);

            // ═══════════════════════════════════════════════════════════════════════════════════════════
            // RECURSE
            // ═══════════════════════════════════════════════════════════════════════════════════════════
            //
            // digit_position + 1:
            //   0 + 1 = 1 → process next digit
            //   1 + 1 = 2 → for "19", this equals len, so next call hits BASE CASE
            //
            // ALL OTHER PARAMS UNCHANGED:
            //   phone_number = "19" (same reference)
            //   mappings = same reference
            //   current_buffer = same reference (now contains "a" or "ay")
            //   results = same reference (accumulating outputs)
            //
            // CALL STACK at max depth (processing "ay"):
            //   ┌─────────────────────────────────────────────┐
            //   │ generate_helper(..., 2, "ay", [...])        │ ← BASE CASE
            //   ├─────────────────────────────────────────────┤
            //   │ generate_helper(..., 1, "ay", [...])        │ ← ITER 1 of inner loop
            //   ├─────────────────────────────────────────────┤
            //   │ generate_helper(..., 0, "ay", [...])        │ ← ITER 1 of outer loop
            //   └─────────────────────────────────────────────┘
            //   stack depth = 3 = phone_number.len() + 1
            //
            generate_helper(phone_number, mappings, digit_position + 1, current_buffer, results);

            // ═══════════════════════════════════════════════════════════════════════════════════════════
            // POP LETTER FROM BUFFER (BACKTRACK)
            // ═══════════════════════════════════════════════════════════════════════════════════════════
            //
            // WHY POP?
            //   WITHOUT POP:
            //     buf = "ay" → OUTPUT "ay"
            //     buf.push('z') → buf = "ayz" → OUTPUT "ayz" ← WRONG!
            //   WITH POP:
            //     buf = "ay" → OUTPUT "ay"
            //     buf.pop() → buf = "a"
            //     buf.push('z') → buf = "az" → OUTPUT "az" ← CORRECT!
            //
            // pop() removes last character:
            //   current_buffer = "ay" → pop() → current_buffer = "a"
            //   current_buffer = "a" → pop() → current_buffer = ""
            //
            // WHEN DOES BUFFER BECOME EMPTY?
            //   after ITER 1 of outer loop completes: pop 'a' → ""
            //   after ITER 2 of outer loop completes: pop 'b' → ""
            //   after ITER 3 of outer loop completes: pop 'c' → ""
            //
            // BUFFER STATES FOR "19":
            //   "" → "a" → "ay" → OUTPUT → "a" → "az" → OUTPUT → "a" → ""
            //      → "b" → "by" → OUTPUT → "b" → "bz" → OUTPUT → "b" → ""
            //      → "c" → "cy" → OUTPUT → "c" → "cz" → OUTPUT → "c" → ""
            //
            current_buffer.pop();
        }
    }
}

pub fn generate_combinations(phone_number: &str, mappings: &[Vec<char>]) -> Vec<String> {
    // ┌────────────────────────────────────────────────────────────────────────────────────┐
    // │ YOU IMPLEMENT HERE                                                                 │
    // │                                                                                    │
    // │ First answer these by writing on paper:                                            │
    // │                                                                                    │
    // │ Q1: input = "23", digit = input.chars().nth(0) = ???                               │
    // │     digit as u8 = ???                                                              │
    // │     '0' as u8 = ???                                                                │
    // │     (digit as u8 - '0' as u8) = ???                                                │
    // │     mappings[???] = ???                                                            │
    // │                                                                                    │
    // │ Q2: If I have string "dg" and want string "dh":                                    │
    // │     What do I do to "dg"? ____________________                                     │
    // │     Then what? ____________________                                                │
    // │                                                                                    │
    // │ Q3: If I have string "di" and want string "eg":                                    │
    // │     What do I do to "di"? ____________________                                     │
    // │     Then? ____________________                                                     │
    // │     Then? ____________________                                                     │
    // │                                                                                    │
    // │ Q4: mappings[9].len() = ???                                                        │
    // │     If I write `for j in 0..3` and digit is '9', what happens at j=2? ______       │
    // │                                                                                    │
    // │ Q5: When do I output? (describe the condition, not the code)                       │
    // │     ____________________________________________________________________           │
    // │                                                                                    │
    // └────────────────────────────────────────────────────────────────────────────────────┘
    // todo!("Complete the markdown exercises first, then implement")

    let mut results : Vec<String>   =   Vec::new();
    let mut current_buffer :String  =   String::new();

    generate_helper(phone_number, mappings, 0, &mut current_buffer, &mut results);
    results
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn get_mappings() -> Vec<Vec<char>> {
        vec![
            vec![],                     // 0: unused
            vec!['a', 'b', 'c'],        // 1
            vec!['d', 'e', 'f'],        // 2
            vec!['g', 'h', 'i'],        // 3
            vec!['j', 'k', 'l'],        // 4
            vec!['m', 'n', 'o'],        // 5
            vec!['p', 'q', 'r'],        // 6
            vec!['s', 't', 'u'],        // 7
            vec!['v', 'w', 'x'],        // 8
            vec!['y', 'z'],             // 9: YOU NOTICED THIS HAS 2?
        ]
    }
    
    #[test]
    fn test_single_digit_1() {
        let mappings = get_mappings();
        let result = generate_combinations("1", &mappings);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"b".to_string()));
        assert!(result.contains(&"c".to_string()));
    }
    
    #[test]
    fn test_single_digit_9() {
        // TRAP: mappings[9] has only 2 letters
        let mappings = get_mappings();
        let result = generate_combinations("9", &mappings);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"y".to_string()));
        assert!(result.contains(&"z".to_string()));
    }
    
    #[test]
    fn test_two_digits_23() {
        // YOU calculated: 3 × 3 = ??? outputs
        // YOU listed: dg, dh, di, eg, eh, ei, fg, fh, fi
        let mappings = get_mappings();
        let result = generate_combinations("23", &mappings);
        assert_eq!(result.len(), 9);
        let expected = vec!["dg","dh","di","eg","eh","ei","fg","fh","fi"];
        for exp in expected {
            assert!(result.contains(&exp.to_string()), "Missing: {}", exp);
        }
    }
    
    #[test]
    fn test_three_digits_283() {
        // YOU calculated: ??? × ??? × ??? = ??? outputs
        let mappings = get_mappings();
        let result = generate_combinations("283", &mappings);
        assert_eq!(result.len(), 27);
        assert!(result.contains(&"dvg".to_string()));
        assert!(result.contains(&"fxi".to_string()));
    }
    
    #[test]
    fn test_mixed_199() {
        // YOU calculated: 3 × 2 × 2 = ??? outputs
        // YOU listed all 12 by hand in the markdown
        let mappings = get_mappings();
        let result = generate_combinations("199", &mappings);
        assert_eq!(result.len(), 12);
        let expected = vec![
            "ayy", "ayz", "azy", "azz",
            "byy", "byz", "bzy", "bzz",
            "cyy", "cyz", "czy", "czz",
        ];
        for exp in expected {
            assert!(result.contains(&exp.to_string()), "Missing: {}", exp);
        }
    }
    
    #[test]
    fn test_99() {
        // YOU calculated: 2 × 2 = ??? outputs
        let mappings = get_mappings();
        let result = generate_combinations("99", &mappings);
        assert_eq!(result.len(), 4);
        let expected = vec!["yy", "yz", "zy", "zz"];
        for exp in expected {
            assert!(result.contains(&exp.to_string()), "Missing: {}", exp);
        }
    }
    
    #[test]
    fn test_empty() {
        // YOU decided: 0 or 1 output?
        let mappings = get_mappings();
        let result = generate_combinations("", &mappings);
        assert!(result.is_empty() || result == vec!["".to_string()]);
    }
    
    #[test]
    fn test_12345() {
        // YOU calculated: 3^5 = ??? outputs
        let mappings = get_mappings();
        let result = generate_combinations("12345", &mappings);
        assert_eq!(result.len(), 243);
        for combo in &result {
            assert_eq!(combo.len(), 5);
        }
    }
}
