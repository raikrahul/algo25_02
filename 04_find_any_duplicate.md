# Problem 3: Find Any Duplicate Number

## The Constraint That Breaks Your Brain

Given an array of N integers where each element is between 1 and N-1 inclusive, you must find any duplicated integer and you may destroy the array in the process which means you can mutate it, overwrite values, use it as scratch space, so the question becomes what information can you extract from the constraint that every value is between 1 and N-1 and here is the brutal truth: if you have N slots and only N-1 distinct possible values then by pigeonhole principle at least one value must repeat, but finding WHICH value repeats is where you will fail, not because the problem is hard but because you will skip the connection between VALUE and INDEX.

## Drawing 1: The Raw Input Array

```
Array a of length N=7:
┌───────────────────────────────────────────────────────┐
│ Index:  0     1     2     3     4     5     6         │
│       ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┐     │
│ Value:│  3  │  1  │  3  │  4  │  2  │  6  │  5  │     │
│       └─────┴─────┴─────┴─────┴─────┴─────┴─────┘     │
│                                                       │
│ N = 7, so values must be in range [1, N-1] = [1, 6]   │
│ Possible values: 1, 2, 3, 4, 5, 6 (exactly 6 values)  │
│ But we have 7 slots, so at least one repeats          │
│ Answer: 3 appears at index 0 AND index 2              │
└───────────────────────────────────────────────────────┘
```

WHY this drawing: because you will forget that the constraint 1 to N-1 means every value in the array can legally be used as an index into the array itself (shifted by something perhaps), and this is the trick you will miss, you will try to use extra space like a HashSet or you will try to sort and scan adjacent elements but both approaches miss the point that the array ITSELF can be used as a marker board because value V can point to index V.

FROM WHERE: this array comes from the problem statement where N=7 means indices 0 through 6 and values 1 through 6.

TO WHOM: this constraint speaks directly to the relationship between VALUE space (1 to N-1) and INDEX space (0 to N-1), they overlap almost perfectly, off by one, which you will bungle.

WHY NOT: why not just use a HashSet? Because that uses O(N) extra space. Why not sort? Because that is O(N log N) time. The problem begs for O(N) time O(1) space and you will not see it because you are not staring at the VALUE-INDEX connection.

OF WHAT: of the pigeonhole principle applied to 7 slots with 6 possible values meaning at least ceil(7/6) = 2 elements share the same value.

---

## Drawing 2: The Index-Value Mapping Visualization

```
If value V is at index I, then V can "point to" index V:

Index:   0     1     2     3     4     5     6
Value:   3     1     3     4     2     6     5

Value 3 at index 0  ─────────────────► Index 3
Value 1 at index 1  ──► Index 1 (points to itself!)
Value 3 at index 2  ─────────────────► Index 3 (COLLISION!)
Value 4 at index 3  ───────────────────────► Index 4
Value 2 at index 4  ────────► Index 2
Value 6 at index 5  ─────────────────────────────► Index 6
Value 5 at index 6  ──────────────────────────► Index 5

Observe: Index 3 is pointed to by BOTH index 0 and index 2
         because a[0]=3 and a[2]=3
         The duplicate value is 3
```

WHY this drawing: because the essence of the O(1) space solution is treating each value as a pointer to another index and you will fail to see that when two different indices contain the same value they both point to the same destination index causing a "collision" which is exactly what marks the duplicate.

FROM WHERE: from the realization that if a[i] = V then V is a valid index (since 1 ≤ V ≤ N-1 and array has indices 0 to N-1).

TO WHOM: this collision concept directly connects to cycle detection algorithms and to the negation-marking trick.

WHY NOT: why not track collisions with a separate array? Because that is O(N) space and we are allowed to destroy the original array meaning we can use IT as the tracking mechanism.

OF WHAT: of the fundamental insight that the array is not just data but also a map from indices to indices.

---

## Drawing 3: Numerical Example 1 - The Negation Marking Technique

