PROBLEM: Given array with contiguous 0s at start, followed by non-zero integers, return count of zeroes.
https://notebooklm.google.com/notebook/10e49bfd-dbc2-4174-9880-beb3755e976d
EXAMPLE 1:
```
index:   0   1   2   3   4   5   6   7   8   9  10  11
       +---+---+---+---+---+---+---+---+---+---+---+---+
value: | 0 | 0 | 0 | 0 | 0 | 3 | 2 | 8 |11 |10 |15 |22 |
       +---+---+---+---+---+---+---+---+---+---+---+---+
answer = 5
```

BRUTE FORCE — scan left to right, count 0s until you hit non-zero — for the above array you check index 0 (is 0? yes, count=1), check index 1 (is 0? yes, count=2), check index 2 (is 0? yes, count=3), check index 3 (is 0? yes, count=4), check index 4 (is 0? yes, count=5), check index 5 (is 3? not zero, STOP) — you visited 6 cells — answer is 5.

EXAMPLE 2:
```
index:   0   1   2   3   4   5   6   7
       +---+---+---+---+---+---+---+---+
value: | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 9 |
       +---+---+---+---+---+---+---+---+
brute force visits: 0,1,2,3,4,5,6,7 = 8 cells
answer = 7
```

EXAMPLE 3:
```
index:   0   1   2   3   4   5
       +---+---+---+---+---+---+
value: | 0 | 5 | 8 | 3 | 1 | 2 |
       +---+---+---+---+---+---+
brute force visits: 0,1 = 2 cells
answer = 1
```

EXAMPLE 4:
```
index:   0   1   2   3   4
       +---+---+---+---+---+
value: | 7 | 5 | 8 | 3 | 1 |
       +---+---+---+---+---+
brute force visits: 0 = 1 cell
answer = 0
```

EXAMPLE 5:
```
index:   0   1   2   3
       +---+---+---+---+
value: | 0 | 0 | 0 | 0 |
       +---+---+---+---+
brute force visits: 0,1,2,3 = 4 cells, never find non-zero, answer = n = 4
answer = 4
```

EXAMPLE 6 (LARGE):
```
arr = [0, 0, 0, ...(999997 zeroes total)..., 7, 8, 9]
n = 1000000
brute force visits: 999998 cells
answer = 999997
```

THE ANNOYANCE — brute force is O(n) — if answer is near the end, you scan almost the whole array.

---

NOW LOOK AT THE DATA DIFFERENTLY — color each cell:
```
index:   0   1   2   3   4   5   6   7   8   9  10  11
       +---+---+---+---+---+---+---+---+---+---+---+---+
value: | 0 | 0 | 0 | 0 | 0 | 3 | 2 | 8 |11 |10 |15 |22 |
       +---+---+---+---+---+---+---+---+---+---+---+---+
color: | B | B | B | B | B | W | W | W | W | W | W | W |
       +---+---+---+---+---+---+---+---+---+---+---+---+
       [=====BLACK=====][==========WHITE==============]
B = zero, W = non-zero
```

QUESTION: What pattern do you see? Fill in:
- All B cells are _______________  (contiguous? scattered?)
- All W cells are _______________  (contiguous? scattered?)
- B cells come _______________  W cells (before? after? mixed?)

OBSERVATION TABLE (fill in):
```
| index | value | is zero? | color |
|-------|-------|----------|-------|
|   0   |   0   |   YES    |   B   |
|   1   |   0   |   YES    |   B   |
|   2   |   0   |   YES    |   B   |
|   3   |   0   |   YES    |   B   |
|   4   |   0   |   YES    |   B   |
|   5   |   3   |   NO     |   W   |
|   6   |   2   |   NO     |   W   |
|   ....|  .... |   ...    |  ...  |
```

THE PATTERN: Colors go B B B B B W W W W W W W — never B W B or W B W — the boundary between B and W is SHARP — there is exactly ONE boundary.

---

THINK ABOUT IT — if you could look at ANY cell and determine which SIDE of the boundary it's on, what famous algorithm exploits this property?

HINT 1 — imagine a sorted array [1,2,3,4,5,6,7,8,9] and you want to find 5 — you don't scan left to right — you jump to the _______ and ask "is 5 here? no, is 5 to the left or right?"

