# Cyclic Permutation Detection

## The Core Annoyance: Why Your Brain Will Betray You

The problem demands you check if s2 is a cyclic rotation of s1, meaning CDAB is ABCD rotated by 2 positions to the left, so A moves to position 2, B moves to position 3, C moves to position 0, D moves to position 1, and the final string reads C-D-A-B from left to right, which is exactly s2, so the answer is true because every character in s1 exists in s2 at a shifted position that wraps around modularly.

---

## Drawing 1: What Does "Cyclic" Actually Mean?

```
s1 = "ABCD"  (Length = 4)

Position:   0   1   2   3
           +---+---+---+---+
           | A | B | C | D |
           +---+---+---+---+

Rotation by 0:  ABCD  (no change)
Rotation by 1:  BCDA  (A moves to end)
Rotation by 2:  CDAB  (A,B move to end)
Rotation by 3:  DABC  (A,B,C move to end)
Rotation by 4:  ABCD  (full circle = rotation by 0)
```

**Why this drawing?** Because you will forget that rotation by k means taking the first k characters from the front and appending them to the back, not the other way around, and you will confuse left rotation with right rotation, and you will forget that rotation by length equals rotation by 0, and you will forget that there are exactly length distinct rotations possible.

---

## Drawing 1.6: The Intuition - Why Concatenate?

This is how a programmer actually discovers the trick. It comes from frustration.

**Step 1: The Frustration (Hitting the Wall)**
You want to check if `CDAB` is a rotation of `ABCD`.
You start matching character by character.

```
Array:      [ A , B , C , D ] 
Indices:      0   1   2   3
                          
Target:       C   D   A   B
              ↓   ↓   
Match?      [2] [3]   ... Uh oh!
```

- You matched `C` at index 2. OK.
- You matched `D` at index 3. OK.
- Next you need to match `A`.
- Your finger moves to index 4. **There is nothing there.**
- You hit the wall. The array ended.
- To find `A`, you have to **jump back** to index 0.

**Step 2: The Wish**
"I hate jumping back to 0. The modulo math is annoying. I wish the array didn't end."
"I wish `A` was just sitting right there after `D`."

**Step 3: The Fix (Unrolling)**
"What if I just pasted the array again?"

```
Array:      [ A , B , C , D , A , B , C , D ]
Indices:      0   1   2   3   4   5   6   7

Target:       C   D   A   B
              ↓   ↓   ↓   ↓
Match?      [2] [3] [4] [5]  -> LINEAR MATCH!
```

- Match `C` at index 2. OK.
- Match `D` at index 3. OK.
- Match `A`. Move to index 4. It's there! (Because we pasted `A`).
- Match `B`. Move to index 5. It's there! (Because we pasted `B`).

**Conclusion:**
We didn't use `s1+s1` because it's a magic formula.
We used it because we wanted to **walk off the edge of the first array** and land safely on the start of the second array without doing any math.
The concatenation simply **removes the edge**.

---

## The Discovery: How on Earth Do You Find the Trick?

You asked: *"There is no way a person can see this problem and come up with this trick solution."*
You are right. You **CANNOT** see it immediately. You must **BUILD** it by staring at the index patterns until they scream at you.

**Step 1: Write down the raw indices for every rotation of "ABCDE" (Length 5)**
Don't think about code. Just look at where the characters move.

**Position Indices:**
- `0 1 2 3 4` (Original indices of A B C D E)

**Rotation 0 (Shift 0):** `A B C D E`
Indices used: `0, 1, 2, 3, 4`

**Rotation 1 (Shift 1):** `B C D E A`
Indices used: `1, 2, 3, 4` ... and then `0` wrapped around.

**Rotation 2 (Shift 2):** `C D E A B`
Indices used: `2, 3, 4` ... and then `0, 1` wrapped around.

**Rotation 3 (Shift 3):** `D E A B C`
Indices used: `3, 4` ... and then `0, 1, 2` wrapped around.