```
Initial:    [3, 1, 3, 4, 2, 6, 5]    (indices 0-6)

Step-by-step (ATTEMPTING, likely to fail):

i=0: a[0]=3, go to index |3|=3, check a[3]
     a[3]=4 (positive), so mark it negative: a[3]=-4
     Array: [3, 1, 3, -4, 2, 6, 5]

i=1: a[1]=1, go to index |1|=1, check a[1]
     a[1]=1 (positive), so mark it negative: a[1]=-1
     Array: [3, -1, 3, -4, 2, 6, 5]

i=2: a[2]=3, go to index |3|=3, check a[3]
     a[3]=-4 (ALREADY NEGATIVE!!!)
     DUPLICATE FOUND: value 3

The trick: when you visit index V and find it already negative,
           the value V has been seen before, so V is the duplicate.
```

WHY this drawing: because you will forget to take absolute value when reading a[i] since previous iterations may have made a[i] negative, and you will get wrong index calculations, for example if a[1] is -1 you need to use |−1|=1 not -1 as an index.

FROM WHERE: from the realization that "marking" an index as visited can be done by negating its value, which is a reversible transformation (multiply by -1 twice returns original).

TO WHOM: this directly answers how to use O(1) space for tracking, the sign bit becomes the "visited" flag.

WHY NOT: why not use index 0 as a special case? Because a value of 0 cannot be negated meaningfully (−0 = 0), but wait, the constraint says values are 1 to N-1 so 0 never appears as a value, but 0 IS a valid index, so you might bungle what to do when i=0 or when a[i] points to 0.

OF WHAT: of the sign-manipulation technique which requires careful handling of absolute values.

---

## DEEP DIVE: Why is value 3 the duplicate when index 3 has -4?

```
TRACE THE CAUSALITY BACKWARDS:

At i=2, we read a[2]=3
We go to index 3
We find a[3]=-4 (negative!)

Question: WHO made a[3] negative?
Answer: At i=0, we read a[0]=3, went to index 3, found a[3]=4 (positive), marked it -4

         i=0                              i=2
    ┌─────────────┐                  ┌─────────────┐
    │ a[0] = 3    │                  │ a[2] = 3    │
    │     │       │                  │     │       │
    │     ▼       │                  │     ▼       │
    │ index = 3   │                  │ index = 3   │
    │     │       │                  │     │       │
    │     ▼       │                  │     ▼       │
    │ a[3]=4(+)   │   ─────────►     │ a[3]=-4(-)  │
    │ MARK: a[3]=-4                  │ ALREADY NEG!│
    └─────────────┘                  └─────────────┘
    
Both i=0 and i=2 tried to mark the SAME index 3.
Why? Because a[0]=3 and a[2]=3. Same value = same target index.
The duplicate VALUE is 3, not the contents of index 3.
```

WHY: The value at index 3 is -4, but -4 is not the duplicate. The duplicate is the VALUE that CAUSED two different loop iterations to target the same index. Both a[0] and a[2] contain the value 3, so both iterations go to index 3. The first marks it, the second finds it marked. The duplicate value is 3.

---

## DEEP DIVE: The Negation Marking - Middle to End to Start

```
MIDDLE OF THE ALGORITHM: When we are at iteration i:
Step M1: Read a[i], take absolute value → call this V
Step M2: Use V as index → target_index = V
Step M3: Check a[target_index], is it negative?
         If YES → V has been seen before → return V
         If NO  → first time seeing V → mark a[target_index] = -a[target_index]

WHY Step M1 uses absolute value?
Because in earlier iterations, a[i] itself might have been marked negative.

Example at i=3 in array [3, 1, 3, -4, 2, 6, 5]:
  a[3] = -4 (was marked negative by earlier iteration)
  Without abs: target_index = -4 → CRASH (negative index)
  With abs: target_index = |-4| = 4 → valid

END OF ALGORITHM: When do we terminate?
Either: Found negative at target → return the duplicate value V
Or: Loop exhausts all indices → panic (should never happen per constraints)

START OF ALGORITHM: What do we assume?
All values are initially positive (1 to N-1)
No index has been marked yet
```

---

## DEEP DIVE: Index 0 Never Targeted - So What?