HINT 2 — in our problem, if you look at cell index 6 and see value 2 (non-zero), what does that tell you about the boundary? The boundary is at index 6 or __before_____ (before? after?) index 6.

HINT 3 — if you look at cell index 3 and see value 0 (zero), what does that tell you about the boundary? The boundary is iafter _______ (before? after?) index 3.

---

FILL IN THE BLANKS — given array [0,0,0,0,0,3,2,8,11,10,15,22]:
```
If I look at index 6, value = 2, which is non-zero.
Boundary is at or before index 6.
I should search the __left _____ half next.

If I look at index 3, value = 0, which is zero.
Boundary is after index 3.
I should search the ___right ____ half next.
```

WHAT ALGORITHM DOES THIS? _________binary search______

---

EDGE CASE ANALYSIS — what should happen?

CASE 1: First element is non-zero
```
index:   0   1   2   3   4
       +---+---+---+---+---+
value: | 7 | 5 | 8 | 3 | 1 |
       +---+---+---+---+---+
There are ___NIL zeroes.
The boundary is at index _0__.
```

CASE 2: All elements are zero
```
index:   0   1   2   3
       +---+---+---+---+
value: | 0 | 0 | 0 | 0 |
       +---+---+---+---+
There are _ALL__ zeroes.
The boundary is at index 3___ (what if there's no non-zero element? boundary = n?)
```

CASE 3: Empty array
```
index: (none)
value: (none)
There are ___0 zeroes.
```

CASE 4: Single zero
```
index:   0
       +---+
value: | 0 |
       +---+
There are 1___ zeroes.
```

CASE 5: Single non-zero
```
index:   0
       +---+
value: | 5 |
       +---+
There are __0_ zeroes.
```

---

PREDICTED FAILURES YOU WILL MAKE:

FAILURE 1 — You will initialize high = n-1, but when all elements are zero, you'll return n-1 instead of n — trace through [0,0,0,0] yourself.

FAILURE 2 — You will confuse which direction to search — when you see 0, does boundary come BEFORE or AFTER? Think carefully. It is after 

FAILURE 3 — You will use low = mid instead of low = mid + 1 — this causes infinite loop when low and high are adjacent — trace through it. THis is where I have problem 

FAILURE 4 — You will use high = mid - 1 instead of high = mid — this skips the boundary when boundary IS at mid — trace through [0,0,3,5,7].- THis is where I have problem 

FAILURE 5 — You will return high instead of low — trace through and see which one holds the answer at the end. Low 

---

YOUR TRACE WORKSHEET — do these by hand before coding:

TRACE 1: arr = [0,0,0,5,7,2,9], n=7, expected answer=3
```
Step 1: low=__0_ high=___6 mid=___3 arr[mid]=__5_ decision:_Move left or this could be __
Step 2: low=__0_ high=_3__ mid=___1 arr[mid]=_0__ decision:_Move right __
Step 3: low=___1 high=__3_ mid=___2 arr[mid]=_0__ decision:_Move right__
...continue until low >= high...
FINAL: answer = ___
```

TRACE 2: arr = [0,0,0,0], n=4, expected answer=4
```
Step 1: low=_0__ high=_4__ mid=_2__ arr[mid]=__0_ decision:_0 == 0 true __
Step 2: low=_3__ high=__4_ mid=_3__ arr[mid]=_0__ decision:___0 == o true 
Now low becomes 4 and we stop
...continue until low >= high...
FINAL: answer = ___
```

TRACE 3: arr = [7,5,8,3,1], n=5, expected answer=0
```
Step 1: low=_0__ high=___5 mid=__2_ arr[mid]=__8_ decision:_8 != 0 hence we reduce mid__
Step 2: low=__0_ high=__1_ mid=_0 __ arr[mid]=__7_ decision:__ 7!= 0_ hence we reduce mid
now low is same as high we return low 
...continue until low >= high...
FINAL: answer = 0___
```

---

═══════════════════════════════════════════════════════════════════════════════
WHY THE ASYMMETRY? WHY low=mid+1 BUT high=mid (NOT mid-1)?
YOUR FILTHY BRAIN WANTS TO MEMORIZE. I WILL MAKE YOU CALCULATE INSTEAD.
═══════════════════════════════════════════════════════════════════════════════

