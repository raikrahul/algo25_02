# 06_reverse_words_in_sentence.md

## PROBLEM STATEMENT

Write a function to reverse the words of given sentence. Modify the input array in-place.

**Function Prototype:** `fn reverse_words(s: &mut [char])`

**Example:**
- Input: `India is Best`
- Output: `Best is India`

---

## 1. THE ANNOYANCE (Why This Problem Is Hard)

```
BEFORE:  ['I','n','d','i','a',' ','i','s',' ','B','e','s','t']
AFTER:   ['B','e','s','t',' ','i','s',' ','I','n','d','i','a']
```

The annoyance is this: the word "India" (length 5) needs to swap with "Best" (length 4).
You cannot just copy "Best" to position 0 because it overwrites "Indi..." and leaves an 'a' behind.
You cannot shift everything because shifting is O(N^2).
We need O(N) time and O(1) space.

---

## 2. THE DERIVATION (Why We Reverse Everything)

### ATTEMPT 1: THE "TETRIS" APPROACH (Moving blocks)
Input: `[Ind, is, Best]`
Goal:  `[Best, is, Ind]`
Logic: "Just move 'Best' to the front!"
Problem: 'Best' is length 4. 'Ind' is length 5. The holes don't match.

### ATTEMPT 2: THE "MIRROR" OBSERVATION
We want to flip the ORDER of the units (words).
What is the cheapest way to flip order? **Reversal**.

Let's reverse the WHOLE THING.
Input: `[I,n,d, ,i,s, ,B,e,s,t]`
Reverse: `[t,s,e,B, ,s,i, ,d,n,I]`

**Did we get what we wanted?**
- Chunk 1: `tseB` ("Best" backwards)
- Chunk 2: `si`   ("is" backwards)
- Chunk 3: `dnI`  ("Ind" backwards)

**THE AHA MOMENT:**
The **position** of the words is CORRECT. "Best" is at the front. "Ind" is at the back.
The only problem is the **internal integrity** is destroyed.
**THE FIX:** Reverse each chunk again to fix the internal integrity.
`Reverse(tseB)` -> `Best`.

**MATHEMATICAL PROOF:**
1. Sequence $S = A + B$. Goal: $B + A$.
2. Reverse(S) = $B^r + A^r$.
3. ReverseComponents($B^r + A^r$) = Reverse($B^r$) + Reverse($A^r$) = $B + A$.

---

## 3. THE REVERSAL LOGIC (`left < right`)

### THE EVEN LENGTH TRACE (4 chars)
```
Start: ['a', 'b', 'c', 'd']   Indices: 0, 1, 2, 3
        L=0         R=3       (0 < 3 YES. Swap.)

Step 2: ['d', 'b', 'c', 'a']
             L=1   R=2        (1 < 2 YES. Swap.)

Step 3: ['d', 'c', 'b', 'a']
             R=1   L=2        (2 < 1 NO. STOP.)
```

### THE ODD LENGTH TRACE (5 chars)
```
Start: ['a', 'b', 'c', 'd', 'e']
        L=0               R=4 (0 < 4 YES. Swap.)

Step 2: ['e', 'b', 'c', 'd', 'a']
             L=1       R=3    (1 < 3 YES. Swap.)

Step 3: ['e', 'd', 'c', 'b', 'a']
                  L=2
                  R=2         (2 < 2 NO. STOP.)
```

**CONCLUSION:** The pivot element (index 2) is never touched. Logic `while left < right` handles both cases perfectly.

---

## 4. THE SCANNING LOOP TRAPS

### TRAP 1: THE DOUBLE SPACE (`cursor > index_after_space`)
Input: `['A', ' ', ' ', 'B']`
Space at index 1 is handled. `index_after_space` becomes 2.
Then we hit space at index 2.
`cursor=2`. `index_after_space=2`.
If we check `if s[cursor] == ' '`, we try `reverse(2, 2-1)`. USIZE UNDERFLOW.
**FIX:** Only reverse if `cursor > index_after_space`.

### TRAP 2: THE TRAILING SPACE (`index_after_space < len`)
Input: `['A', ' ']`
1. Hit space at index 1. Reverse "A". `index_after_space` becomes 2.
2. Loop Ends.
3. `index_after_space` is 2. `len` is 2.
4. Final check: `index_after_space < len` (2 < 2) is FALSE.
5. We DO NOT reverse. Correct. (There is no word after the space).