```
Array: [3, 4, 3, 2, 1, 5]   N=6, values in [1, 5]

Draw the "points to" arrows:
Index:  0     1     2     3     4     5
Value:  3     4     3     2     1     5
        │     │     │     │     │     │
        ▼     ▼     ▼     ▼     ▼     ▼
Target: 3     4     3     2     1     5

WHO POINTS TO INDEX 0? Nobody!
Value 0 would point to index 0, but values are [1,5], no zeros.

CONFUSION: "If index 0 is never targeted, its sign never gets checked!"

ANSWER: Index 0's sign doesn't need to be CHECKED AS A TARGET.
        Index 0 IS VISITED AS A SOURCE when i=0.
        
Two different roles:
┌─────────────────────────────────────────────────────────┐
│ INDEX AS SOURCE (i=___):  We READ a[i] to get a value   │
│ INDEX AS TARGET:          We CHECK/MARK a[target_index] │
└─────────────────────────────────────────────────────────┘

Index 0 is a SOURCE at i=0. We read a[0]=3, target index 3.
Index 0 is never a TARGET because no value equals 0.

This is FINE because:
- The duplicate is detected when TWO sources point to SAME target
- Index 0 being a source is enough
- Whether index 0 is ever a target is irrelevant
```

---

## DEEP DIVE: Can We Start Loop at i=1?

```
Array: [3, 1, 3, 4, 2, 6, 5]

If we skip i=0 and start at i=1:

i=1: a[1]=1 → index 1 → a[1]=1 (pos) → mark → a[1]=-1
     [3, -1, 3, 4, 2, 6, 5]
     
i=2: a[2]=3 → index 3 → a[3]=4 (pos) → mark → a[3]=-4
     [3, -1, 3, -4, 2, 6, 5]
     
i=3: a[3]=-4 → |−4|=4 → index 4 → a[4]=2 (pos) → mark → a[4]=-2
     [3, -1, 3, -4, -2, 6, 5]

... eventually i=6, loop ends, NO DUPLICATE FOUND!

WHY? Because a[0]=3 was NEVER PROCESSED.
The first occurrence of value 3 is at index 0.
By skipping i=0, we only process the SECOND occurrence at i=2.
When i=2 marks index 3, nothing was there before.
The collision that would have happened (i=0 marks, i=2 finds) never occurs.

RULE: You MUST iterate from i=0 to process ALL sources.
```

---

## DEEP DIVE: Why `for i in 0..a.len()` and NOT `for x in a.iter()`?

```
WHAT YOU WANT TO DO INSIDE THE LOOP:
1. Read a[i] to get a value V
2. Use V as an index to access a[V]
3. Possibly MUTATE a[V] (negate it)

ATTEMPT 1: for x in a.iter()
┌────────────────────────────────────────────────────────────┐
│ for x in a.iter() {                                        │
│     let target = x.abs() as usize;                         │
│     if a[target] < 0 { ... }  // ERROR: can't borrow a     │
│     a[target] = -a[target];   // ERROR: can't borrow a mut │
│ }                                                          │
└────────────────────────────────────────────────────────────┘
WHY ERROR?
- a.iter() borrows the entire array immutably
- While that borrow is active, you cannot access a[target] again
- Rust's borrow checker prevents this: "cannot borrow `a` as mutable 
  because it is also borrowed as immutable"

ATTEMPT 2: for x in a.iter_mut()
┌────────────────────────────────────────────────────────────┐
│ for x in a.iter_mut() {                                    │
│     let target = x.abs() as usize;                         │
│     if a[target] < 0 { ... }  // ERROR: same problem       │
│ }                                                          │
└────────────────────────────────────────────────────────────┘
WHY ERROR?
- a.iter_mut() borrows the entire array mutably
- While that borrow is active, you cannot access a[target]
- Same borrow checker complaint

ATTEMPT 3: for i in 0..a.len()
┌────────────────────────────────────────────────────────────┐
│ for i in 0..a.len() {                                      │
│     let val = a[i].abs() as usize;  // access by index     │
│     if a[val] < 0 {                 // access by computed  │
│         return val as i32;          // index, works!       │
│     }                                                      │
│     a[val] = -a[val];               // mutate, works!      │
│ }                                                          │
└────────────────────────────────────────────────────────────┘
WHY WORKS?
- 0..a.len() creates a Range iterator over numbers 0, 1, 2, ..., n-1
- This Range does NOT borrow the array
- Each a[i] and a[val] is an independent borrow that ends immediately
- No overlapping borrows = borrow checker is happy

NUMERICAL TRACE with a = [2, 1, 2]:

Range 0..3 produces: 0, 1, 2 (three iterations)

i=0: 
  0..3 yields 0
  a[0] accessed → 2
  val = |2| = 2
  a[2] accessed → 2 (positive)
  a[2] mutated → -2
  Array: [2, 1, -2]

i=1:
  0..3 yields 1
  a[1] accessed → 1
  val = |1| = 1
  a[1] accessed → 1 (positive)
  a[1] mutated → -1
  Array: [2, -1, -2]

i=2:
  0..3 yields 2
  a[2] accessed → -2
  val = |-2| = 2
  a[2] accessed → -2 (NEGATIVE!)
  return 2

THE KEY INSIGHT:
0..a.len() gives you INDICES (numbers)
a.iter() gives you VALUES (references)

You need INDICES because:
- You compute target_index = |a[i]|
- You access a[target_index]
- target_index ≠ i (usually)
- You need random access, not sequential iteration
```