THE ANNOYANCE: Your brain says "if low gets +1, high should get -1 for symmetry!"
THIS IS WRONG. Let me make you SEE why through calculations.

---

SETUP: arr = [0, 0, 3, 5, 7], n = 5, answer = 2 (first non-zero at index 2)
```
index:   0   1   2   3   4
       +---+---+---+---+---+
value: | 0 | 0 | 3 | 5 | 7 |
       +---+---+---+---+---+
       [ZERO][=NON-ZERO====]
       answer = 2
```

---

CALCULATION 1: What is the starting range?
```
low = __0_    high = __5_    range = [low, high) = [_0__, __5_)
```
WHY: The range [low, high) represents "the answer is somewhere in here"
FORMULA: low = 0 (start), high = n (end, exclusive)
FILL IN: low = 0, high = 5, range = [0, 5)

---

CALCULATION 2: What is mid when low=0, high=5?
```
mid = low + (high - low) / 2
mid = 0 + (5 - 0) / 2
mid = 0 + 5/2
mid = 0 + _2__
mid = __2_
```
WHY: mid is where we probe the array to decide which half to search
FORMULA: mid = low + (high - low) / 2 (integer division)
FILL IN: mid = 0 + 2 = 2

---

CALCULATION 3: What is arr[mid] when mid=2?
```
arr[2] = 3___
Is arr[2] == 0? __no_:  (YES or NO)
```
WHY: This tells us which side of the boundary mid is on
FILL IN: arr[2] = 3, which is NOT zero

---

CALCULATION 4: If arr[mid] != 0, what does it mean?
```
arr[2] = 3 (non-zero)
The boundary is at index 2 or _______BEFORE_____ (BEFORE or AFTER?) index 2
```
WHY: Non-zero means we're in the NON-ZERO zone, so boundary is at or before us
FILL IN: The boundary is at index 2 or BEFORE index 2

---

CALCULATION 5: If boundary is at index 2 or before, what is the NEW range?
```
Old range: [0, 5)
Boundary is at or before 2
New range must include index 2? __Yes_ (YES or NO)
New range: [0, 2___) 
```
WHY: We found mid=2 is non-zero, but it COULD BE the boundary (first non-zero)
FILL IN: YES, must include 2. New range: [0, 3) — WAIT, that's wrong! Should be [0, 2]? No...

HERE IS THE MEAT:
```
If we do high = mid - 1 = 2 - 1 = 1, new range = [0, 1)
   → We search indices {0} only
   → We SKIP index 2 forever
   → But index 2 IS THE ANSWER!
   → WRONG!

If we do high = mid = 2, new range = [0, 2)
   → We search indices {0, 1}
   → Wait, we also skip index 2?
   → NO! When low=2 and high=2, loop STOPS, we return low=2
   → CORRECT!
```

---

CALCULATION 6: Let's trace with high = mid (CORRECT):
```
Step 1: low=0, high=5, mid=2, arr[2]=3≠0, high=mid=2
Step 2: low=0, high=2, mid=0+(2-0)/2=1, arr[1]=0, low=mid+1=2
Step 3: low=2, high=2, 2<2? NO, STOP
Answer = low = 2 ✓
```
FILL IN THE BLANKS TO VERIFY.

---

CALCULATION 7: Let's trace with high = mid - 1 (WRONG):
```
Step 1: low=0, high=5, mid=2, arr[2]=3≠0, high=mid-1=1
Step 2: low=0, high=1, mid=0+(1-0)/2=0, arr[0]=0, low=mid+1=1
Step 3: low=1, high=1, 1<1? NO, STOP
Answer = low = 1 ✗ WRONG! Should be 2!
```
FILL IN THE BLANKS TO VERIFY.

---

NOW THE OTHER SIDE: Why low = mid + 1 (not low = mid)?

CALCULATION 8: arr = [0, 0, 3, 5, 7], step where mid=1
```
low=0, high=2, mid=0+(2-0)/2=1, arr[1]=0
arr[mid] = 0, which is zero
The boundary is ____after________ (BEFORE or AFTER?) index 1
```
WHY: Zero means we're in the ZERO zone, so boundary is AFTER us
FILL IN: The boundary is AFTER index 1

---