### TRAP 3: THE LAST WORD (`index_after_space < len`)
Input: `['A', ' ', 'B']`
1. Hit space at index 1. Reverse "A". `index_after_space` becomes 2.
2. Loop hits 'B'. Not space.
3. Loop Ends.
4. `index_after_space` is 2. `len` is 3. 'B' is sitting there backwards.
5. Final check: `index_after_space < len` (2 < 3) is TRUE.
6. We REVERSE "B". Correct.

### TRUTH TABLE: `index_after_space < len`
| Case | Input | End `index_after_space` | End `len` | Condition | Action |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Trailing Space | "Hi " | 3 | 3 | FALSE | Skip (Correct) |
| Normal Word | "Hi" | 0 | 2 | TRUE | Reverse (Correct) |
| Last Word | "A B" | 2 | 3 | TRUE | Reverse (Correct) |

---

## 5. THE GAP SKIPPING LOGIC

### TRACE FOR "a     b" (Gap of 5 spaces)

1. First space at index 1. `cursor`=1. `index_after_space`=0.
   Check: 1 > 0 is TRUE.
   Action: reverse(s, 0, 0). 'a' stays 'a'.
   Update: `index_after_space` becomes 1 + 1 = 2.

2. Second space at index 2. `cursor`=2.
   Check: 2 > 2 is FALSE.
   Action: SKIP! (This is the gap skipping magic).
   Update: `index_after_space` becomes 2 + 1 = 3.

3. Third space at index 3. `cursor`=3.
   Check: 3 > 3 is FALSE.
   Action: SKIP!
   Update: `index_after_space` becomes 3 + 1 = 4.

**CONCLUSION:** The `index_after_space` keeps chasing `cursor` + 1.
As long as they are "touching" (cursor == index_after_space), implies 0 length word.
So we do nothing. We drive over the gap safely.

---

## 6. FULL HAND TRACE: "        ab      cd      "

### INPUT SETUP
```
Input: "        ab      cd      "
        ^^^^^^^^  ^^^^^^  ^^^^^^
        8 spaces  6 space 6 space
                ^^      ^^
               'ab'   'cd'

Indices: 0-7 spaces, 8='a', 9='b', 10-15 spaces, 16='c', 17='d', 18-23 spaces
Length = 24
```

### PHASE 1: FULL REVERSE
```
BEFORE: "        ab      cd      "
AFTER:  "      dc      ba        "
```
(Trailing 6 spaces → front, Leading 8 spaces → back, words swapped positions but reversed internally)

### PHASE 2: WORD SCANNER

| cursor | char | cursor > idx_after_space? | Action | New idx_after_space |
|:------:|:----:|:-------------------------:|:-------|:--------------------|
| 0-5 | ' ' | FALSE each time | SKIP! | 1→2→3→4→5→6 |
| 6 | 'd' | - | Skip (not space) | 6 |
| 7 | 'c' | - | Skip (not space) | 6 |
| 8 | ' ' | **8 > 6 = TRUE** | **reverse(6, 7)** "dc"→"cd" | 9 |
| 9-13 | ' ' | FALSE each time | SKIP! | 10→11→12→13→14 |
| 14 | 'b' | - | Skip (not space) | 14 |
| 15 | 'a' | - | Skip (not space) | 14 |
| 16 | ' ' | **16 > 14 = TRUE** | **reverse(14, 15)** "ba"→"ab" | 17 |
| 17-23 | ' ' | FALSE each time | SKIP! | 18→19→20→21→22→23→24 |

### FINAL CHECK
`24 < 24`? FALSE. No straggler word.

### RESULT
```
"      cd      ab        "
```
Words "ab" and "cd" swapped positions. **CORRECT!**

---

## 7. MEMORY STATE TRACES

### TRACE 1: Leading Space Barrier
Input: `[' ', 'A']`
- i=0 is ' '. `0 > 0` FALSE. SKIP (prevents `0-1` underflow).
- Final reverse(1, 1). Correct.

### TRACE 2: Double Space Tire Tracks
Input: `['A', ' ', ' ', 'B']`
- i=1 (' '): `1 > 0` TRUE. reverse(0,0).
- i=2 (' '): `2 > 2` FALSE. SKIP (empty word).
- Final reverse(3, 3). Correct.

### TRACE 3: Trailing Space Abyss
Input: `['A', ' ']`
- i=1 (' '): `1 > 0` TRUE. reverse(0,0). `index_after_space`=2.
- Final check: `2 < 2` FALSE. STOP. No phantom word.

### TRACE 4: Normal Last Word
Input: `['A', 'B']`
- No spaces found. `index_after_space` stays 0.
- Final check: `0 < 2` TRUE. reverse(0, 1). Correct.