---

## DEEP DIVE: Self-Pointing Example - Why Not Third Iteration?

```
Array: [2, 1, 3, 4, 3, 5]   N=6

You expected duplicate found at i=2 because a[2]=3 and a[4]=3.
Let me trace why it takes until i=4:

i=0: a[0]=2 → target=2 → a[2]=3 (pos) → mark a[2]=-3
     Source index 0 with value 2 → marks index 2
     Duplicate value 2? NO, we're looking at value 2, nothing else has 2.
     
i=1: a[1]=1 → target=1 → a[1]=1 (pos) → mark a[1]=-1
     Source index 1 with value 1 → marks index 1 (self-pointing)
     
i=2: a[2]=-3 → |−3|=3 → target=3 → a[3]=4 (pos) → mark a[3]=-4
     Source index 2 with value 3 → marks index 3
     THIS IS FIRST TIME value 3 is processed as a source!
     Index 3 was not marked before, so no duplicate yet.
     
i=3: a[3]=-4 → |−4|=4 → target=4 → a[4]=3 (pos) → mark a[4]=-3
     Source index 3 with value 4 → marks index 4
     
i=4: a[4]=-3 → |−3|=3 → target=3 → a[3]=-4 (NEGATIVE!)
     Source index 4 with value 3 → goes to index 3
     Index 3 was marked at i=2!
     DUPLICATE FOUND: value 3

WHY NOT AT i=2?
At i=2, value 3 is being processed for the FIRST time.
No earlier iteration processed value 3.
i=0 processed value 2.
i=1 processed value 1.
i=2 is first to process value 3, so it marks index 3.
i=4 is second to process value 3, so it finds index 3 already marked.
```

---

## DEEP DIVE: Failure 4 - Check Before Mark, Not After

```
WRONG ORDER (mark first, check second):
target = |a[i]|
a[target] = -a[target]    // mark FIRST
if a[target] < 0:         // check SECOND - ALWAYS TRUE NOW!
    return target

Why always true? Because you just made it negative on the line before!

Example with [2, 1, 2]:
i=0: target=2, mark a[2]=-2, check a[2]<0? YES! return 2
WRONG because this is first time seeing value 2!

CORRECT ORDER (check first, mark second):
target = |a[i]|
if a[target] < 0:         // check FIRST - was it already marked?
    return target         // yes → duplicate
a[target] = -a[target]    // mark SECOND - now mark it for future

Example with [2, 1, 2]:
i=0: target=2, check a[2]=2>0? NO not negative, mark a[2]=-2
i=1: target=1, check a[1]=1>0? NO not negative, mark a[1]=-1
i=2: target=|−2|=2, check a[2]=-2<0? YES! return 2
CORRECT!
```

---

## DEEP DIVE: Failure 6 - Return Value vs Return Index

```
At moment of detection:
i = 4 (current loop index, WHERE we are in the loop)
a[i] = a[4] = -3 (value at current position, already marked negative by someone)
|a[i]| = 3 (the ACTUAL VALUE that is duplicated)
target_index = 3 (where we went to check)

WHAT TO RETURN?
- Return i=4? NO! 4 is just where we are in the loop
- Return target_index=3? NO! 3 is where we checked, not the duplicate value
- Return |a[i]|=3? YES! This is the value that appeared twice

WHY |a[i]| and not a[i]?
Because a[i] might be negative (marked by earlier iteration).
We need the actual value, which is the absolute value.

Array: [2, 1, 3, 4, 3, 5]
At i=4: a[4]=-3 (was marked negative at i=3)
        |a[4]| = 3
        We return 3, which is correct: value 3 appears at index 2 and index 4.
```