**Rotation 4 (Shift 4):** `E A B C D`
Indices used: `4` ... and then `0, 1, 2, 3` wrapped around.

**Step 2: Stare at the broken sequences**
Look at the index sequences again. They are broken by the wraparound.
`1, 2, 3, 4` ... `0`
`2, 3, 4` ... `0, 1`
`3, 4` ... `0, 1, 2`

This wraparound (`... 0 ...`) is annoying. Math hates discontinuities.
How do we make the sequence `2, 3, 4, 0, 1` continuous?
We want it to look like `2, 3, 4, 5, 6`.

**Step 3: What if we extended the array?**
If we just copied the array again at the end, the indices `0, 1, 2...` would repeat as positions `5, 6, 7...`

Original: `A(0) B(1) C(2) D(3) E(4)`
Copy:     `A(5) B(6) C(7) D(8) E(9)`

Now look at **Rotation 2** again (`C D E A B`).
Old strict indices: `2, 3, 4, 0, 1` (Broken!)
New extended indices: `2, 3, 4, 5, 6` (Continuous!)
Wait... `5` is `A`. `6` is `B`.
So `2, 3, 4, 5, 6` is `C, D, E, A, B`.
It is exactly Rotation 2.

**Check Rotation 3 (`D E A B C`):**
Old strict indices: `3, 4, 0, 1, 2` (Broken!)
New extended indices: `3, 4, 5, 6, 7` (Continuous!)
`3(D), 4(E), 5(A), 6(B), 7(C)`.
It works.

**The Realization:**
By duplicating the string (`s1 + s1`), we convert the **modular wraparound** (which is hard) into **linear sliding windows** (which are easy).
We didn't "invent" the trick. We just **hated the wraparound** so much we fixed the index numbers by pasting the array again so we could count `4, 5, 6` instead of `4, 0, 1`.

---

## Drawing 1.5: The Paper Ring That Kills All Confusion

```
Step 1: You have a strip of paper with ABCD written on it

    ┌───┬───┬───┬───┐
    │ A │ B │ C │ D │
    └───┴───┴───┴───┘

Step 2: Glue the ends together (D's right edge to A's left edge) to make a ring

         ╭─── A ───╮
        D           B
         ╰─── C ───╯

Step 3: Read starting from A, going clockwise: A B C D
Step 4: Read starting from B, going clockwise: B C D A
Step 5: Read starting from C, going clockwise: C D A B
Step 6: Read starting from D, going clockwise: D A B C

That's it. Rotation = pick a different starting point on the ring, read all 4 letters clockwise.

4 possible starting points = 4 possible rotations:
  A B C D
  B C D A
  C D A B
  D A B C

There is no other combination possible.
```

**Why DCAB is NOT a rotation:**

```
On the ring:
  After D comes A (clockwise on the ring)
  After A comes B
  After B comes C
  After C comes D

DCAB claims:
  After D comes C  ← WRONG! On the ring, A comes after D, not C.

The ring FORCES the order. You cannot get DCAB from ABCD.
DCAB would require cutting the ring and rearranging, which is NOT rotation.
```

---

## Drawing 2: The Concatenation Trick That You Will Mess Up

```
s1 = "ABCD"
s1 + s1 = "ABCDABCD"

Position:   0   1   2   3   4   5   6   7
           +---+---+---+---+---+---+---+---+
           | A | B | C | D | A | B | C | D |
           +---+---+---+---+---+---+---+---+

All rotations hiding inside s1+s1:
  Position 0-3: A B C D  (rotation 0)
  Position 1-4: B C D A  (rotation 1)
  Position 2-5: C D A B  (rotation 2) <-- s2 = "CDAB" found here!
  Position 3-6: D A B C  (rotation 3)
```

**Why this drawing?** Because you will implement substring search thinking you are so clever, but you will forget to check if len(s1) == len(s2) first, and you will return true for s1="ABCD" and s2="CD" because "CD" is found in "ABCDABCD", but "CD" has length 2 and "ABCD" has length 4, so they cannot be cyclic permutations of each other, the lengths must match, and you will forget this and waste 20 minutes debugging.

