/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
/// REVERSE WORDS IN SENTENCE - THE IN-PLACE CHARACTER ARRAY MASSACRE
/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
///
/// THE PROBLEM FROM THE MIDDLE: You have an array ['I','n','d','i','a',' ','i','s',' ','B','e','s','t'] at memory
/// addresses 0x1000 through 0x100C (that's 13 bytes: 0x100C - 0x1000 + 1 = 12 + 1 = 13 in hex math, or just
/// indices 0 through 12 inclusive meaning 12 - 0 + 1 = 13 slots). The word "India" occupies slots 0,1,2,3,4
/// which is 5 characters (4 - 0 + 1 = 5), the space occupies slot 5, the word "is" occupies slots 6,7 which
/// is 2 characters (7 - 6 + 1 = 2), another space at slot 8, and "Best" at slots 9,10,11,12 which is 4
/// characters (12 - 9 + 1 = 4). Sum check: 5 + 1 + 2 + 1 + 4 = 13. Matches array length. Good.
///
/// THE TWIST: You cannot allocate a new array. You must shuffle these 13 bytes around IN PLACE. If you try
/// to move "Best" (4 chars) to indices 0-3 and "India" (5 chars) to indices 8-12, you have a mismatch: "Best"
/// needs 4 slots starting at 0, then space at 4, then "is" needs slots 5-6, then space at 7, then "India"
/// needs 5 slots starting at 8 which goes 8,9,10,11,12 = 5 slots. Sum: 4 + 1 + 2 + 1 + 5 = 13. It fits! But
/// HOW do you move "Best" from slots 9-12 to slots 0-3 without destroying "India" at slots 0-4 which you
/// haven't saved anywhere?
///
/// THE TRICK (MIDDLE CALCULATION): If you reverse the ENTIRE array first, you get:
/// Original: ['I','n','d','i','a',' ','i','s',' ','B','e','s','t']
/// Reversed: ['t','s','e','B',' ','s','i',' ','a','i','d','n','I']
///
/// Swap count for full reverse: floor(13/2) = 6 swaps. Pairs: (0↔12), (1↔11), (2↔10), (3↔9), (4↔8), (5↔7).
/// Index 6 is the middle: 13/2 = 6.5, floor is 6, so index 6 is the center and untouched.
///
/// Now "tseB" is at 0-3 (backwards "Best"), "si" is at 5-6 (backwards "is"), "aidnI" is at 8-12 (backwards "India").
/// If you reverse each word INDIVIDUALLY:
/// - Reverse 0-3 (length 4, 4/2=2 swaps): t↔B, s↔e → "Best"
/// - Reverse 5-6 (length 2, 2/2=1 swap): s↔i → "is"
/// - Reverse 8-12 (length 5, 5/2=2 swaps): a↔I, i↔n, d stays → "India"
///
/// Result: ['B','e','s','t',' ','i','s',' ','I','n','d','i','a'] = "Best is India"
///
/// Total swaps: 6 + 2 + 1 + 2 = 11 operations for a 13-character array.
/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════