CALCULATION 9: If boundary is AFTER index 1, should we include index 1 in the new range?
```
Index 1 contains 0 (zero). 
Is index 1 potentially the answer (first non-zero)? ___no (YES or NO)
Should low become mid or mid+1? _yes__
```
WHY: Index 1 is a zero, so it's NOT the answer. We can EXCLUDE it.
FILL IN: NO, index 1 is not the answer. low = mid + 1

---

CALCULATION 10: What if we used low = mid (WRONG)?
```
low=0, high=2, mid=1, arr[1]=0, low=mid=1 (WRONG)
Next: low=1, high=2, mid=1+(2-1)/2=1, arr[1]=0, low=mid=1
Next: low=1, high=2, mid=1 again...
INFINITE LOOP!
```
TRACE IT YOURSELF TO SEE THE LOOP.

---

THE ASYMMETRY EXPLAINED:
```
┌────────────────────────────────────────────────────────────────────────────┐
│ CASE: arr[mid] == 0 (we're in ZERO zone)                                   │
│                                                                            │
│ index:   0   1   2   3   4                                                 │
│        +---+---+---+---+---+                                               │
│ value: | 0 | 0 | 3 | 5 | 7 |                                               │
│        +---+---+---+---+---+                                               │
│              ↑                                                             │
│            mid=1                                                           │
│            arr[1]=0                                                        │
│                                                                            │
│ We KNOW index 1 is a zero. Index 1 is NOT the answer.                     │
│ The answer is STRICTLY AFTER index 1.                                      │
│ So we can EXCLUDE index 1 from future search.                              │
│ low = mid + 1 = 2 (EXCLUDE mid, it's definitely not the answer)           │
└────────────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────────────┐
│ CASE: arr[mid] != 0 (we're in NON-ZERO zone)                               │
│                                                                            │
│ index:   0   1   2   3   4                                                 │
│        +---+---+---+---+---+                                               │
│ value: | 0 | 0 | 3 | 5 | 7 |                                               │
│        +---+---+---+---+---+                                               │
│                  ↑                                                         │
│                mid=2                                                       │
│                arr[2]=3                                                    │
│                                                                            │
│ We KNOW index 2 is non-zero. But is it THE FIRST non-zero?                │
│ We DON'T KNOW! Maybe index 1 is also non-zero. Maybe not.                 │
│ So we CANNOT exclude index 2 from future search.                           │
│ high = mid = 2 (INCLUDE mid, it might be the answer)                      │
└────────────────────────────────────────────────────────────────────────────┘
```

SUMMARY:
```
┌───────────────────┬────────────────────┬────────────────────────────────────┐
│ arr[mid] value    │ What we KNOW       │ What to do                         │
├───────────────────┼────────────────────┼────────────────────────────────────┤
│ arr[mid] == 0     │ mid is NOT answer  │ EXCLUDE mid: low = mid + 1         │
│ arr[mid] != 0     │ mid MIGHT BE answer│ INCLUDE mid: high = mid            │
└───────────────────┴────────────────────┴────────────────────────────────────┘
```

QUIZ — fill in:
```
Q1: Why low = mid + 1?
A: Because when arr[mid] == 0, we KNOW mid is __________, so we can __________ it.

Q2: Why high = mid (not mid - 1)?
A: Because when arr[mid] != 0, mid __________ be the answer, so we must __________ it.

Q3: What happens if you use high = mid - 1?
A: You might __________ the answer if it happens to be at index mid.

Q4: What happens if you use low = mid?
A: __________ loop when low and high are adjacent.
```

---

═══════════════════════════════════════════════════════════════════════════════
BRUTAL ROAST OF YOUR ERRORS — READ CAREFULLY — EVERY MISTAKE IS A LOST MARK
═══════════════════════════════════════════════════════════════════════════════

SLOPPINESS #1 — TRACE 1: YOU USED high=6 INSTEAD OF high=7
```
arr = [0,0,0,5,7,2,9]
n = 7 (SEVEN elements, indices 0,1,2,3,4,5,6)

YOU WROTE: high=6
CORRECT:   high=n=7

WHY THIS MATTERS:
┌───────────────────────────────────────────────────────┐
│ index:   0   1   2   3   4   5   6   ║  7 (imaginary) │
│        +---+---+---+---+---+---+---+ ║                │
│ value: | 0 | 0 | 0 | 5 | 7 | 2 | 9 | ║  (out of bounds)│
│        +---+---+---+---+---+---+---+ ║                │
│          ↑                       ↑   ↑                │
│        low=0                 high=6 high=7            │
│                              WRONG   CORRECT          │
└───────────────────────────────────────────────────────┘

If you use high=n-1=6, and ALL elements were zero [0,0,0,0,0,0,0]:
- Your answer would cap at 6 (WRONG)
- Correct answer is 7

high=n allows low to REACH n when all elements are zero.
high=n-1 CAPS answer at n-1.

YOU DID NOT READ THE PROBLEM. YOU DID NOT TRACE THE EDGE CASE.
```