---

## Drawing 3: The Length Check You Will Skip

```
s1 = "ABCD"    len = 4
s2 = "CD"      len = 2

s1 + s1 = "ABCDABCD"

Is "CD" found in "ABCDABCD"? 
Position 2-3: C D  --> YES, found!

But is "CD" a cyclic permutation of "ABCD"?
NO! Because 4 ≠ 2.

Your buggy code will return TRUE.
Correct code returns FALSE.
```

**Why this drawing?** Because your brain says "found means true" but the problem says "cyclic permutation" which requires same length and same character counts and same characters just rearranged in a shifted manner, and you will skip the length check because it seems obvious, but it is the first thing that will break your code.

---

## Drawing 4: The Empty String Edge Case You Will Ignore

```
Case 1: s1 = ""   s2 = ""
  len(s1) = 0, len(s2) = 0
  0 == 0 --> lengths match
  s1 + s1 = "" + "" = ""
  Is "" found in ""? --> YES (empty string is everywhere)
  Answer: TRUE (empty string is cyclic permutation of itself)

Case 2: s1 = ""   s2 = "A"
  len(s1) = 0, len(s2) = 1
  0 ≠ 1 --> lengths differ
  Answer: FALSE (skip substring search entirely)

Case 3: s1 = "A"  s2 = ""
  len(s1) = 1, len(s2) = 0
  1 ≠ 0 --> lengths differ
  Answer: FALSE
```

**Why this drawing?** Because you will either crash on empty strings or return wrong answers for them, and you will forget that empty concatenation is still empty, and you will forget that empty string found in empty string is a valid match, and you will add unnecessary null checks that break the logic.

---

## Drawing 5: The Off-By-One You Will Create

```
s1 = "AB"   s2 = "BA"

s1 + s1 = "ABAB"

Positions where s2 could start:
  Position 0: "AB" (s2 = "BA"? NO)
  Position 1: "BA" (s2 = "BA"? YES!)
  Position 2: "AB" (s2 = "BA"? NO)

But if you search with (len(s1+s1) - len(s2)) as limit:
  4 - 2 = 2
  You check positions 0, 1, 2
  
If you search with (len(s1+s1) - len(s2) - 1) as limit:
  4 - 2 - 1 = 1
  You check positions 0, 1
  Position 2 is missed... but wait, position 2 gives "AB" not "BA", so no problem here.

If you search with (len(s1+s1)) as limit:
  You check positions 0, 1, 2, 3
  Position 3 gives "B" + out of bounds crash!
```

**Why this drawing?** Because you will mess up the loop bounds in your substring search, you will either check too few positions and miss a valid match, or check too many positions and crash on array index out of bounds, and you will spend 30 minutes drawing arrays with different sizes to figure out the correct upper bound formula.

---

## Drawing 6: The False Positive Trap

```
s1 = "ABCD"   s2 = "ACBD"

s1 + s1 = "ABCDABCD"

Searching for "ACBD" in "ABCDABCD":
  Position 0: "ABCD" ≠ "ACBD" (A=A, B≠C) FAIL
  Position 1: "BCDA" ≠ "ACBD" (B≠A) FAIL
  Position 2: "CDAB" ≠ "ACBD" (C≠A) FAIL
  Position 3: "DABC" ≠ "ACBD" (D≠A) FAIL
  Position 4: "ABCD" ≠ "ACBD" (A=A, B≠C) FAIL

"ACBD" is NOT found in "ABCDABCD".
Answer: FALSE

Why is this correct?
  "ACBD" has same characters as "ABCD" (A, B, C, D all present)
  "ACBD" has same length as "ABCD" (both = 4)
  But "ACBD" is NOT a rotation of "ABCD" because no rotation produces A-C-B-D order.
```

**Why this drawing?** Because you will think "same characters + same length = cyclic permutation" but that is WRONG, the order matters, "ACBD" is an anagram of "ABCD" but not a cyclic rotation, and you will confuse anagram with rotation and waste time.

