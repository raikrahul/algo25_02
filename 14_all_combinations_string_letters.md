# All Combinations of String Letters

## Problem

```
Input: "ABC"
Output: "A", "B", "C", "AB", "AC", "BC", "ABC"
```

## Numerical Analysis

### Input → State Evolution

```
s = "ABC"
n = 3
chars = ['A', 'B', 'C']
indices = [0, 1, 2]
```

### Expected Output Count

```
n = 3
2^n - 1 = 2^3 - 1 = 8 - 1 = 7 combinations
∴ 7 non-empty subsets

Breakdown:
1-char: C(3,1) = 3
2-char: C(3,2) = 3  
3-char: C(3,3) = 1
Total: 3 + 3 + 1 = 7 ✓
```

### State Space Exploration

```
At index 0 ('A'):
  ├─ Include 'A' → tray = ['A']
  │   At index 1 ('B'):
  │     ├─ Include 'B' → tray = ['A','B']
  │     │   At index 2 ('C'):
  │     │     ├─ Include 'C' → tray = ['A','B','C'] → output "ABC"
  │     │     └─ Exclude 'C' → tray = ['A','B'] → output "AB"
  │     └─ Exclude 'B' → tray = ['A']
  │         At index 2 ('C'):
  │           ├─ Include 'C' → tray = ['A','C'] → output "AC"
  │           └─ Exclude 'C' → tray = ['A'] → output "A"
  └─ Exclude 'A' → tray = []
      At index 1 ('B'):
        ├─ Include 'B' → tray = ['B']
        │   At index 2 ('C'):
        │     ├─ Include 'C' → tray = ['B','C'] → output "BC"
        │     └─ Exclude 'C' → tray = ['B'] → output "B"
        └─ Exclude 'B' → tray = []
            At index 2 ('C'):
              ├─ Include 'C' → tray = ['C'] → output "C"
              └─ Exclude 'C' → tray = [] → output "" ✗ (excluded)
```

### Failure Pattern Prediction #1: Base Case Mishandling

```
char_pos = 0, 1, 2, 3
n = 3

WRONG: char_pos == n → always output tray
  → char_pos = 3, tray = [] → outputs "" ✗

RIGHT: char_pos == n ∧ tray ≠ [] → output tray
  → char_pos = 3, tray = [] → skip ✓
  → char_pos = 3, tray = ['A'] → output "A" ✓
```

### Failure Pattern Prediction #2: Missing Branch

```
At char_pos = 0:
  WRONG: Only recurse with include
    helper(0) → tray.push('A') → helper(1) → ...
    Result: {A, AB, ABC} (3 combinations) ✗
    Missing: 2^3 - 1 - 3 = 4 combinations

  RIGHT: Recurse twice (include branch + exclude branch)
    helper(0) → 
      ├─ tray.push('A') → helper(1) → ... → tray.pop()
      └─ helper(1) → ...
    Result: 7 combinations ✓
```

### Failure Pattern Prediction #3: Pop Timing

```
SCENARIO: char_pos = 1, tray = ['A']

WRONG Order:
  tray.push('B')     → tray = ['A', 'B']
  helper(2)          → explores all paths with "AB" prefix
  // Missing pop!
  helper(2)          → explores with tray STILL = ['A', 'B'] ✗

Resulting tray states:
  First helper(2) completes → tray = ['A', 'B', 'C'] after deep recursion
  Second helper(2) starts → tray = ['A', 'B', 'C'] (corrupted) ✗

RIGHT Order:
  tray.push('B')     → tray = ['A', 'B']
  helper(2)          → explores all paths with "AB" prefix  
  tray.pop()         → tray = ['A'] (restored)
  helper(2)          → explores with tray = ['A'] ✓

∴ push → recurse → pop = mandatory sandwich
```

### Failure Pattern Prediction #4: Index Progression

```
WRONG: helper(char_pos) calls helper(char_pos)
  → infinite loop
  → stack overflow after ~10000 calls

WRONG: helper(char_pos) calls helper(char_pos + 2)
  → skips every other character
  → n=3: processes indices [0, 2] only
  → missing index 1 ('B')
  → outputs: {A, AC, C} (3 combinations) ✗

RIGHT: helper(char_pos) calls helper(char_pos + 1)
  → processes indices [0, 1, 2, 3]
  → char_pos increments: 0→1, 1→2, 2→3
  → 3 == n → base case ✓
```

### Data Structure States (Example: "ABC")

```
Call Stack Depth vs Tray Size:

Depth 0: char_pos=0, tray=[]
Depth 1: char_pos=1, tray=['A'] or []
Depth 2: char_pos=2, tray=['A','B'] or ['A'] or ['B'] or []
Depth 3: char_pos=3, tray=['A','B','C'] or ['A','B'] or ['A','C'] or ['A'] or ['B','C'] or ['B'] or ['C'] or []

Max depth = n + 1 = 4
Max tray size = n = 3
```

### Failure Pattern Prediction #5: Output Timing

```
WRONG: Output after BOTH recursive calls

  fn helper(pos) {
    if pos == n {
      if !tray.is_empty() { output(tray) }
      return
    }
    
    tray.push(s[pos])
    helper(pos+1)
    tray.pop()
    
    helper(pos+1)
    
    output(tray)  // ✗ Wrong position!
  }

  → Outputs partial combinations multiple times
  → Total outputs: 2^(n+1) - 1 = 15 (not 7) ✗

RIGHT: Output ONLY in base case
  → Outputs exactly when char_pos == n
  → Total outputs: 2^n - 1 = 7 ✓
```

### Numerical Trace (n=2, s="AB")