SLOPPINESS #2 — TRACE 1 STEP 3: YOU WROTE low=1 INSTEAD OF low=2
```
YOUR WORK:
Step 2: low=0 high=3 mid=1 arr[1]=0 → low=mid+1=1+1=2
Step 3: low=1 ← WHERE DID 1 COME FROM?

THE CALCULATION:
low = mid + 1 = 1 + 1 = 2

YOUR BRAIN:
"mid is 1, so low is 1"  ← WRONG
"mid is 1, so low is mid + 1 = 2" ← CORRECT

YOU COPIED mid TO low INSTEAD OF COMPUTING mid + 1.
YOU SKIPPED THE "+1" BECAUSE YOUR EYES GLAZED OVER.
```

SLOPPINESS #3 — TRACE 3: YOU SKIPPED A STEP
```
arr = [7,5,8,3,1], n=5

YOUR WORK:
Step 1: low=0 high=5 mid=2 arr[2]=8 → high=mid=2
Step 2: low=0 high=1 ← WHERE DID 1 COME FROM?

CORRECT:
Step 1: low=0 high=5 mid=2 arr[2]=8 → high=2
Step 2: low=0 high=2 mid=1 arr[1]=5 → high=1  ← YOU SKIPPED THIS
Step 3: low=0 high=1 mid=0 arr[0]=7 → high=0
Step 4: STOP

YOU JUMPED FROM high=2 TO high=1 WITHOUT SHOWING THE WORK.
YOU ASSUMED THE ANSWER INSTEAD OF COMPUTING IT.
YOU DID NOT TRACE. YOU GUESSED.
```

SLOPPINESS #4 — CASE 2: YOU SAID BOUNDARY IS AT INDEX 3
```
arr = [0,0,0,0], n=4

YOU WROTE: "The boundary is at index 3"
CORRECT:   "The boundary is at index 4"

WHY:
┌─────────────────────────────────────────┐
│ index:   0   1   2   3   ║  4 (boundary)│
│        +---+---+---+---+ ║              │
│ value: | 0 | 0 | 0 | 0 | ║              │
│        +---+---+---+---+ ║              │
│                          ↑              │
│                     BOUNDARY = 4        │
│                     (first non-zero)    │
│                     (doesn't exist)     │
│                     (so boundary = n)   │
└─────────────────────────────────────────┘

arr[3] = 0 ← THIS IS STILL A ZERO
The boundary is WHERE THE FIRST NON-ZERO WOULD BE.
Since there is no non-zero, boundary = n = 4.

YOU CONFUSED "last zero index" WITH "boundary index".
```

SLOPPINESS #5 — RUST CODE TYPOS
```
YOUR CODE:
Line 248: let n :uszie = arr.len()l    ← "uszie" not "usize", extra "l"
Line 249: let mut low :uszie = 0;      ← "uszie" not "usize"
Line 251: while !                       ← incomplete, gibberish

DIAGNOSIS:
- You typed "uszie" TWICE. You didn't even look at what you typed.
- You added a random "l" after arr.len()
- You wrote "while !" and stopped mid-thought.

THIS IS NOT THINKING. THIS IS RANDOM KEY-MASHING.
```

═══════════════════════════════════════════════════════════════════════════════
PATTERN OF YOUR FAILURES
═══════════════════════════════════════════════════════════════════════════════

1. You SKIP the "+1" in "low = mid + 1"
2. You SKIP intermediate steps and jump to answers
3. You CONFUSE n-1 with n
4. You type without looking at the screen
5. You start writing code before finishing traces

═══════════════════════════════════════════════════════════════════════════════
WHAT YOU MUST DO BEFORE TOUCHING THE KEYBOARD AGAIN
═══════════════════════════════════════════════════════════════════════════════

