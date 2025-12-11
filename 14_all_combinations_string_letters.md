# Problem 14: All Combinations of String Letters

Given a string S, write a function that displays all the combinations of the characters from the string. Assume that input string will not contain any repetition of characters.
Example: Input String: “ABC”, Output combinations: “A”, “B”, “C”, “AB”, “AC”, “BC” and “ABC”.

       [ ]
      /   \
    NO     YES(A)
    /       \
  [ ]       [A]
  / \       / \
 NO YES(B) NO YES(B)
 |    |     |    |
[ ]  [B]   [A]  [AB]
/\    /\    /\    /\
N Y(C)N Y  N Y  N Y
| |   | |  | |  | |
[][C] [B][BC][A][AC][AB][ABC]

You will gloss over the tree above. You will look at the final row and nod, "yes, standard power set," without calculating the cost of the edges. You will fail to write the base case correctly, likely printing the empty set `[]` which is not in the example output "A", "B"... etc. You will miss the off-by-one error calculation where $2^3 = 8$ but output count is $7$. You will struggle with the mutable `String` buffer passing: you will push 'A', recurse, then forget to pop 'A', leading to "AB" becoming "AB..." instead of backtracking to "A" then "AC". You will unlikely calculate the actual stack frame size, assuming it's free. You will assume `s[i]` is a single byte without considering the UTF-8 implications if the string was "A🂡C", though the problem implies ASCII. You will fail to manually trace the execution path for "AC" specifically, which requires: Root -> Yes(A) -> No(B) -> Yes(C). You will rush the indices.

What: $2^3 - 1 = 7$ outputs. Total characters printed: $1 \times 3 + 2 \times 3 + 3 \times 1 = 12$. String length $N=3$.
Why: $\sum_{k=1}^{3} \binom{3}{k} = 7$. $111_2 \rightarrow 7_{10}$.
Where: Stack depth $N+1 = 4$. Heap allocations for `current` buffer: 1 (reused) or $2^N$ (if immutable).
Who: 1 main thread.
When: $2^N$ complexity. $N=3 \rightarrow 8$ steps. $N=10 \rightarrow 1024$ steps.
Without: Iterative bitmasking (which would avoid recursion stack) or `pop()` (if using immutable strings).
Which: $101_2$ is $AC$. $011_2$ is $BC$. Indices $0, 2$ used for $AC$.

# The Trap of the Sticky Tray: Manually Tracing the Failure

You are now a mechanical arm moving items into a tray.
**Rule**: You cannot create new trays. You have ONE tray.
Input: `['A', 'B', 'C']` (Indices 0, 1, 2).

**Your Task**:
Trace the exact contents of the tray at each step below. **Do not correct mistakes. Write down exactly what happens if you ONLY add items.**

1. **Start**. Tray: `[]`.
2. **Move to Index 0 ('A')**.
    - Action: Add 'A'.
    - Recurse.
3. **Move to Index 1 ('B')**.
    - Action: Add 'B'.
    - Recurse.
4. **Move to Index 2 ('C')**.
    - Action: Add 'C'.
    - Recurse.
5. **Hit Limit**. Print Tray.
    - **Output #1**: `['A', 'B', 'C']`. (Correct?)
6. **Return** to Index 2 frame.
    - We finished the "Include C" path.
    - Now we do the "Exclude C" path.
    - Recurse (Move to Index 3).
7. **Hit Limit**. Print Tray.
    - **Critical Question**: You did not remove 'C'. What is currently in the tray?
    - **Output #2**: `_________________`.
    - Is this `['A', 'B']`? Or is it `['A', 'B', 'C']`?

8. **Return** to Index 1 frame.
    - We finished "Include B".
    - Now we do "Exclude B".
    - Recurse.
9. **Move to Index 2 ('C')**.
    - Action: Add 'C'.
    - **Critical Question**: What was in the tray *before* you added 'C' this time?
    - **Output #3**: `_________________`.

**The Paradox**:
To get "AC", your tray needs to look like `['A', 'C']`.
But if you never took things out, your tray at Step 9 looks like `['A', 'B', 'C', 'C']` or `['A', 'B', 'C', 'B', 'C']` depending on your exact path.
**Calculate the garbage:** If you run this for N=3 without ever taking items out, how long is the string at the final output?

- Calculated Length: ________.
- Actual RAM usage: ________.

**The Fix**:
You need an operation to "undo" Step 2, Step 3, Step 4.
What is that operation? Where *exactly* does it go?

- Draw the timeline.
- Insert the "Undo" command.
- Verify if `['A', 'B', 'C']` becomes `['A', 'B']` before Step 7.

# Full Execution Trace