```
helper(0, [])
  ├─ push('A') → [A]
  │  helper(1, [A])
  │    ├─ push('B') → [A,B]
  │    │  helper(2, [A,B])  
  │    │    → char_pos=2 == n=2 ∧ tray≠[] → OUTPUT "AB" ✓
  │    │  pop('B') → [A]
  │    └─ helper(2, [A])
  │         → char_pos=2 == n=2 ∧ tray≠[] → OUTPUT "A" ✓
  │  pop('A') → []
  └─ helper(1, [])
       ├─ push('B') → [B]
       │  helper(2, [B])
       │    → char_pos=2 == n=2 ∧ tray≠[] → OUTPUT "B" ✓
       │  pop('B') → []
       └─ helper(2, [])
            → char_pos=2 == n=2 ∧ tray=[] → SKIP ✓

Results: ["AB", "A", "B"] → 3 combinations
Expected: 2^2 - 1 = 3 ✓
```

### Failure Pattern Prediction #6: String Building

```
WRONG: Building string during recursion
  fn helper(pos, current_str: String)
  → current_str = "AB"
  → After helper call, current_str STILL = "AB" (String not mutated) ✗
  → Need to manually remove last char

RIGHT: Use Vec<char> as tray
  → tray.push('A') → tray = ['A']
  → tray.pop() → tray = []
  → O(1) operations ✓

Performance:
  String: clone per call → O(n) per operation → O(n * 2^n) total
  Vec<char>: push/pop → O(1) per operation → O(2^n) total
  ∴ Vec is n times faster
```

### Calculation Challenge #1: Small Scale (n=1)

```
s = "X"
n = 1

Tree:
helper(0, [])
  ├─ push('X') → [X]
  │  helper(1, [X]) → OUTPUT "X" ✓
  │  pop() → []
  └─ helper(1, []) → SKIP ✓

Output count: 1
Expected: 2^1 - 1 = 1 ✓
```

### Calculation Challenge #2: Mid Scale (n=4)

```
s = "ABCD"
n = 4

Expected outputs: 2^4 - 1 = 15

1-char: C(4,1) = 4
2-char: C(4,2) = 6
3-char: C(4,3) = 4
4-char: C(4,4) = 1
Total: 4+6+4+1 = 15 ✓

Max call depth: 4 + 1 = 5
Total function calls: 2^5 - 1 = 31
Calls with output: 15
Calls skipped: 1 (empty tray at depth 5)
Calls internal: 31 - 15 - 1 = 15
```

### Calculation Challenge #3: Edge Case (n=0)

```
s = ""
n = 0

helper(0, [])
  → char_pos=0 == n=0 ∧ tray=[] → SKIP ✓
  → No recursive calls
  → No outputs

Expected: 2^0 - 1 = 0 ✓
```

### Calculation Challenge #4: Performance (n=10)

```
s = "ABCDEFGHIJ"
n = 10

Output count: 2^10 - 1 = 1023
Function calls: 2^11 - 1 = 2047

For n=20:
Output count: 2^20 - 1 = 1,048,575
Function calls: 2^21 - 1 = 2,097,151

For n=25:
Output count: 2^25 - 1 = 33,554,431
Function calls: 2^26 - 1 = 67,108,863

∴ Exponential growth → infeasible for n > 25
```

### Calculation Challenge #5: Memory (n=3)

```
s = "ABC"
n = 3

Per call stack frame:
  char_pos: usize = 8 bytes
  tray: &mut Vec<char> = 8 bytes (reference)
  Total ~16 bytes per frame

Max depth = 4
Max stack memory = 4 × 16 = 64 bytes

Tray storage (shared):
  Max size = 3 chars
  Vec overhead = 24 bytes
  Char storage = 3 × 4 = 12 bytes
  Total = 36 bytes

Total memory: 64 + 36 = 100 bytes ✓
```

### Calculation Challenge #6: Branch Factor

```
At each char_pos:
  2 recursive calls (include, exclude)

Branch factor = 2
Depth = n + 1

Total nodes in tree = 2^0 + 2^1 + 2^2 + ... + 2^n
                    = 2^(n+1) - 1

For n=3:
  Depth 0: 1 node
  Depth 1: 2 nodes  
  Depth 2: 4 nodes
  Depth 3: 8 nodes
  Total: 1+2+4+8 = 15 = 2^4 - 1 ✓
```

### Calculation Challenge #7: Fractional Analysis

```
Probability of outputting at leaf node:

Total leaf nodes = 2^n
Valid outputs = 2^n - 1 (exclude empty)

P(output) = (2^n - 1) / 2^n = 1 - 1/2^n

For n=3:
  P(output) = 7/8 = 0.875 = 87.5%

For n=10:
  P(output) = 1023/1024 ≈ 0.999 = 99.9%

For n=1:
  P(output) = 1/2 = 0.5 = 50%

∴ As n↑, P(output)→1
```

## Predicted Failures Summary

```
1. char_pos == n → output WITHOUT checking tray.is_empty()
   → Outputs empty string ✗

2. Missing exclude branch → Only include path explored
   → Outputs n combinations instead of 2^n - 1 ✗

3. Missing tray.pop() between branches
   → Tray state corrupted for exclude branch ✗

4. helper(char_pos + 2) instead of helper(char_pos + 1)
   → Skips characters ✗

5. Output after recursive calls instead of in base case
   → Duplicate/partial outputs ✗

6. Using String instead of Vec<char>
   → O(n * 2^n) instead of O(2^n) ✗

7. Infinite recursion: helper(char_pos) calls helper(char_pos)
   → Stack overflow ✗
```