---

## Numerical Example 2: Off-By-One Disaster

```
Array: [2, 1, 2]   N=3, values in [1, 2]
Indices: 0, 1, 2

Attempt (WRONG - using value directly as index):
i=0: a[0]=2, go to index 2, a[2]=2 (positive), mark: a[2]=-2
     Array: [2, 1, -2]
i=1: a[1]=1, go to index 1, a[1]=1 (positive), mark: a[1]=-1
     Array: [2, -1, -2]
i=2: a[2]=-2, value=|−2|=2, go to index 2, a[2]=-2 (NEGATIVE!)
     DUPLICATE FOUND: 2 ✓

Wait, this worked? Let's try another:

Array: [1, 1]   N=2, values in [1, 1]
i=0: a[0]=1, go to index 1, a[1]=1 (positive), mark: a[1]=-1
     Array: [1, -1]
i=1: a[1]=-1, value=|−1|=1, go to index 1, a[1]=-1 (NEGATIVE!)
     DUPLICATE FOUND: 1 ✓

Another: [1, 2, 1]  N=3, values in [1, 2]
i=0: a[0]=1, go to index 1, a[1]=2 (positive), mark: a[1]=-2
     Array: [1, -2, 1]
i=1: a[1]=-2, value=|−2|=2, go to index 2, a[2]=1 (positive), mark: a[2]=-1
     Array: [1, -2, -1]
i=2: a[2]=-1, value=|−1|=1, go to index 1, a[1]=-2 (NEGATIVE!)
     DUPLICATE FOUND: 1 ✓
```

WHY this drawing: because off-by-one errors will haunt you, specifically whether to use a[i] directly as index or a[i]-1 as index, and forgetting to take absolute value, and the iteration stopping condition.

FROM WHERE: from the constraint that values are 1 to N-1 but indices are 0 to N-1, a perfect off-by-one trap.

TO WHOM: this is a direct challenge to your index calculation skills.

WHY NOT: why not shift everything by 1? Because then you introduce another layer of translation errors.

OF WHAT: of the most common bug in index-based algorithms.

---

## Numerical Example 3: The 0-Index Problem

```
Array: [5, 4, 3, 2, 1, 3]   N=6, values in [1, 5]

Question: What happens when a value points to index 0?

i=0: a[0]=5, go to index 5, a[5]=3 (positive), mark: a[5]=-3
     Array: [5, 4, 3, 2, 1, -3]
i=1: a[1]=4, go to index 4, a[4]=1 (positive), mark: a[4]=-1
     Array: [5, 4, 3, 2, -1, -3]
i=2: a[2]=3, go to index 3, a[3]=2 (positive), mark: a[3]=-2
     Array: [5, 4, 3, -2, -1, -3]
i=3: a[3]=-2, value=|−2|=2, go to index 2, a[2]=3 (positive), mark: a[2]=-3
     Array: [5, 4, -3, -2, -1, -3]
i=4: a[4]=-1, value=|−1|=1, go to index 1, a[1]=4 (positive), mark: a[1]=-4
     Array: [5, -4, -3, -2, -1, -3]
i=5: a[5]=-3, value=|−3|=3, go to index 3, a[3]=-2 (NEGATIVE!)
     DUPLICATE FOUND: 3 ✓

But what if array was [3, 4, 3, 2, 1, 5]?
The value 3 would point to index 3, not 0.
The value 1 at some index would point to index 1.
No value ever points to index 0 in this scheme!

Problem: if no value points to index 0, index 0's sign never gets checked!
This is fine because we iterate through ALL indices including 0.
```

WHY this drawing: because you will wonder what happens with index 0 and confuse yourself about whether all indices get visited, the answer is yes because we loop i from 0 to N-1.

FROM WHERE: from the asymmetry between value range [1, N-1] and index range [0, N-1].

TO WHOM: this addresses the confusion about boundary indices.

WHY NOT: why not start the loop at i=1? Because then you would skip checking if a[0]'s pointed-to index was already marked.

OF WHAT: of boundary condition analysis.

---

## Numerical Example 4: Multiple Duplicates