1. COMPLETE TRACE 2 FULLY — write step 3 and FINAL answer
2. COMPLETE TRACE 3 FULLY — write ALL 4 steps
3. READ what you type BEFORE pressing Enter
4. DELETE your broken Rust code and start fresh

ERROR LOG — fill in as you code:
```
ATTEMPT 1:
  What went wrong: _______________________
  Root cause: ___________________________
  Fix applied: __________________________

ATTEMPT 2:
  What went wrong: _______________________
  Root cause: ___________________________
  Fix applied: __________________________
```

═══════════════════════════════════════════════════════════════════════════════
COMPREHENSIVE ROAST OF ALL YOUR ERRORS — WITH DIAGRAMS
═══════════════════════════════════════════════════════════════════════════════

WHY THIS ROAST EXISTS: You made these errors. Each error is a mark lost in an exam. Each error shows you didn't trace carefully. Each error shows you GUESSED instead of CALCULATED.

---

ERROR #1: TRACE 1 — YOU USED high=6 INSTEAD OF high=7

WHY THIS DIAGRAM: To show that high=n=7, not high=n-1=6
```
arr = [0, 0, 0, 5, 7, 2, 9]
n = 7 (count the elements: 1,2,3,4,5,6,7 = SEVEN elements)

index:   0   1   2   3   4   5   6    ║   7
       +---+---+---+---+---+---+---+  ║
value: | 0 | 0 | 0 | 5 | 7 | 2 | 9 |  ║ (out of bounds)
       +---+---+---+---+---+---+---+  ║
         ↑                       ↑    ↑
        low=0               high=6  high=7
                             ^^^^^   ^^^^^
                             WRONG   CORRECT

YOU WROTE: high = 6
CORRECT:   high = n = 7
```

WHY THIS MATTERS:
```
If arr = [0,0,0,0,0,0,0] (all zeroes), n = 7:
  With high = 6: max answer = 6 (WRONG, should be 7)
  With high = 7: answer can reach 7 (CORRECT)
```

---

ERROR #2: TRACE 1 STEP 3 — YOU WROTE low=1 INSTEAD OF low=2

WHY THIS DIAGRAM: To show the calculation you SKIPPED
```
YOUR STEP 2: low=0, high=3, mid=1, arr[1]=0 → decision: move right
             low = mid + 1 = 1 + 1 = ???
                                     ↓
                            YOU WROTE: 1
                            CORRECT:   2

                   ┌─────────────────┐
                   │ mid = 1         │
                   │ mid + 1 = 1 + 1 │
                   │        = 2      │ ← YOU SKIPPED THIS
                   └─────────────────┘
                   
YOUR STEP 3: low = 1 ← WHERE DID 1 COME FROM?
CORRECT:     low = 2

YOU COPIED mid TO low INSTEAD OF COMPUTING mid + 1
```

---

ERROR #3: TRACE 3 — YOU SKIPPED A STEP (high=2 → high=1 WITHOUT WORK)

WHY THIS DIAGRAM: To show the step you skipped
```
arr = [7, 5, 8, 3, 1], n = 5

YOUR TRACE:
┌──────────────────────────────────────────────────────────┐
│ Step 1: low=0, high=5, mid=2, arr[2]=8≠0, high=mid=2    │
│ Step 2: low=0, high=1 ← WHERE DID 1 COME FROM???        │
│                  ↑                                       │
│                  You jumped from 2 to 1 without work!   │
└──────────────────────────────────────────────────────────┘

CORRECT TRACE:
┌──────────────────────────────────────────────────────────┐
│ Step 1: low=0, high=5, mid=2, arr[2]=8≠0, high=2        │
│ Step 2: low=0, high=2, mid=1, arr[1]=5≠0, high=1 ← HERE │
│ Step 3: low=0, high=1, mid=0, arr[0]=7≠0, high=0        │
│ Step 4: low=0, high=0, STOP, return 0                   │
└──────────────────────────────────────────────────────────┘

YOU SKIPPED STEP 2 AND JUMPED TO STEP 3's VALUES
```

---

ERROR #4: CASE 2 — YOU SAID BOUNDARY IS AT INDEX 3 (SHOULD BE 4)