---

## Predicted Failures You Will Make

**Failure 1: Forgetting the length check entirely.** You will concatenate s1+s1, search for s2, find it, return true, and then fail on test case s1="ABCD" s2="CD" because you never checked if len(s1)==len(s2) and you will stare at your code for 10 minutes before noticing the missing check.

**Failure 2: Implementing your own substring search incorrectly.** You will write a nested loop to search for s2 in s1+s1, and you will get the loop indices wrong, either starting at index 1 instead of 0, or ending at the wrong position, or comparing characters at wrong offsets, and your code will fail for edge cases like s1="AA" s2="AA" or s1="A" s2="A".

**Failure 3: Using the wrong comparison logic.** You will compare s1[i+j] with s2[j] but forget to handle the case where i+j exceeds the length of s1+s1, or you will use == when you meant != in your break condition, or you will return false inside the loop when you meant to continue to the next starting position.

**Failure 4: Confusing contains with equals.** Rust's `contains()` method checks if s2 is a substring of s1+s1, which is exactly what you need, but you will think contains means "has the same characters" like an anagram check, and you will waste time implementing character counting instead of using the simple one-liner solution.

**Failure 5: Panicking on empty strings.** You will call methods that panic on empty slices, or you will return false for empty strings when the correct answer is true (empty is cyclic permutation of empty), or you will have off-by-one errors that only manifest when length is 0 or 1.

**Failure 6: Creating s1+s1 incorrectly in Rust.** You will try to use `+` operator on `&str` or `char[]` and get confused by ownership, or you will use `format!` incorrectly, or you will forget that `String` is different from `&str` and spend time fighting the borrow checker.

**Failure 7: Returning from inside the loop prematurely.** You will return `true` when you find a match (correct), but you will also return `false` when one position doesn't match (incorrect because you should check all positions first), and your function will return false for valid cyclic permutations after checking only the first position.

---

## Numerical Example 1: Basic Rotation

```
s1 = "ABCD"  s2 = "CDAB"

Step 1: Check lengths
  len(s1) = 4
  len(s2) = 4
  4 == 4? YES, proceed.

Step 2: Concatenate
  s1 + s1 = "A" + "B" + "C" + "D" + "A" + "B" + "C" + "D" = "ABCDABCD"

Step 3: Search for s2 in s1+s1
  concat = "ABCDABCD" (indices 0,1,2,3,4,5,6,7)
  target = "CDAB"     (indices 0,1,2,3)
  
  Start at i=0: concat[0..4] = "ABCD" vs "CDAB"
    concat[0]='A' vs target[0]='C' --> 'A' ≠ 'C' --> MISMATCH
    
  Start at i=1: concat[1..5] = "BCDA" vs "CDAB"
    concat[1]='B' vs target[0]='C' --> 'B' ≠ 'C' --> MISMATCH
    
  Start at i=2: concat[2..6] = "CDAB" vs "CDAB"
    concat[2]='C' vs target[0]='C' --> MATCH
    concat[3]='D' vs target[1]='D' --> MATCH
    concat[4]='A' vs target[2]='A' --> MATCH
    concat[5]='B' vs target[3]='B' --> MATCH
    All 4 characters matched! FOUND at position 2.

Step 4: Return TRUE
```

---

## Numerical Example 2: Not a Rotation

```
s1 = "ABCD"  s2 = "ACBD"

Step 1: Check lengths
  len(s1) = 4
  len(s2) = 4
  4 == 4? YES, proceed.

Step 2: Concatenate
  s1 + s1 = "ABCDABCD"

Step 3: Search for s2 in s1+s1
  i=0: "ABCD" vs "ACBD" --> A=A, B≠C --> MISMATCH
  i=1: "BCDA" vs "ACBD" --> B≠A --> MISMATCH
  i=2: "CDAB" vs "ACBD" --> C≠A --> MISMATCH
  i=3: "DABC" vs "ACBD" --> D≠A --> MISMATCH
  i=4: "ABCD" vs "ACBD" --> A=A, B≠C --> MISMATCH
  
  No more valid starting positions (i=5,6,7 would exceed bounds).
  NOT FOUND.

Step 4: Return FALSE
```