/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
/// REVERSE HELPER FUNCTION - THE CORE SWAP MACHINE
/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
///
/// NUMERICAL EXAMPLE 1 (MIDDLE-OUT): You call reverse(s, 3, 8). The segment is indices 3,4,5,6,7,8.
/// Length = 8 - 3 + 1 = 6. Swap count = 6 / 2 = 3.
///
/// Initial state at indices 3-8: ['d','i','a',' ','i','s'] (this is part of some bigger array)
/// left=3, right=8: swap s[3]='d' with s[8]='s' → ['s','i','a',' ','i','d'], left=4, right=7
/// left=4, right=7: swap s[4]='i' with s[7]=' ' → ['s',' ','a',' ','i','d'] WAIT NO. s[7]='i', s[4]='i'. 
///                   Let me recalculate. Original: s[3]='d', s[4]='i', s[5]='a', s[6]=' ', s[7]='i', s[8]='s'
///                   After swap(3,8): s[3]='s', s[8]='d' → ['s','i','a',' ','i','d']
///                   After swap(4,7): s[4]='i', s[7]='i' → ['s','i','a',' ','i','d'] (no visible change, same char)
///                   After swap(5,6): s[5]='a', s[6]=' ' → ['s','i',' ','a','i','d']
/// Final: indices 3-8 are now ['s','i',' ','a','i','d'] which is the reverse of ['d','i','a',' ','i','s'].
///
/// NUMERICAL EXAMPLE 2 (HARDER - BREAKING THE FLOW): reverse(s, 0, 0). Length = 0 - 0 + 1 = 1. Swaps = 1/2 = 0.
/// left=0, right=0: Is left < right? Is 0 < 0? NO. Loop doesn't run. Single element stays in place. Correct!
///
/// NUMERICAL EXAMPLE 3 (EDGE - EVEN LENGTH): reverse(s, 2, 5). Segment: s[2],s[3],s[4],s[5]. Length=4. Swaps=2.
/// Suppose s[2..=5] = ['a','b','c','d'].
/// left=2, right=5: swap s[2]='a' with s[5]='d' → ['d','b','c','a'], left=3, right=4
/// left=3, right=4: swap s[3]='b' with s[4]='c' → ['d','c','b','a'], left=4, right=3
/// left=4 < right=3? NO. Stop. Result: ['d','c','b','a']. Correct reversal!
///
/// NUMERICAL EXAMPLE 4 (USIZE TRAP): What if someone calls reverse(s, 5, 3)? That's left > right.
/// left=5, right=3: Is 5 < 3? NO. Loop doesn't run. Nothing happens. Is this correct? Depends on contract.
/// If we REQUIRE left <= right, caller's fault. If we want to handle it, we should check and swap left/right.
///
/// NUMERICAL EXAMPLE 5 (LARGE SCALE): reverse(s, 0, 99). Length = 100, swaps = 50.
/// Pairs: (0,99), (1,98), (2,97), ..., (48,51), (49,50). After 50 swaps, left=50, right=49. 50 < 49? NO. Stop.
///
/// NUMERICAL EXAMPLE 6 (FRACTIONAL THINKING): What if length was 7? 7/2 = 3.5 → 3 swaps in integer math.
/// Indices 0-6. Pairs: (0,6), (1,5), (2,4). Index 3 is center, untouched. 3 swaps total. Middle survives.
///
/// NUMERICAL EXAMPLE 7 (MID-START): reverse(s, 50, 55). Length = 6, swaps = 3.
/// If s[50..=55] = ['x','y','z','1','2','3']:
/// swap(50,55): x↔3 → ['3','y','z','1','2','x']
/// swap(51,54): y↔2 → ['3','2','z','1','y','x']
/// swap(52,53): z↔1 → ['3','2','1','z','y','x']
/// left=53, right=52. 53 < 52? NO. Done. Reversed correctly.
/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
fn reverse(s: &mut [char], mut left: usize, mut right: usize) {
    // ════════════════════════════════════════════════════════════════════════════
    // THE BRUTE FORCE REVERSAL
    // ════════════════════════════════════════════════════════════════════════════
    // Why 'mut left' and 'mut right'? 
    // Because we need to move them towards each other.
    //
    // TRACE for reverse(s, 2, 5) -> "a b c d" (indices 2,3,4,5)
    // ITERATION 1:
    //   left = 2, right = 5.   Condition (2 < 5) is TRUE.
    //   swap(s[2], s[5]).      "a"<->"d". Array becomes "d b c a".
    //   left becomes 3.
    //   right becomes 4.
    //
    // ITERATION 2:
    //   left = 3, right = 4.   Condition (3 < 4) is TRUE.
    //   swap(s[3], s[4]).      "b"<->"c". Array becomes "d c b a".
    //   left becomes 4.
    //   right becomes 3.
    //
    // ITERATION 3:
    //   left = 4, right = 3.   Condition (4 < 3) is FALSE.
    //   STOP.
    // ════════════════════════════════════════════════════════════════════════════
    
    // ════════════════════════════════════════════════════════════════════════════
    // YOUR TURN TO DERIVE THE LOOP
    // ════════════════════════════════════════════════════════════════════════════
    // Q1: In the trace for length 4 (indices 0,1,2,3), we stopped when left=2, right=1.
    //     What is the relationship between left and right at that moment?
    //
    // Q2: In the trace for length 5 (indices 0,1,2,3,4), we stopped when left=2, right=2.
    //     What is the relationship between left and right at that moment?
    //
    // Q3: Combine Q1 and Q2. What is the SINGLE condition that must be TRUE to KEEP GOING?
    //     (If it is false, we stop).
    //
    // TYPE THE LOOP HERE BASED ON Q3:
    // while ??? {
    //     s.swap(???, ???);
    //     left becomes ???
    //     right becomes ???
    // }
    // todo!("Type the while loop here based on your derivation");

    while left < right 
    {
        s.swap(left, right);
        left +=1;
        right -=1;
    }

}


