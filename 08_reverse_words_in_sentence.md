08 Reverse Words in Sentence

PROBLEM
Reverse word order in a character array in-place.
Input:  India is Best
Output: Best is India

FIGURE 1: Why Direct Swap Fails
Words have different lengths. Direct swap corrupts memory.

Input:  [I,n,d,i,a, ,i,s, ,B,e,s,t]
        Word1: 5 chars    Word3: 4 chars

Copy "Best" to index 0? Overwrites "Indi" and leaves 'a' at index 4.
Shift all characters? O(N^2) time. Forbidden.

FIGURE 2: Two-Reversal Technique

Step 1: Reverse entire array.
  Before: [I,n,d,i,a, ,i,s, ,B,e,s,t]
  After:  [t,s,e,B, ,s,i, ,a,i,d,n,I]

Observe: Word positions correct. Internal order wrong.
  "tseB" at front (should be "Best")
  "aidnI" at back (should be "India")

Step 2: Reverse each word individually.
  reverse(0,3): [B,e,s,t, ,s,i, ,a,i,d,n,I]
  reverse(5,6): [B,e,s,t, ,i,s, ,a,i,d,n,I]
  reverse(8,12): [B,e,s,t, ,i,s, ,I,n,d,i,a]

Result: "Best is India". Correct.

FIGURE 3: Reversal Loop Termination

Even length (4 chars):
  L=0, R=3. Swap. L=1, R=2. Swap. L=2, R=1. Stop (2<1 false).

Odd length (5 chars):
  L=0, R=4. Swap. L=1, R=3. Swap. L=2, R=2. Stop (2<2 false).
  Middle element (index 2) untouched.

Condition: while L < R

FIGURE 4: Double Space Trap

Input: [A, , ,B]  (indices 0,1,2,3)
       Space at 1. Space at 2.

cursor=1, idx_after_space=0. 1>0? Yes. reverse(0,0). idx=2.
cursor=2, idx_after_space=2. 2>2? No. Skip. idx=3.
cursor=3. Not space. Skip.
Final: idx=3, len=4. 3<4? Yes. reverse(3,3).

Skipped empty word between consecutive spaces.

FIGURE 5: Trailing Space Trap

Input: [A, ]  (indices 0,1)

cursor=1, idx_after_space=0. 1>0? Yes. reverse(0,0). idx=2.
Loop ends.
Final: idx=2, len=2. 2<2? No. Skip.

No phantom reversal attempted after trailing space.

FIGURE 6: Full Trace with Gaps

Input: "        ab      cd      " (8+2+6+2+6 = 24 chars)

Phase 1 (Full Reverse):
  Before: "        ab      cd      "
  After:  "      dc      ba        "

Phase 2 (Word Scanner):
  cursor 0-5: All spaces. idx catches up. No reverse.
  cursor 6-7: 'dc'. Not space.
  cursor 8: Space. 8>6? Yes. reverse(6,7). "dc"->"cd". idx=9.
  cursor 9-13: Spaces. idx catches up. No reverse.
  cursor 14-15: 'ba'. Not space.
  cursor 16: Space. 16>14? Yes. reverse(14,15). "ba"->"ab". idx=17.
  cursor 17-23: Spaces. idx catches up. No reverse.
  Final: idx=24, len=24. 24<24? No. Skip.

Result: "      cd      ab        "

IMPLEMENTATION

fn reverse(s: &mut [char], mut l: usize, mut r: usize) {
    while l < r {
        s.swap(l, r);
        l += 1;
        r -= 1;
    }
}

fn reverse_words(s: &mut [char]) {
    if s.is_empty() { return; }
    
    // Step 1: Reverse entire array
    reverse(s, 0, s.len() - 1);
    
    // Step 2: Reverse each word
    let mut idx = 0;
    for cursor in 0..s.len() {
        if s[cursor] == ' ' {
            if cursor > idx {
                reverse(s, idx, cursor - 1);
            }
            idx = cursor + 1;
        }
    }
    
    // Handle last word
    if idx < s.len() {
        reverse(s, idx, s.len() - 1);
    }
}