```
Array: [2, 2, 2, 2]   N=4, values in [1, 3]  WAIT - 2 is in [1, 3] ✓

i=0: a[0]=2, go to index 2, a[2]=2 (positive), mark: a[2]=-2
     Array: [2, 2, -2, 2]
i=1: a[1]=2, value=2, go to index 2, a[2]=-2 (NEGATIVE!)
     DUPLICATE FOUND: 2 ✓

We found it early! We don't need to check all elements.

Another: [1, 2, 3, 1, 2, 3]   N=6, values in [1, 5]
i=0: a[0]=1, go to index 1, a[1]=2 (positive), mark: a[1]=-2
     Array: [1, -2, 3, 1, 2, 3]
i=1: a[1]=-2, value=2, go to index 2, a[2]=3 (positive), mark: a[2]=-3
     Array: [1, -2, -3, 1, 2, 3]
i=2: a[2]=-3, value=3, go to index 3, a[3]=1 (positive), mark: a[3]=-1
     Array: [1, -2, -3, -1, 2, 3]
i=3: a[3]=-1, value=1, go to index 1, a[1]=-2 (NEGATIVE!)
     DUPLICATE FOUND: 1 ✓

The problem only asks for ANY duplicate, not ALL duplicates.
```

WHY this drawing: because you might think you need to find all duplicates or count them, but the function prototype asks for a single int return, meaning any one duplicate suffices.

FROM WHERE: from the function prototype `int FindDuplicate(int[] a)`.

TO WHOM: this clarifies the termination condition: return immediately when first duplicate is found.

WHY NOT: why not continue to find all? Because the problem says "any duplicated integer" singular.

OF WHAT: of careful reading of the problem statement.

---

## Numerical Example 5: Larger Scale Calculation

```
N = 10, Array: [7, 3, 1, 8, 2, 5, 9, 7, 4, 6]
Values in range [1, 9], indices [0, 9]

Trace:
i=0: a[0]=7 → index 7, a[7]=7 (pos) → mark → a[7]=-7
     [7, 3, 1, 8, 2, 5, 9, -7, 4, 6]
     
i=1: a[1]=3 → index 3, a[3]=8 (pos) → mark → a[3]=-8
     [7, 3, 1, -8, 2, 5, 9, -7, 4, 6]
     
i=2: a[2]=1 → index 1, a[1]=3 (pos) → mark → a[1]=-3
     [7, -3, 1, -8, 2, 5, 9, -7, 4, 6]
     
i=3: a[3]=-8 → |−8|=8 → index 8, a[8]=4 (pos) → mark → a[8]=-4
     [7, -3, 1, -8, 2, 5, 9, -7, -4, 6]
     
i=4: a[4]=2 → index 2, a[2]=1 (pos) → mark → a[2]=-1
     [7, -3, -1, -8, 2, 5, 9, -7, -4, 6]
     
i=5: a[5]=5 → index 5, a[5]=5 (pos) → mark → a[5]=-5
     [7, -3, -1, -8, 2, -5, 9, -7, -4, 6]
     
i=6: a[6]=9 → index 9, a[9]=6 (pos) → mark → a[9]=-6
     [7, -3, -1, -8, 2, -5, 9, -7, -4, -6]
     
i=7: a[7]=-7 → |−7|=7 → index 7, a[7]=-7 (NEGATIVE!)
     DUPLICATE FOUND: 7 ✓
```

WHY this drawing: because a larger trace forces you to track the absolute value extraction step which you will forget, notice at i=3 and i=7 we must use absolute value.

FROM WHERE: from the need to stress-test your algorithm on more than minimal examples.

TO WHOM: this is practice for your brain to not lose track of the sign bit.

WHY NOT: why not use N=100? Because the pattern is clear by N=10.

OF WHAT: of the algorithm's behavior on non-trivial input.

---

## Numerical Example 6: Edge Case - Minimum Size

```
N = 2, Array: [1, 1]
Values in range [1, 1], indices [0, 1]

Only one possible value! Must be duplicate.

i=0: a[0]=1 → index 1, a[1]=1 (pos) → mark → a[1]=-1
     [1, -1]
i=1: a[1]=-1 → |−1|=1 → index 1, a[1]=-1 (NEGATIVE!)
     DUPLICATE FOUND: 1 ✓

N = 2, Array: [1, 1] has:
- 2 elements
- Values must be in [1, N-1] = [1, 1]
- Only value 1 is possible
- Pigeonhole: 2 slots, 1 value → guaranteed duplicate
```