---

## Numerical Example 3: Single Character

```
s1 = "A"  s2 = "A"

Step 1: Check lengths
  len(s1) = 1
  len(s2) = 1
  1 == 1? YES.

Step 2: Concatenate
  s1 + s1 = "A" + "A" = "AA"

Step 3: Search for s2="A" in "AA"
  i=0: concat[0..1] = "A" vs "A" --> MATCH
  Found at position 0.

Step 4: Return TRUE
```

---

## Numerical Example 4: Different Lengths

```
s1 = "ABCD"  s2 = "ABC"

Step 1: Check lengths
  len(s1) = 4
  len(s2) = 3
  4 == 3? NO!

Step 2: Return FALSE immediately. Do NOT concatenate. Do NOT search.
```

---

## Numerical Example 5: Repeated Characters

```
s1 = "AABB"  s2 = "BBAA"

Step 1: Check lengths
  len(s1) = 4
  len(s2) = 4
  4 == 4? YES.

Step 2: Concatenate
  s1 + s1 = "AABBAABB"

Step 3: Search for "BBAA" in "AABBAABB"
  i=0: "AABB" vs "BBAA" --> A≠B --> MISMATCH
  i=1: "ABBA" vs "BBAA" --> A≠B --> MISMATCH
  i=2: "BBAA" vs "BBAA" --> B=B, B=B, A=A, A=A --> MATCH!
  Found at position 2.

Step 4: Return TRUE

Verification: Is "BBAA" a rotation of "AABB"?
  "AABB" rotated left by 2:
    Take first 2 chars: "AA"
    Remaining: "BB"
    Result: "BB" + "AA" = "BBAA" ✓
```

---

## Numerical Example 6: All Same Characters

```
s1 = "AAAA"  s2 = "AAAA"

Step 1: Check lengths
  len(s1) = 4
  len(s2) = 4
  4 == 4? YES.

Step 2: Concatenate
  s1 + s1 = "AAAAAAAA"

Step 3: Search for "AAAA" in "AAAAAAAA"
  i=0: "AAAA" vs "AAAA" --> MATCH immediately!
  Found at position 0.

Step 4: Return TRUE

Note: Every starting position would match, but we only need one.
```

---

## Numerical Example 7: Empty Strings

```
s1 = ""  s2 = ""

Step 1: Check lengths
  len(s1) = 0
  len(s2) = 0
  0 == 0? YES.

Step 2: Concatenate
  s1 + s1 = "" + "" = ""

Step 3: Search for "" in ""
  Empty string is considered to be "found" in any string, including empty.
  Found at position 0.

Step 4: Return TRUE

Rust behavior: "".contains("") returns true.
```

---

## Time and Space Complexity

**Time Complexity using contains():** The contains method in Rust uses an optimized string matching algorithm, typically O(n*m) in worst case for naive implementation or O(n+m) for more advanced algorithms where n=len(s1+s1)=2*len(s1) and m=len(s2), so overall O(n) where n is the length of the input strings since m equals n for valid inputs.

**Space Complexity:** O(n) for the concatenated string s1+s1 which has length 2*n where n is the length of s1, and no other significant space is used unless the contains() implementation uses additional buffers.

**Alternative approach KMP:** If you implement Knuth-Morris-Pratt for the substring search, you can achieve guaranteed O(n) time complexity, but this is overkill for this problem since the simple concatenation+contains approach is already efficient and easy to implement correctly.

---

## Function Prototype

```rust
fn is_cyclic_perm(s1: &str, s2: &str) -> bool
```

**Why &str?** Because we are reading immutable string slices, not character arrays, and Rust prefers &str for borrowed string data over converting to Vec<char> which would require additional allocation and indexing overhead, and the contains() method works directly on &str making the implementation trivial.