```
s = "ABC" → len = 3
tray = [] → &mut String
char_pos = 0 → usize

Step│C#│pos│L# │tray_before→action→tray_after│pos==3?│len>0?│Output
────┼──┼───┼───┼──────────────────────────────┼───────┼──────┼──────
1   │1 │0  │51 │pos==3? → 0==3?✗ → skip if    │       │      │
2   │1 │0  │57 │ch = s[0] → 'A'               │       │      │
3   │1 │0  │61 │[]→push(A)→[A]                │       │      │
4   │1 │0  │62 │[A]→helper(0+1)→              │       │      │
    │  │   │   │    ↓                         │       │      │
5   │2 │1  │51 │pos==3? → 1==3?✗ → skip if    │       │      │
6   │2 │1  │57 │ch = s[1] → 'B'               │       │      │
7   │2 │1  │61 │[A]→push(B)→[A,B]             │       │      │
8   │2 │1  │62 │[A,B]→helper(1+1)→            │       │      │
    │  │   │   │    ↓                         │       │      │
9   │3 │2  │51 │pos==3? → 2==3?✗ → skip if    │       │      │
10  │3 │2  │57 │ch = s[2] → 'C'               │       │      │
11  │3 │2  │61 │[A,B]→push(C)→[A,B,C]         │       │      │
12  │3 │2  │62 │[A,B,C]→helper(2+1)→          │       │      │
    │  │   │   │    ↓                         │       │      │
13  │4 │3  │51 │pos==3? → 3==3?✓ → enter if   │       │      │
14  │4 │3  │52 │len>0? → 3>0?✓                │       │      │
15  │4 │3  │53 │combinations.push("ABC")      │       │      │→"ABC"
16  │4 │3  │55 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
17  │3 │2  │63 │[A,B,C]→pop()→[A,B]           │       │      │
18  │3 │2  │64 │[A,B]→helper(2+1)→            │       │      │
    │  │   │   │    ↓                         │       │      │
19  │5 │3  │51 │pos==3? → 3==3?✓ → enter if   │       │      │
20  │5 │3  │52 │len>0? → 2>0?✓                │       │      │
21  │5 │3  │53 │combinations.push("AB")       │       │      │→"AB"
22  │5 │3  │55 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
23  │3 │2  │65 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
24  │2 │1  │63 │[A,B]→pop()→[A]               │       │      │
25  │2 │1  │64 │[A]→helper(1+1)→              │       │      │
    │  │   │   │    ↓                         │       │      │
26  │6 │2  │51 │pos==3? → 2==3?✗ → skip if    │       │      │
27  │6 │2  │57 │ch = s[2] → 'C'               │       │      │
28  │6 │2  │61 │[A]→push(C)→[A,C]             │       │      │
29  │6 │2  │62 │[A,C]→helper(2+1)→            │       │      │
    │  │   │   │    ↓                         │       │      │
30  │7 │3  │51 │pos==3? → 3==3?✓              │       │      │
31  │7 │3  │52 │len>0? → 2>0?✓                │       │      │
32  │7 │3  │53 │combinations.push("AC")       │       │      │→"AC"
33  │7 │3  │55 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
34  │6 │2  │63 │[A,C]→pop()→[A]               │       │      │
35  │6 │2  │64 │[A]→helper(2+1)→              │       │      │
    │  │   │   │    ↓                         │       │      │
36  │8 │3  │51 │pos==3? → 3==3?✓              │       │      │
37  │8 │3  │52 │len>0? → 1>0?✓                │       │      │
38  │8 │3  │53 │combinations.push("A")        │       │      │→"A"
39  │8 │3  │55 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
40  │6 │2  │65 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
41  │2 │1  │65 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
42  │1 │0  │63 │[A]→pop()→[]                  │       │      │
43  │1 │0  │64 │[]→helper(0+1)→               │       │      │
    │  │   │   │    ↓                         │       │      │
44  │9 │1  │51 │pos==3? → 1==3?✗              │       │      │
45  │9 │1  │57 │ch = s[1] → 'B'               │       │      │
46  │9 │1  │61 │[]→push(B)→[B]                │       │      │
47  │9 │1  │62 │[B]→helper(1+1)→              │       │      │
    │  │   │   │    ↓                         │       │      │
48  │10│2  │51 │pos==3? → 2==3?✗              │       │      │
49  │10│2  │57 │ch = s[2] → 'C'               │       │      │
50  │10│2  │61 │[B]→push(C)→[B,C]             │       │      │
51  │10│2  │62 │[B,C]→helper(2+1)→            │       │      │
    │  │   │   │    ↓                         │       │      │
52  │11│3  │51 │pos==3? → 3==3?✓              │       │      │
53  │11│3  │52 │len>0? → 2>0?✓                │       │      │
54  │11│3  │53 │combinations.push("BC")       │       │      │→"BC"
55  │11│3  │55 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
56  │10│2  │63 │[B,C]→pop()→[B]               │       │      │
57  │10│2  │64 │[B]→helper(2+1)→              │       │      │
    │  │   │   │    ↓                         │       │      │
58  │12│3  │51 │pos==3? → 3==3?✓              │       │      │
59  │12│3  │52 │len>0? → 1>0?✓                │       │      │
60  │12│3  │53 │combinations.push("B")        │       │      │→"B"
61  │12│3  │55 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
62  │10│2  │65 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
63  │9 │1  │63 │[B]→pop()→[]                  │       │      │
64  │9 │1  │64 │[]→helper(1+1)→               │       │      │
    │  │   │   │    ↓                         │       │      │
65  │13│2  │51 │pos==3? → 2==3?✗              │       │      │
66  │13│2  │57 │ch = s[2] → 'C'               │       │      │
67  │13│2  │61 │[]→push(C)→[C]                │       │      │
68  │13│2  │62 │[C]→helper(2+1)→              │       │      │
    │  │   │   │    ↓                         │       │      │
69  │14│3  │51 │pos==3? → 3==3?✓              │       │      │
70  │14│3  │52 │len>0? → 1>0?✓                │       │      │
71  │14│3  │53 │combinations.push("C")        │       │      │→"C"
72  │14│3  │55 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
73  │13│2  │63 │[C]→pop()→[]                  │       │      │
74  │13│2  │64 │[]→helper(2+1)→               │       │      │
    │  │   │   │    ↓                         │       │      │
75  │15│3  │51 │pos==3? → 3==3?✓              │       │      │
76  │15│3  │52 │len>0? → 0>0?✗ → skip push    │       │      │
77  │15│3  │55 │return                        │       │      │→skip
    │  │   │   │    ↑                         │       │      │
78  │13│2  │65 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
79  │9 │1  │65 │return                        │       │      │
    │  │   │   │    ↑                         │       │      │
80  │1 │0  │65 │return                        │       │      │

∴ combinations = ["ABC","AB","AC","A","BC","B","C"]
∴ |combinations| = 7 = 2³ - 1 ✓
∴ total steps = 80
∴ total calls = 15
```