/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
/// REVERSE WORDS FUNCTION - THE MAIN ORCHESTRATOR
/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
///
/// NUMERICAL TRACE (MIDDLE TO END TO START) FOR INPUT "A BC DEF":
///
/// INPUT ARRAY: ['A',' ','B','C',' ','D','E','F']
/// INDICES:       0   1   2   3   4   5   6   7
/// LENGTH: 8
///
/// STEP 1: FULL REVERSE (swap count = 8/2 = 4)
/// Pairs: (0,7), (1,6), (2,5), (3,4)
/// swap(0,7): 'A'↔'F' → ['F',' ','B','C',' ','D','E','A']
/// swap(1,6): ' '↔'E' → ['F','E','B','C',' ','D',' ','A']
/// swap(2,5): 'B'↔'D' → ['F','E','D','C',' ','B',' ','A']
/// swap(3,4): 'C'↔' ' → ['F','E','D',' ','C','B',' ','A']
///
/// AFTER FULL REVERSE: ['F','E','D',' ','C','B',' ','A']
/// Words are now backwards: "FED" (was "DEF"), "CB" (was "BC"), "A" (still "A")
///
/// STEP 2: FIND AND REVERSE EACH WORD
/// word_start = 0
/// i=0: s[0]='F' != ' ', continue
/// i=1: s[1]='E' != ' ', continue
/// i=2: s[2]='D' != ' ', continue
/// i=3: s[3]=' ' == ' ' → reverse(s, 0, 2), word_start = 4
///       reverse(0,2): swap(0,2) 'F'↔'D', swap(1,1) nothing → ['D','E','F',' ','C','B',' ','A']
/// i=4: s[4]='C' != ' ', continue
/// i=5: s[5]='B' != ' ', continue
/// i=6: s[6]=' ' == ' ' → reverse(s, 4, 5), word_start = 7
///       reverse(4,5): swap(4,5) 'C'↔'B' → ['D','E','F',' ','B','C',' ','A']
/// i=7: s[7]='A' != ' ', continue
/// AFTER LOOP: reverse(s, 7, 7) for last word (single char, no swap)
///
/// FINAL: ['D','E','F',' ','B','C',' ','A'] = "DEF BC A"
/// EXPECTED: "DEF BC A" ← Words of "A BC DEF" in reverse order. CORRECT!
///
/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
/// HARDER NUMERICAL EXAMPLE: "Hello World !"
///
/// INPUT: ['H','e','l','l','o',' ','W','o','r','l','d',' ','!']
/// LENGTH: 13 (same as "India is Best"!)
/// INDICES: 0  1  2  3  4  5  6  7  8  9 10 11 12
///
/// STEP 1: FULL REVERSE (13/2 = 6 swaps, index 6 is center)
/// Pairs: (0,12), (1,11), (2,10), (3,9), (4,8), (5,7)
/// swap(0,12): 'H'↔'!' 
/// swap(1,11): 'e'↔' '
/// swap(2,10): 'l'↔'d'
/// swap(3,9):  'l'↔'l' (same char, no visible change)
/// swap(4,8):  'o'↔'r'
/// swap(5,7):  ' '↔'o'
/// Index 6 'W' untouched.
///
/// AFTER FULL REVERSE: ['!', ' ', 'd', 'l', 'r', 'o', 'W', ' ', 'o', 'l', 'l', 'e', 'H']
/// Words: "!" at [0,0], "dlroW" at [2,6], "olleH" at [8,12]
/// Wait, space at index 1? So word 1 is just "!" at index 0!
/// Let me re-trace:
///   s[0]='!', s[1]=' ', s[2]='d', s[3]='l', s[4]='r', s[5]='o', s[6]='W', 
///   s[7]=' ', s[8]='o', s[9]='l', s[10]='l', s[11]='e', s[12]='H'
///
/// STEP 2:
/// word_start = 0
/// i=0: '!' != ' '
/// i=1: ' ' == ' ' → reverse(0,0) (single char, nothing), word_start = 2
/// i=2..6: 'd','l','r','o','W' != ' '
/// i=7: ' ' == ' ' → reverse(2,6) (length 5, 2 swaps: 2↔6='d'↔'W', 3↔5='l'↔'o')
///       ['!', ' ', 'W', 'o', 'r', 'l', 'd', ' ', 'o', 'l', 'l', 'e', 'H']
/// word_start = 8
/// i=8..12: 'o','l','l','e','H' != ' '
/// AFTER LOOP: reverse(8,12) (length 5, 2 swaps: 8↔12='o'↔'H', 9↔11='l'↔'e')
///       ['!', ' ', 'W', 'o', 'r', 'l', 'd', ' ', 'H', 'e', 'l', 'l', 'o']
///
/// FINAL: "! World Hello"
/// EXPECTED: "! World Hello" (words of "Hello World !" in reverse). CORRECT!
///
/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
/// EDGE CASE: EMPTY INPUT s.len() = 0
/// If you write `for i in 0..s.len()` → 0..0 → empty range, loop doesn't run. OK.
/// If you write `for i in 0..s.len()-1` → 0..(0-1) → 0..(usize::MAX) → PANIC or infinite loop!
/// ALWAYS CHECK s.is_empty() FIRST.
///
/// EDGE CASE: SINGLE CHARACTER s = ['X']
/// Full reverse: reverse(0,0) → no swap (0 < 0 is false)
/// Word detection: word_start=0, no spaces found, after loop reverse(0,0) → no swap
/// Output: ['X']. Same as input. Correct!
///
/// EDGE CASE: ALL SPACES s = [' ',' ',' ']
/// Full reverse: swap(0,2) ' '↔' ' (same), no visible change
/// Word detection: 
///   i=0: ' ' == ' ' → reverse(0,-1)? NO! word_start=0, end=i-1=-1. -1 is usize UNDERFLOW!
/// TRAP: When i=0 and s[i]==' ', end = i - 1 = 0 - 1 = usize underflow = crash.
/// SOLUTION: Only reverse if word_start < i (meaning there's at least one character in the word).
/// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
fn reverse_words(s: &mut [char]) {
    // DERIVATION STEP 1:
    // If we don't reverse the whole string first, and try to move "Best" (len 4) to "India" (len 5),
    // we have a size mismatch.
    // PROVE IT: Try to put 4 liters of water into a 5 liter bucket. You have 1 liter gap.
    // Try to put 5 liters into 4 liter bucket. Spill.
    // ACTION: Reverse the whole string to get the "blocks" in the right general area.
    
    // CODE STEP 1: Handle empty array (Trap #1: usize underflow if len is 0)
    // if ?? { return; }

    // CODE STEP 2: Reverse the whole thing.
    // reverse(s, 0, ??);

    // DERIVATION STEP 2:
    // Now we have "Best is India" but backwards: "tseB si aidnI".
    // We need to find the "holes" (words) and flip them back.
    
    // TRAP #2: The Last Word.
    // "tseB si aidnI" -> Space after 'B'? Yes. Space after 'i'? Yes. Space after 'I'? NO.
    // The loop that looks for ' ' will MISS the last word.
    
    // TRAP #3: The Multi-Space.
    // "A  B" -> Space at index 1. Space at index 2.
    // If you reverse the "word" between index 1 and 2, it is empty.
    // Should you reverse empty words? (See your reverse function trace for len 0).
    
    // WRITE YOUR LOOP HERE:
    // Track where the current word starts.
    // Scan `i` to find a space.
    // When found, reverse from start to `i-1`. (Why i-1? Because i is the space).
    // Update start.
    
    if s.is_empty() {
        return;
    }

    // now what 
    reverse(s, 0, s.len() - 1);
    // ════════════════════════════════════════════════════════════════════════════
    // ROASTING YOUR "CARELESS SLOPPINESS" (Line 269-277)
    // ════════════════════════════════════════════════════════════════════════════
    // ERROR 1: `for i in s.len()`
    //   `s.len()` is a NUMBER (e.g., 13).
    //   Rust asks: "How do I iterate over the number 13?"
    //   Answer: You cannot. You iterate over a RANGE. 0..13.
    //   FIX: `for i in 0..s.len()`
    //
    // ERROR 2: The Braces `{}` Logic Trap
    //   You wrote:
    //      if s[i] == ' ' { reverse(...) }
    //      }
    //      start = i + 1;
    //   Trace this with "A B":
    //   i=0 ('A'): if false. Loop ends. `start` becomes 0 + 1 = 1.
    //   i=1 (' '): if true. reverse(0, 0). `start` becomes 1 + 1 = 2.
    //   i=2 ('B'): if false. `start` becomes 2 + 1 = 3.
    //   RESULT: You are moving `start` on EVERY character, not just after spaces.
    //   FIX: Put `start = i + 1` INSIDE the if block.
    //
    // CORRECT LOGIC TRACE FOR "A B" (Indices 0, 1, 2):
    // start = 0
    // i=0 ('A'): != ' '. Do nothing.
    // i=1 (' '): == ' '.
    //    reverse(start=0, end=0). Word "A" reversed.
    //    start = i + 1 = 2.
    // i=2 ('B'): != ' '. Do nothing.
    // Loop ends.
    // ════════════════════════════════════════════════════════════════════════════

    let mut index_after_space = 0;
    
    // WHY RENAME 'i' TO 'cursor'? 
    // Because it scans through the string character by character.
    for cursor in 0..s.len() {
        
        // QUESTION 1: Why compare to space?
        // "tseB si aidnI" -> We need to know where "tseB" ends.
        // The space is the ONLY MARKER that tells us "Hey, the word ended!".
        // If we don't check for space, we don't know where to chop the words.
        if s[cursor] == ' ' {
            
            // QUESTION 2: Why check `cursor > index_after_space`?
            // VISUAL TRACE FOR "a     b" (Gap of 5 spaces)
            // 
            // 1. First space at index 1. `cursor`=1. `index_after_space`=0.
            //    Check: 1 > 0 is TRUE.
            //    Action: reverse(s, 0, 0). 'a' stays 'a'.
            //    Update: `index_after_space` becomes 1 + 1 = 2.
            //
            // 2. Second space at index 2. `cursor`=2.
            //    Check: 2 > 2 is FALSE.
            //    Action: SKIP! (This is the gap skipping magic).
            //    Update: `index_after_space` becomes 2 + 1 = 3.
            //
            // 3. Third space at index 3. `cursor`=3.
            //    Check: 3 > 3 is FALSE.
            //    Action: SKIP!
            //    Update: `index_after_space` becomes 3 + 1 = 4.
            //
            // CONCLUSION: The `index_after_space` keeps chasing `cursor` + 1.
            // As long as they are "touching" (cursor == index_after_space), implies 0 length word.
            // So we do nothing. We drive over the gap safely.
            if cursor > index_after_space {
                reverse(s, index_after_space, cursor - 1);
            }
            index_after_space = cursor + 1;
        }
    }
    
    // THE LAST WORD TRAP
    // Loop finished. "A B". Loop did 0..3.
    // i=2 was 'B'. No space. Loops ends.
    // `index_after_space` is 2. `s.len()` is 3.
    // We have characters from 2 to 2 ('B') that were never reversed.
    
    // DEBUG: PROVING THE TRAP
    // println!("State before final fix: {:?}", s);
    // println!("Start index: {}, Length: {}", index_after_space, s.len());
    
    // ACTION: Reverse from `index_after_space` to `s.len() - 1`.
    if index_after_space < s.len() {
        reverse(s, index_after_space, s.len() - 1);
    }



}


// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
// TEST SUITE - YOUR PREDICTIONS WILL BE TESTED HERE
// ═══════════════════════════════════════════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 1: The exact example from the problem statement
    /// Input: "India is Best" (13 chars)
    /// Expected: "Best is India"
    #[test]
    fn test_india_is_best() {
        let mut s: Vec<char> = "India is Best".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "Best is India");
    }

    /// TEST 2: Two words only
    /// Input: "Hello World" (11 chars)
    /// Expected: "World Hello"
    #[test]
    fn test_two_words() {
        let mut s: Vec<char> = "Hello World".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "World Hello");
    }

    /// TEST 3: Single word - nothing to reverse between words
    /// Input: "Rust" (4 chars)
    /// Expected: "Rust" (unchanged)
    #[test]
    fn test_single_word() {
        let mut s: Vec<char> = "Rust".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "Rust");
    }

    /// TEST 4: Empty input - the usize underflow trap
    /// Input: "" (0 chars)
    /// Expected: "" (empty)
    #[test]
    fn test_empty_input() {
        let mut s: Vec<char> = "".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "");
    }

    /// TEST 5: Single character
    /// Input: "X" (1 char)
    /// Expected: "X" (unchanged)
    #[test]
    fn test_single_char() {
        let mut s: Vec<char> = "X".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "X");
    }

    /// TEST 6: Four words - the trace from the code comments
    /// Input: "A BC DEF GHIJ" (14 chars)
    /// Expected: "GHIJ DEF BC A"
    #[test]
    fn test_four_words() {
        let mut s: Vec<char> = "A BC DEF GHIJ".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "GHIJ DEF BC A");
    }

    /// TEST 7: Leading space - word_start handling trap
    /// Input: " Hello" (6 chars, space at index 0)
    /// Expected: "Hello " (space moves to end)
    #[test]
    fn test_leading_space() {
        let mut s: Vec<char> = " Hello".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "Hello ");
    }

    /// TEST 8: Trailing space
    /// Input: "Hello " (6 chars, space at index 5)
    /// Expected: " Hello" (space moves to start)
    #[test]
    fn test_trailing_space() {
        let mut s: Vec<char> = "Hello ".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, " Hello");
    }

    /// TEST 9: Multiple consecutive spaces
    /// Input: "A  B" (4 chars, two spaces between)
    /// Expected: "B  A" (spaces preserved in middle)
    #[test]
    fn test_multiple_spaces() {
        let mut s: Vec<char> = "A  B".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "B  A");
    }

    /// TEST 10: All spaces - edge case
    /// Input: "   " (3 spaces)
    /// Expected: "   " (unchanged, just spaces)
    #[test]
    fn test_all_spaces() {
        let mut s: Vec<char> = "   ".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "   ");
    }

    /// TEST 11: Even length string
    /// Input: "AB CD" (5 chars - wait, 5 is odd. Let me count: A-B-space-C-D = 5)
    /// Let's use: "AB CDEF" = 7 chars. Still odd.
    /// "AB CD EF" = 8 chars. Even!
    /// Expected: "EF CD AB"
    #[test]
    fn test_even_length() {
        let mut s: Vec<char> = "AB CD EF".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "EF CD AB");
    }

    /// TEST 12: Odd length string to verify center element handling
    /// Input: "A B C" (5 chars, center is 'B' at index 2)
    /// Wait: A-space-B-space-C = 5 chars. Indices 0,1,2,3,4.
    /// Full reverse: swap(0,4)='A'↔'C', swap(1,3)=' '↔' '. Index 2='B' untouched.
    /// After full reverse: "C B A"
    /// Words after full reverse: "C" at [0,0], "B" at [2,2], "A" at [4,4]. Already single chars!
    /// No inner reversal needed. Output: "C B A"
    #[test]
    fn test_odd_length_center() {
        let mut s: Vec<char> = "A B C".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "C B A");
    }

    /// TEST 13: Long sentence
    /// Input: "The quick brown fox jumps"
    /// Expected: "jumps fox brown quick The"
    #[test]
    fn test_long_sentence() {
        let mut s: Vec<char> = "The quick brown fox jumps".chars().collect();
        reverse_words(&mut s);
        let result: String = s.into_iter().collect();
        assert_eq!(result, "jumps fox brown quick The");
    }
}

fn main() {
    // Manual testing area
    let mut s: Vec<char> = "India is Best".chars().collect();
    println!("Before: {:?}", s);
    reverse_words(&mut s);
    println!("After:  {:?}", s);
    let result: String = s.into_iter().collect();
    println!("Result: {}", result);
}