WHY THIS DIAGRAM: To show the difference between "last zero" and "boundary"
```
arr = [0, 0, 0, 0], n = 4

index:   0   1   2   3   ║   4
       +---+---+---+---+ ║
value: | 0 | 0 | 0 | 0 | ║ (imaginary position)
       +---+---+---+---+ ║
                     ↑   ↑
           last zero │   └── BOUNDARY (first non-zero position)
                 at 3│       = index 4 = n
                     │
         YOU SAID: boundary at 3 ← WRONG
         CORRECT:  boundary at 4

arr[3] = 0 ← THIS IS STILL A ZERO, NOT THE BOUNDARY
The boundary is WHERE the first non-zero WOULD BE.
Since all are zero, boundary = n = 4.
```

---

ERROR #5: RUST CODE — TYPOS (uszie, extra l, incomplete while)

WHY THIS DIAGRAM: To show you typed WITHOUT LOOKING
```
YOUR CODE:
┌────────────────────────────────────────────────────────────────┐
│ Line 248: let n :uszie = arr.len()l                            │
│                  ^^^^^            ^                            │
│                  TYPO             TYPO                         │
│                  "uszie"          extra "l"                    │
│                  should be        should be                    │
│                  "usize"          nothing                      │
│                                                                │
│ Line 249: let mut low :uszie = 0;                              │
│                        ^^^^^                                   │
│                        TYPO AGAIN                              │
│                        You made the SAME typo TWICE            │
│                                                                │
│ Line 251: while !                                              │
│                 ^^^^^^^^                                       │
│                 INCOMPLETE GIBBERISH                           │
│                 You stopped mid-thought                        │
└────────────────────────────────────────────────────────────────┘

DIAGNOSIS:
- You typed "uszie" TWICE. Did you even LOOK at the screen?
- You added a random "l" after arr.len(). What is "arr.len()l"?
- You wrote "while !" and gave up. Where is the condition?

THIS IS NOT CODING. THIS IS KEYBOARD MASHING.
```

---

ERROR #6: CALCULATION 9 — YOU ANSWERED "YES" INSTEAD OF "mid+1"

WHY THIS DIAGRAM: To show you didn't read the question
```
QUESTION: Should low become mid or mid+1? ___

YOU WROTE: _yes__

CORRECT:   mid+1 (or the exact value like "2")

THE QUESTION ASKS: "mid or mid+1"
YOUR ANSWER:       "yes"

"yes" is not one of the options. The options are "mid" or "mid+1".
You answered a DIFFERENT question.
```

---

PATTERN OF YOUR ERRORS:
```
┌─────────────────────────────────────────────────────────────────────────────┐
│ ERROR TYPE              │ COUNT │ ROOT CAUSE                               │
├─────────────────────────┼───────┼──────────────────────────────────────────┤
│ Off-by-one (n vs n-1)   │   2   │ Not understanding why high = n           │
│ Skipping +1 in mid+1    │   1   │ Eyes glazed over the "+1"                │
│ Skipping steps in trace │   1   │ Guessing instead of calculating          │
│ Typos in code           │   3   │ Typing without looking at screen         │
│ Wrong answer format     │   1   │ Not reading the question                 │
└─────────────────────────┴───────┴──────────────────────────────────────────┘
                                                      TOTAL ERRORS: 8
```

WHAT THESE ERRORS REVEAL:
```
1. You DO NOT TRACE. You GUESS.
2. You DO NOT READ. You SKIM.
3. You DO NOT CALCULATE. You ASSUME.
4. You DO NOT CHECK. You SUBMIT.
5. You TYPE BEFORE THINKING.
6. You SKIP THE "+1" BECAUSE IT LOOKS SMALL.
7. You CONFUSE "last index" WITH "count" (n-1 vs n).
```

---

THE FIX:
```
BEFORE YOU WRITE ANYTHING:
  1. READ the question TWICE.
  2. IDENTIFY: What is n? What is n-1? Which one do I need?
  3. TRACE: low=?, high=?, mid=?, arr[mid]=?, decision=?
  4. CALCULATE: mid+1 means ADD ONE. Actually do 1+1=2.
  5. CHECK: Did I skip a step? Did I copy the wrong value?
  6. TYPE SLOWLY. LOOK AT THE SCREEN.
```

═══════════════════════════════════════════════════════════════════════════════
END OF ROAST
═══════════════════════════════════════════════════════════════════════════════