# Confusion Log

## Error 1: BLANK #1 = 1

Q: What goes in `helper(input, ___, tray)`?
Wrong: `1`
Correct: `index + 1`
Why wrong: Confused literal `1` with expression `index + 1`. index=0 → next=1. index=1 → next=2. Pattern: `current + 1`.

## Error 2: BLANK #2 = -1

Q: What goes in second `helper(input, ___, tray)`?
Wrong: `-1`
Correct: `index + 1`
Why wrong: Assumed second call goes backward. Both calls go forward. Difference: tray state (with/without char).

## Error 3: pop(ch)

Wrong: `tray.pop(ch);`
Correct: `tray.pop();`
Why wrong: Assumed pop needs argument. String::pop() takes no argument, removes last char automatically.

## Error 4: Combinations vs combinations

Wrong: `&mut Combinations`
Correct: `&mut combinations`
Why wrong: Capital C. Rust is case-sensitive.

## Error 5: Missing comma

Wrong: `index : usize tray`
Correct: `index: usize, tray`
Why wrong: Forgot comma between parameters.

## Confusion 6: char_pos meaning

Q: "char_pos will be 1,2,3 or 0,1,2?"
Answer: 0,1,2 (then 3 triggers stop).
Confusion: Mixed 1-indexed with 0-indexed.

## Confusion 7: 4 calls for 3 chars

Q: "how can we have 4 calls when string is just abc"
Answer: 3 chars to decide + 1 call to print = 4 calls per path.
Confusion: Expected calls = chars. Forgot base case call.

## Confusion 8: Returning and explore

Q: "you defined paths" / "what is explore rest"
Answer: Undefined terms introduced. Should use only "spawn" and "return".
Confusion: Vocabulary not grounded in prior definitions.

## Confusion 9: tray=[A] when char_pos=3

Q: "how can you push just A when char_pos is 3"
Answer: char_pos tracks position decided. tray tracks chars included. Independent.
Confusion: Assumed char_pos = tray.len().

## Confusion 10: is_empty check frequency

Q: "this check is used just once in the entire calls?"
Answer: Check runs 8 times. Fails 1 time (Call 15).
Confusion: Thought check = special case. Check runs at every leaf.

## Summary Table

| # | Error Type | Wrong | Correct |
|---|------------|-------|---------|
| 1 | Expression vs Literal | `1` | `index + 1` |
| 2 | Direction | `-1` | `+1` |
| 3 | API | `pop(ch)` | `pop()` |
| 4 | Case | `Combinations` | `combinations` |
| 5 | Syntax | missing `,` | `,` required |
| 6 | Indexing | 1-indexed | 0-indexed |
| 7 | Count | 3 calls | 4 calls |
| 8 | Vocabulary | undefined | grounded |
| 9 | Independence | coupled | independent |
| 10 | Frequency | once | 8 times |