WHY this drawing: because N=2 is the minimal non-trivial case and you might have off-by-one errors in your loop bounds.

FROM WHERE: from the constraint that N must be at least 2 for the problem to make sense.

TO WHOM: this tests your loop: `for i in 0..N` should visit both indices 0 and 1.

WHY NOT: why not N=1? Because then range [1, N-1] = [1, 0] is empty/invalid.

OF WHAT: of boundary case validation.

---

## Numerical Example 7: Self-Pointing Values

```
Array: [2, 1, 3, 4, 3, 5]   N=6, values in [1, 5]

Note: a[1]=1 (self-pointing: value 1 at index 1)
      a[3]=4 but index 4 has value 3... not self-pointing

i=0: a[0]=2 → index 2, a[2]=3 (pos) → mark → a[2]=-3
     [2, 1, -3, 4, 3, 5]
     
i=1: a[1]=1 → index 1, a[1]=1 (pos) → mark → a[1]=-1
     [2, -1, -3, 4, 3, 5]
     
i=2: a[2]=-3 → |−3|=3 → index 3, a[3]=4 (pos) → mark → a[3]=-4
     [2, -1, -3, -4, 3, 5]
     
i=3: a[3]=-4 → |−4|=4 → index 4, a[4]=3 (pos) → mark → a[4]=-3
     [2, -1, -3, -4, -3, 5]
     
i=4: a[4]=-3 → |−3|=3 → index 3, a[3]=-4 (NEGATIVE!)
     DUPLICATE FOUND: 3 ✓

Self-pointing is fine because we still mark and check.
```

WHY this drawing: because self-pointing values (where value equals index) might confuse you into thinking infinite loops could occur, but the negation still works.

FROM WHERE: from the realization that if a[k]=k then visiting index k marks a[k] negative, and revisiting k later will find it negative.

TO WHOM: this addresses the paranoia about cycles and infinite loops.

WHY NOT: why not special-case self-pointing? Because the algorithm handles it uniformly.

OF WHAT: of algorithm robustness.

---

## Predicted Logic Failures

### Failure 1: Forgetting Absolute Value
You will write `a[a[i]]` without taking absolute value of `a[i]`, and when `a[i]` is negative (because it was marked earlier), you will get a negative index or array out of bounds error, the fix is `a[abs(a[i])]` but you will forget this on your first attempt.

### Failure 2: Off-By-One Index Confusion
You will confuse whether to use `a[i]` or `a[i]-1` as the target index, since values are in [1, N-1] and indices are [0, N-1], some versions of this algorithm use `a[i]-1` to map value V to index V-1, you will pick the wrong one and get wrong results.

### Failure 3: Not Returning Early
You will write a loop that marks all elements first and then scans for duplicates in a second pass, missing the elegant early-return when you first encounter a negative during marking, wasting time and overcomplicating.

### Failure 4: Checking Before Marking vs After
You will get the order wrong: you must CHECK if `a[target_index]` is already negative BEFORE marking it, but you will mark first and check second, thus never finding the duplicate.

### Failure 5: Integer Overflow with Large Values
In Rust, if the values are u32 or i32 and you try to negate a value at the boundary (like i32::MIN), you will get overflow panic in debug mode, the constraint says values are 1 to N-1 so this cannot happen, but you will still worry about it and waste time.

### Failure 6: Using Wrong Variable for Return
When you find the duplicate, you will return `i` (the current loop index) instead of `abs(a[i])` (the value that is duplicated), or vice versa, mixing up "where I am" with "what value is duplicated".

### Failure 7: Mutating While Iterating Confusion
You will try to iterate with a for-each loop and mutate simultaneously, Rust's borrow checker will scream at you, you need indexed access with `for i in 0..n` not `for x in a.iter()`.

---

## Time and Space Complexity Analysis

Time Complexity is O(N) because we iterate through the array once, and each iteration does O(1) work consisting of reading a value, computing absolute value, reading at computed index, potentially negating, and returning early if duplicate found.

Space Complexity is O(1) because we only use the input array itself for marking, no additional data structures, the "may destroy the array" permission is what enables this.

---

## Function Prototype Translation

```
C:    int FindDuplicate(int[] a)
Rust: fn find_duplicate(a: &mut [i32]) -> i32
```

The Rust version takes a mutable slice because we may destroy (mutate) the array, and returns i32 because that is the duplicated value.
