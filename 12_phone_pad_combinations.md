# Phone Pad Combinations: Building Blocks from Scratch


https://notebooklm.google.com/notebook/0bacdeb1-e976-4988-a2e6-f2c19e8d9c1d
---

## BLOCK 1: Raw Physical Input

```
Phone: Nokia 3310
Button pressed: 2, then 8, then 3
Sequence: "283"
```

```
Button 2 on phone has printed: D E F
Button 8 on phone has printed: V W X  
Button 3 on phone has printed: G H I
```

---

## BLOCK 2: Storage in Memory

```
Address 0x1000: ['a','b','c']  ← digit 1
Address 0x1010: ['d','e','f']  ← digit 2
Address 0x1020: ['g','h','i']  ← digit 3
Address 0x1030: ['j','k','l']  ← digit 4
Address 0x1040: ['m','n','o']  ← digit 5
Address 0x1050: ['p','q','r']  ← digit 6
Address 0x1060: ['s','t','u']  ← digit 7
Address 0x1070: ['v','w','x']  ← digit 8
Address 0x1080: ['y','z']      ← digit 9 (ONLY 2)
```

```
To access digit 2's letters:
  digit = '2'
  '2' = ASCII 50
  '0' = ASCII 48
  50 - 48 = 2
  mappings[2] = Address 0x1010 = ['d','e','f']
```

---

## BLOCK 3: Smallest Case (1 digit)

```
Input: "2"
digits: 1
mappings[2] = ['d','e','f']
```

```
Output #1: "d"
Output #2: "e"
Output #3: "f"
Total: 3
```

```
Loop trace:
  j=0: mappings[2][0] = 'd' → OUTPUT
  j=1: mappings[2][1] = 'e' → OUTPUT
  j=2: mappings[2][2] = 'f' → OUTPUT
  j=3: 3 ≥ 3 → STOP
```

---

## BLOCK 4: Two Digits - Multiplication Appears

```
Input: "23"
digit[0] = '2' → mappings[2] = ['d','e','f'] → 3 choices
digit[1] = '3' → mappings[3] = ['g','h','i'] → 3 choices
```

```
Position 0 has 3 choices: d, e, f
Position 1 has 3 choices: g, h, i

Total combinations = 3 × 3 = 9
```

**WHY MULTIPLICATION NOT ADDITION:**
```
If I pick 'd' for position 0:
  I can still pick g OR h OR i for position 1 → 3 sub-combinations
  
If I pick 'e' for position 0:
  I can still pick g OR h OR i for position 1 → 3 sub-combinations
  
If I pick 'f' for position 0:
  I can still pick g OR h OR i for position 1 → 3 sub-combinations
  
Total = 3 + 3 + 3 = 3 × 3 = 9
```

**ENUMERATE ALL 9:**
```
d + g = "dg"    e + g = "eg"    f + g = "fg"
d + h = "dh"    e + h = "eh"    f + h = "fh"
d + i = "di"    e + i = "ei"    f + i = "fi"
```

---

## BLOCK 5: Two Nested Loops for 2 Digits

```
Input: "23"

for i in 0..3:           // i loops over mappings[2]
    for j in 0..3:       // j loops over mappings[3]
        output mappings[2][i] + mappings[3][j]
```

**TRACE:**
```
i=0, j=0: 'd' + 'g' = "dg" → OUTPUT #1
i=0, j=1: 'd' + 'h' = "dh" → OUTPUT #2
i=0, j=2: 'd' + 'i' = "di" → OUTPUT #3
i=0, j=3: j ≥ 3 → inner loop ends
i=1, j=0: 'e' + 'g' = "eg" → OUTPUT #4
i=1, j=1: 'e' + 'h' = "eh" → OUTPUT #5
i=1, j=2: 'e' + 'i' = "ei" → OUTPUT #6
i=1, j=3: j ≥ 3 → inner loop ends
i=2, j=0: 'f' + 'g' = "fg" → OUTPUT #7
i=2, j=1: 'f' + 'h' = "fh" → OUTPUT #8
i=2, j=2: 'f' + 'i' = "fi" → OUTPUT #9
i=2, j=3: j ≥ 3 → inner loop ends
i=3: i ≥ 3 → outer loop ends
```

**COUNT CHECK:** 9 outputs ✓

---

## BLOCK 6: Three Digits - Need Third Loop

```
Input: "283"
mappings[2] = ['d','e','f'] → 3
mappings[8] = ['v','w','x'] → 3
mappings[3] = ['g','h','i'] → 3

Total = 3 × 3 × 3 = 27
```

```
for i in 0..3:           // mappings[2]
    for j in 0..3:       // mappings[8]
        for k in 0..3:   // mappings[3]
            output mappings[2][i] + mappings[8][j] + mappings[3][k]
```

**PARTIAL TRACE (first 9 of 27):**
```
i=0, j=0, k=0: 'd'+'v'+'g' = "dvg" → OUTPUT #1
i=0, j=0, k=1: 'd'+'v'+'h' = "dvh" → OUTPUT #2
i=0, j=0, k=2: 'd'+'v'+'i' = "dvi" → OUTPUT #3
i=0, j=1, k=0: 'd'+'w'+'g' = "dwg" → OUTPUT #4
i=0, j=1, k=1: 'd'+'w'+'h' = "dwh" → OUTPUT #5
i=0, j=1, k=2: 'd'+'w'+'i' = "dwi" → OUTPUT #6
i=0, j=2, k=0: 'd'+'x'+'g' = "dxg" → OUTPUT #7
i=0, j=2, k=1: 'd'+'x'+'h' = "dxh" → OUTPUT #8
i=0, j=2, k=2: 'd'+'x'+'i' = "dxi" → OUTPUT #9
i=1, j=0, k=0: 'e'+'v'+'g' = "evg" → OUTPUT #10
... continues to OUTPUT #27
```

---

## BLOCK 7: The Problem - Unknown Number of Digits

```
Input: "12345678" → 8 digits → need 8 nested loops
Input: "1"        → 1 digit  → need 1 loop
Input: "23"       → 2 digits → need 2 nested loops
```

**PROBLEM:** 
```
Number of loops = input.len()
input.len() = UNKNOWN at compile time
Cannot write N loops when N is variable
```

---

## BLOCK 8: The Stack Emerges

```
Instead of N nested loops, use ONE loop + STACK

Stack = data structure where:
  PUSH = add to top
  POP = remove from top
  Last In, First Out (LIFO)
```

**SIMULATE "23" WITH STACK - COMPLETE TRACE:**

```
Input: "23"
mappings[2] = ['d','e','f']
mappings[3] = ['g','h','i']
Expected outputs: 9

═══════════════════════════════════════════════════════════════════════════════

STEP 1: PUSH 'd'
  ┌───┐
  │ d │ ← top (from mappings[2][0])
  └───┘
  stack.len() = 1
  input.len() = 2
  1 ≠ 2 → NOT OUTPUT → go deeper

═══════════════════════════════════════════════════════════════════════════════

STEP 2: PUSH 'g'
  ┌───┐
  │ g │ ← top (from mappings[3][0])
  ├───┤
  │ d │
  └───┘
  stack.len() = 2
  input.len() = 2
  2 = 2 → OUTPUT #1: "dg"

═══════════════════════════════════════════════════════════════════════════════

STEP 3: POP 'g'
  ┌───┐
  │ d │ ← top
  └───┘
  'g' was mappings[3][0]
  next index = 1
  1 < 3 → more siblings in mappings[3]
  → PUSH next sibling

═══════════════════════════════════════════════════════════════════════════════

STEP 4: PUSH 'h'
  ┌───┐
  │ h │ ← top (from mappings[3][1])
  ├───┤
  │ d │
  └───┘
  stack.len() = 2 = 2 → OUTPUT #2: "dh"

═══════════════════════════════════════════════════════════════════════════════

STEP 5: POP 'h'
  ┌───┐
  │ d │ ← top
  └───┘
  'h' was mappings[3][1]
  next index = 2
  2 < 3 → more siblings in mappings[3]
  → PUSH next sibling

═══════════════════════════════════════════════════════════════════════════════

STEP 6: PUSH 'i'
  ┌───┐
  │ i │ ← top (from mappings[3][2])
  ├───┤
  │ d │
  └───┘
  stack.len() = 2 = 2 → OUTPUT #3: "di"

═══════════════════════════════════════════════════════════════════════════════

STEP 7: POP 'i'
  ┌───┐
  │ d │ ← top
  └───┘
  'i' was mappings[3][2]
  next index = 3
  3 ≥ 3 → mappings[3] EXHAUSTED
  → must POP parent too

═══════════════════════════════════════════════════════════════════════════════

STEP 8: POP 'd' (SECOND CONSECUTIVE POP)
  └───┘
  stack = []
  'd' was mappings[2][0]
  next index = 1
  1 < 3 → more siblings in mappings[2]
  → PUSH next sibling 'e'

═══════════════════════════════════════════════════════════════════════════════

STEP 9: PUSH 'e'
  ┌───┐
  │ e │ ← top (from mappings[2][1])
  └───┘
  stack.len() = 1 ≠ 2 → NOT OUTPUT → go deeper

═══════════════════════════════════════════════════════════════════════════════

STEP 10: PUSH 'g'
  ┌───┐
  │ g │ ← top (from mappings[3][0], restart from beginning)
  ├───┤
  │ e │
  └───┘
  stack.len() = 2 = 2 → OUTPUT #4: "eg"

═══════════════════════════════════════════════════════════════════════════════

STEP 11: POP 'g'
  ┌───┐
  │ e │ ← top
  └───┘
  next index = 1 < 3 → PUSH 'h'

═══════════════════════════════════════════════════════════════════════════════

STEP 12: PUSH 'h'
  ┌───┐
  │ h │ ← top
  ├───┤
  │ e │
  └───┘
  stack.len() = 2 = 2 → OUTPUT #5: "eh"

═══════════════════════════════════════════════════════════════════════════════

STEP 13: POP 'h'
  ┌───┐
  │ e │ ← top
  └───┘
  next index = 2 < 3 → PUSH 'i'

═══════════════════════════════════════════════════════════════════════════════

STEP 14: PUSH 'i'
  ┌───┐
  │ i │ ← top
  ├───┤
  │ e │
  └───┘
  stack.len() = 2 = 2 → OUTPUT #6: "ei"

═══════════════════════════════════════════════════════════════════════════════

STEP 15: POP 'i'
  ┌───┐
  │ e │ ← top
  └───┘
  'i' was mappings[3][2]
  next index = 3 ≥ 3 → EXHAUSTED
  → must POP parent too

═══════════════════════════════════════════════════════════════════════════════

STEP 16: POP 'e' (SECOND CONSECUTIVE POP)
  └───┘
  stack = []
  'e' was mappings[2][1]
  next index = 2
  2 < 3 → more siblings in mappings[2]
  → PUSH 'f'

═══════════════════════════════════════════════════════════════════════════════

STEP 17: PUSH 'f'
  ┌───┐
  │ f │ ← top (from mappings[2][2])
  └───┘
  stack.len() = 1 ≠ 2 → NOT OUTPUT → go deeper

═══════════════════════════════════════════════════════════════════════════════

STEP 18: PUSH 'g'
  ┌───┐
  │ g │ ← top
  ├───┤
  │ f │
  └───┘
  stack.len() = 2 = 2 → OUTPUT #7: "fg"

═══════════════════════════════════════════════════════════════════════════════

STEP 19: POP 'g'
  ┌───┐
  │ f │ ← top
  └───┘
  next index = 1 < 3 → PUSH 'h'

═══════════════════════════════════════════════════════════════════════════════

STEP 20: PUSH 'h'
  ┌───┐
  │ h │ ← top
  ├───┤
  │ f │
  └───┘
  stack.len() = 2 = 2 → OUTPUT #8: "fh"

═══════════════════════════════════════════════════════════════════════════════

STEP 21: POP 'h'
  ┌───┐
  │ f │ ← top
  └───┘
  next index = 2 < 3 → PUSH 'i'

═══════════════════════════════════════════════════════════════════════════════

STEP 22: PUSH 'i'
  ┌───┐
  │ i │ ← top
  ├───┤
  │ f │
  └───┘
  stack.len() = 2 = 2 → OUTPUT #9: "fi"

═══════════════════════════════════════════════════════════════════════════════

STEP 23: POP 'i'
  ┌───┐
  │ f │ ← top
  └───┘
  'i' was mappings[3][2]
  next index = 3 ≥ 3 → EXHAUSTED
  → must POP parent

═══════════════════════════════════════════════════════════════════════════════

STEP 24: POP 'f' (SECOND CONSECUTIVE POP)
  └───┘
  stack = []
  'f' was mappings[2][2]
  next index = 3
  3 ≥ 3 → mappings[2] EXHAUSTED
  → no more siblings
  → DONE

═══════════════════════════════════════════════════════════════════════════════

SUMMARY:
  Total PUSH: 12
  Total POP:  12 (balanced ✓)
  Total OUTPUT: 9 = 3 × 3 ✓
  
  Outputs in order:
  #1: dg    #2: dh    #3: di
  #4: eg    #5: eh    #6: ei
  #7: fg    #8: fh    #9: fi

  Double-POP occurred at:
  - Steps 7-8 (after "di", transitioning 'd'→'e')
  - Steps 15-16 (after "ei", transitioning 'e'→'f')
  - Steps 23-24 (after "fi", termination)
```

---

## BLOCK 9: When to OUTPUT?

```
Input: "23" → len = 2

OUTPUT when stack.len() = input.len()

stack = ['d']     → len = 1 ≠ 2 → NOT OUTPUT
stack = ['d','g'] → len = 2 = 2 → OUTPUT "dg"
stack = ['d','h'] → len = 2 = 2 → OUTPUT "dh"
stack = ['e']     → len = 1 ≠ 2 → NOT OUTPUT
stack = ['e','g'] → len = 2 = 2 → OUTPUT "eg"
```

---

## BLOCK 10: When to POP Once?

```
Scenario: just OUTPUT "dg", want "dh"

stack = ['d','g']
        
'g' = mappings[3][0]
next = mappings[3][1] = 'h'
1 < 3 (more letters available)
∴ POP 'g', PUSH 'h'

Result: stack = ['d','h'] → OUTPUT "dh"
```

---

## BLOCK 11: When to POP Twice?

```
Scenario: just OUTPUT "di", want "eg"

stack = ['d','i']

'i' = mappings[3][2]
next index = 3
3 ≥ 3 (no more letters in mappings[3])
∴ POP 'i'

stack = ['d']

'd' = mappings[2][0]
next index = 1
1 < 3 (more letters available)
∴ POP 'd', PUSH 'e'

stack = ['e']

Now PUSH 'g':
stack = ['e','g'] → OUTPUT "eg"
```

**KEY INSIGHT:**
```
After POPping inner level:
  Check if outer level has more siblings
  If no more children AND no more siblings → POP again
  If siblings available → POP parent, PUSH next sibling
```

---

## BLOCK 12: The Tricky Case - Three POPs

```
Input: "123"
mappings[1] = ['a','b','c']
mappings[2] = ['d','e','f']
mappings[3] = ['g','h','i']

Total = 27

Last output with 'a' as first char:
  stack = ['a','f','i'] → OUTPUT "afi"

Next output:
  stack = ['b','d','g'] → OUTPUT "bdg"

Transition from "afi" to "bdg":
```

```
stack = ['a','f','i']

'i' = mappings[3][2] → index 2
next = 3
3 ≥ 3 → exhausted
∴ POP 'i'

stack = ['a','f']

'f' = mappings[2][2] → index 2
next = 3
3 ≥ 3 → exhausted
∴ POP 'f'

stack = ['a']

'a' = mappings[1][0] → index 0
next = 1
1 < 3 → siblings available
∴ POP 'a', PUSH 'b'

stack = ['b']

PUSH 'd' (first of mappings[2]):
stack = ['b','d']

PUSH 'g' (first of mappings[3]):
stack = ['b','d','g'] → OUTPUT "bdg"

Total POPs in this transition: 3 (i, f, a)
Total PUSHes in this transition: 3 (b, d, g)
```

---

## BLOCK 13: Variable Length Case - Digit 9

```
Input: "19"
mappings[1] = ['a','b','c'] → len 3
mappings[9] = ['y','z']     → len 2

Total = 3 × 2 = 6
```

```
ALL OUTPUTS:
stack = ['a','y'] → "ay" → OUTPUT #1
stack = ['a','z'] → "az" → OUTPUT #2
stack = ['b','y'] → "by" → OUTPUT #3
stack = ['b','z'] → "bz" → OUTPUT #4
stack = ['c','y'] → "cy" → OUTPUT #5
stack = ['c','z'] → "cz" → OUTPUT #6
```

**TRAP IF YOU HARDCODE 3:**
```
for j in 0..3:
  push mappings[9][j]

j=0: mappings[9][0] = 'y' ✓
j=1: mappings[9][1] = 'z' ✓
j=2: mappings[9][2] = ??? CRASH
```

**CORRECT:**
```
for j in 0..mappings[9].len():
  push mappings[9][j]

mappings[9].len() = 2
j=0: 'y' ✓
j=1: 'z' ✓
j=2: 2 ≥ 2 → STOP
```

---

## BLOCK 14: Full Trace for "19"

```
stack = []
idx = 0, digit = '1', mappings[1] = ['a','b','c']

PUSH 'a':
  ┌───┐
  │ a │
  └───┘
  idx = 1, digit = '9', mappings[9] = ['y','z']

PUSH 'y':
  ┌───┐
  │ y │
  ├───┤
  │ a │
  └───┘
  len = 2 = input.len() = 2 → OUTPUT "ay"

POP 'y':
  ┌───┐
  │ a │
  └───┘
  
PUSH 'z':
  ┌───┐
  │ z │
  ├───┤
  │ a │
  └───┘
  len = 2 → OUTPUT "az"

POP 'z':
  ┌───┐
  │ a │
  └───┘
  'z' was mappings[9][1]
  next = 2
  2 ≥ 2 → mappings[9] exhausted

POP 'a':
  └───┘
  'a' was mappings[1][0]
  next = 1
  1 < 3 → mappings[1] NOT exhausted
  
PUSH 'b':
  ┌───┐
  │ b │
  └───┘

PUSH 'y':
  ┌───┐
  │ y │
  ├───┤
  │ b │
  └───┘
  len = 2 → OUTPUT "by"

POP 'y':
  ┌───┐
  │ b │
  └───┘

PUSH 'z':
  ┌───┐
  │ z │
  ├───┤
  │ b │
  └───┘
  len = 2 → OUTPUT "bz"

POP 'z':
  ┌───┐
  │ b │
  └───┘
  mappings[9] exhausted

POP 'b':
  └───┘
  'b' was mappings[1][1]
  next = 2
  2 < 3 → mappings[1] NOT exhausted

PUSH 'c':
  ┌───┐
  │ c │
  └───┘

PUSH 'y':
  ┌───┐
  │ y │
  ├───┤
  │ c │
  └───┘
  len = 2 → OUTPUT "cy"

POP 'y':
  ┌───┐
  │ c │
  └───┘

PUSH 'z':
  ┌───┐
  │ z │
  ├───┤
  │ c │
  └───┘
  len = 2 → OUTPUT "cz"

POP 'z':
  ┌───┐
  │ c │
  └───┘
  mappings[9] exhausted

POP 'c':
  └───┘
  'c' was mappings[1][2]
  next = 3
  3 ≥ 3 → mappings[1] exhausted
  
stack = [] AND idx = 0 exhausted → DONE

OUTPUTS: ay, az, by, bz, cy, cz = 6 ✓
```

---

## BLOCK 15: What You Must Track

At each step, you need:

```
1. stack contents        → current partial string
2. stack.len()           → compare to input.len() for OUTPUT decision
3. for each char in stack:
   - which mappings[] it came from
   - which index within that mappings[]
   - what is next index
   - is next index < mappings[].len() ?
```

**EXAMPLE:**
```
stack = ['d','h']
        
'd' from mappings[2], index 0, next = 1, 1 < 3 ✓
'h' from mappings[3], index 1, next = 2, 2 < 3 ✓

After OUTPUT "dh":
  POP 'h'
  'h' was index 1, next = 2, 2 < 3 → PUSH mappings[3][2] = 'i'
```

---

## BLOCK 16: COMPLETE RECURSION TRACE WITH NAMED VARIABLES

```
═══════════════════════════════════════════════════════════════════════════════
VARIABLE DEFINITIONS
═══════════════════════════════════════════════════════════════════════════════

phone_number: &str
  - the input string user typed
  - example: "19"
  - phone_number.len() = 2
  - phone_number.chars().nth(0) = Some('1')
  - phone_number.chars().nth(1) = Some('9')

mappings: &[Vec<char>]
  - 2D array, mappings[digit] = letters for that digit
  - mappings[0] = [] (unused, placeholder)
  - mappings[1] = ['a','b','c'] at address 0x1000, len = 3
  - mappings[2] = ['d','e','f'] at address 0x1010, len = 3
  - ...
  - mappings[9] = ['y','z'] at address 0x1080, len = 2 ← ONLY 2!

digit_position: usize
  - which digit of phone_number we are currently processing
  - ranges from 0 to phone_number.len()
  - digit_position = 0 means processing phone_number[0] = '1'
  - digit_position = 1 means processing phone_number[1] = '9'
  - digit_position = 2 means we've processed all digits → BASE CASE

current_buffer: &mut String
  - the partial combination being built
  - starts as "" (empty)
  - grows as we go deeper: "" → "a" → "ay"
  - shrinks as we backtrack: "ay" → "a" → ""

results: &mut Vec<String>
  - accumulator for all complete combinations
  - starts as []
  - after completion: ["ay","az","by","bz","cy","cz"]

letter_index: usize (implicit in for loop)
  - which letter within current digit's mapping we are on
  - for digit '1': letter_index ∈ {0,1,2} → 'a','b','c'
  - for digit '9': letter_index ∈ {0,1} → 'y','z'

═══════════════════════════════════════════════════════════════════════════════
FUNCTION SIGNATURE
═══════════════════════════════════════════════════════════════════════════════

fn generate(
    phone_number: &str,           // "19"
    mappings: &[Vec<char>],       // the 2D array
    digit_position: usize,        // 0..=phone_number.len()
    current_buffer: &mut String,  // partial result being built
    results: &mut Vec<String>     // accumulator
)

═══════════════════════════════════════════════════════════════════════════════
INPUT: "19"
TOTAL OUTPUTS: 3 × 2 = 6
═══════════════════════════════════════════════════════════════════════════════

CALL #0: generate("19", mappings, 0, "", [])
│
│ digit_position = 0
│ phone_number.len() = 2
│ 0 < 2 ✓ → NOT base case
│
│ current_digit_char = phone_number.chars().nth(0) = Some('1')
│ current_digit_char.unwrap() = '1'
│ '1' as u8 = 49
│ '0' as u8 = 48
│ digit_as_index = 49 - 48 = 1
│ current_letters = mappings[1] = ['a','b','c']
│ current_letters.len() = 3
│
│ for letter_index in 0..3:
│
│ ═══ letter_index = 0 ═══════════════════════════════════════════════════════
│ letter = current_letters[0] = 'a' (ASCII 97)
│ current_buffer.push('a')
│ current_buffer = "" + "a" = "a"
│ current_buffer.len() = 1
│
├──→ CALL #1: generate("19", mappings, 1, "a", [])
│    │
│    │ digit_position = 1
│    │ phone_number.len() = 2
│    │ 1 < 2 ✓ → NOT base case
│    │
│    │ current_digit_char = phone_number.chars().nth(1) = Some('9')
│    │ '9' as u8 = 57
│    │ digit_as_index = 57 - 48 = 9
│    │ current_letters = mappings[9] = ['y','z']
│    │ current_letters.len() = 2 ← NOT 3!
│    │
│    │ for letter_index in 0..2:  ← loop bound is 2, not 3!
│    │
│    │ ═══ letter_index = 0 ══════════════════════════════════════════════════
│    │ letter = current_letters[0] = 'y' (ASCII 121)
│    │ current_buffer.push('y')
│    │ current_buffer = "a" + "y" = "ay"
│    │ current_buffer.len() = 2
│    │
│    ├──→ CALL #2: generate("19", mappings, 2, "ay", [])
│    │    │
│    │    │ digit_position = 2
│    │    │ phone_number.len() = 2
│    │    │ 2 == 2 ✓ → BASE CASE!
│    │    │
│    │    │ results.push(current_buffer.clone())
│    │    │ results.push("ay".to_string())
│    │    │ results = ["ay"]
│    │    │ results.len() = 1
│    │    │
│    │    │ return ← POP call frame
│    │    │
│    │←── RETURN from CALL #2
│    │
│    │ current_buffer.pop() → removes 'y'
│    │ current_buffer = "ay" - "y" = "a"
│    │
│    │ ═══ letter_index = 1 ══════════════════════════════════════════════════
│    │ letter = current_letters[1] = 'z' (ASCII 122)
│    │ current_buffer.push('z')
│    │ current_buffer = "a" + "z" = "az"
│    │
│    ├──→ CALL #3: generate("19", mappings, 2, "az", ["ay"])
│    │    │
│    │    │ digit_position = 2 == 2 → BASE CASE
│    │    │ results.push("az")
│    │    │ results = ["ay", "az"]
│    │    │ results.len() = 2
│    │    │ return
│    │    │
│    │←── RETURN from CALL #3
│    │
│    │ current_buffer.pop() → removes 'z'
│    │ current_buffer = "az" - "z" = "a"
│    │
│    │ letter_index = 2
│    │ 2 >= current_letters.len() = 2 ✓ → loop ends
│    │
│    │ return ← POP call frame for CALL #1
│    │
│←── RETURN from CALL #1
│
│ current_buffer.pop() → removes 'a'
│ current_buffer = "a" - "a" = ""
│
│ ═══ letter_index = 1 ═══════════════════════════════════════════════════════
│ letter = current_letters[1] = 'b' (ASCII 98)
│ current_buffer.push('b')
│ current_buffer = "" + "b" = "b"
│
├──→ CALL #4: generate("19", mappings, 1, "b", ["ay","az"])
│    │
│    │ digit_position = 1, 1 < 2 → NOT base case
│    │ digit_as_index = 9
│    │ current_letters = mappings[9] = ['y','z']
│    │
│    │ ═══ letter_index = 0 ══════════════════════════════════════════════════
│    │ current_buffer = "b" + "y" = "by"
│    │
│    ├──→ CALL #5: generate("19", mappings, 2, "by", ["ay","az"])
│    │    │ BASE CASE → results.push("by")
│    │    │ results = ["ay","az","by"]
│    │    │ return
│    │←── RETURN
│    │
│    │ current_buffer.pop() → "by" - "y" = "b"
│    │
│    │ ═══ letter_index = 1 ══════════════════════════════════════════════════
│    │ current_buffer = "b" + "z" = "bz"
│    │
│    ├──→ CALL #6: generate("19", mappings, 2, "bz", ["ay","az","by"])
│    │    │ BASE CASE → results.push("bz")
│    │    │ results = ["ay","az","by","bz"]
│    │    │ return
│    │←── RETURN
│    │
│    │ current_buffer.pop() → "bz" - "z" = "b"
│    │ loop ends, return
│    │
│←── RETURN from CALL #4
│
│ current_buffer.pop() → "b" - "b" = ""
│
│ ═══ letter_index = 2 ═══════════════════════════════════════════════════════
│ letter = current_letters[2] = 'c' (ASCII 99)
│ current_buffer.push('c')
│ current_buffer = "" + "c" = "c"
│
├──→ CALL #7: generate("19", mappings, 1, "c", ["ay","az","by","bz"])
│    │
│    │ ═══ letter_index = 0 ══════════════════════════════════════════════════
│    │ current_buffer = "c" + "y" = "cy"
│    │
│    ├──→ CALL #8: generate(..., 2, "cy", [...])
│    │    │ BASE CASE → results.push("cy")
│    │    │ results = ["ay","az","by","bz","cy"]
│    │←── RETURN
│    │
│    │ current_buffer.pop() → "cy" - "y" = "c"
│    │
│    │ ═══ letter_index = 1 ══════════════════════════════════════════════════
│    │ current_buffer = "c" + "z" = "cz"
│    │
│    ├──→ CALL #9: generate(..., 2, "cz", [...])
│    │    │ BASE CASE → results.push("cz")
│    │    │ results = ["ay","az","by","bz","cy","cz"]
│    │    │ results.len() = 6 ✓
│    │←── RETURN
│    │
│    │ current_buffer.pop() → "cz" - "z" = "c"
│    │ loop ends, return
│    │
│←── RETURN from CALL #7
│
│ current_buffer.pop() → "c" - "c" = ""
│
│ letter_index = 3
│ 3 >= current_letters.len() = 3 ✓ → loop ends
│
│ return from CALL #0
│
└── DONE

═══════════════════════════════════════════════════════════════════════════════
FINAL STATE
═══════════════════════════════════════════════════════════════════════════════

results = ["ay", "az", "by", "bz", "cy", "cz"]
results.len() = 6 = 3 × 2 ✓

current_buffer = "" (restored to initial state)

Total CALL count: 10 (CALL #0 through #9)
  - 1 initial call
  - 3 calls at digit_position=1 (one per letter in mappings[1])
  - 6 calls at digit_position=2 (one per output)

Total push() operations: 6 + 6 = 12
  - 6 pushes of first-level letters (a,a,b,b,c,c... wait no)
  
RECALCULATE:
  push 'a' → push 'y' → pop 'y' → push 'z' → pop 'z' → pop 'a'
  push 'b' → push 'y' → pop 'y' → push 'z' → pop 'z' → pop 'b'
  push 'c' → push 'y' → pop 'y' → push 'z' → pop 'z' → pop 'c'

  push count: 3 (first level) + 6 (second level) = 9
  pop count:  3 (first level) + 6 (second level) = 9 ✓ (balanced)

═══════════════════════════════════════════════════════════════════════════════
CALL STACK VISUALIZATION AT MAXIMUM DEPTH
═══════════════════════════════════════════════════════════════════════════════

At CALL #2 (when outputting "ay"):

┌─────────────────────────────────────────────────────────────────────────────┐
│ CALL #2                                                                     │
│   digit_position = 2                                                        │
│   current_buffer = "ay" (points to same String, len=2)                      │
│   return address = CALL #1, line after recursive call                       │
├─────────────────────────────────────────────────────────────────────────────┤
│ CALL #1                                                                     │
│   digit_position = 1                                                        │
│   letter_index = 0                                                          │
│   current_letters = &mappings[9] = &['y','z']                               │
│   current_buffer = "ay" (same String)                                       │
│   return address = CALL #0, line after recursive call                       │
├─────────────────────────────────────────────────────────────────────────────┤
│ CALL #0                                                                     │
│   digit_position = 0                                                        │
│   letter_index = 0                                                          │
│   current_letters = &mappings[1] = &['a','b','c']                           │
│   current_buffer = "ay" (same String)                                       │
│   return address = main(), line after generate() call                       │
└─────────────────────────────────────────────────────────────────────────────┘

Stack depth = 3 = phone_number.len() + 1

Memory used by call stack ≈ 3 frames × ~100 bytes/frame ≈ 300 bytes
```

---

## YOUR EXERCISE

Input: "99"

```
mappings[9] = ['y','z'] → len = 2

Total outputs = 2 × 2 = 4

Complete the trace:

CALL #0: generate("99", mappings, 0, "", [])
│ digit_position = 0
│ digit_as_index = '9' - '0' = ___
│ current_letters = mappings[___] = [___,___]
│ current_letters.len() = ___
│
│ ═══ letter_index = 0 ═══
│ current_buffer = "" + ___ = "___"
│
├──→ CALL #1: generate("99", mappings, 1, "___", [])
│    │ digit_position = 1
│    │ current_letters = mappings[9] = ['y','z']
│    │
│    │ ═══ letter_index = 0 ═══
│    │ current_buffer = "___" + "___" = "___"
│    │
│    ├──→ CALL #2: generate(..., 2, "___", [])
│    │    │ BASE CASE → results.push("___")
│    │    │ results = ["___"]
│    │←── RETURN
│    │
│    │ current_buffer.pop() → "___"
│    │
│    │ ═══ letter_index = 1 ═══
│    │ current_buffer = "___" + "___" = "___"
│    │
│    ├──→ CALL #3: ...
│    │    │ results.push("___")
│    │    │ results = ["___", "___"]
│    │←── RETURN
│    │
│    │ current_buffer.pop() → "___"
│    │ loop ends, return
│    │
│←── RETURN from CALL #1
│
│ current_buffer.pop() → "___"
│
│ ═══ letter_index = 1 ═══
│ ... continue the trace ...

FINAL results = ["___", "___", "___", "___"]
Total CALL count = ___
```

---

## BLOCK 17: WHY POP? SIDE-BY-SIDE COMPARISON

```
═══════════════════════════════════════════════════════════════════════════════
INPUT: "19"
EXPECTED OUTPUT: ["ay", "az", "by", "bz", "cy", "cz"] (6 strings)
═══════════════════════════════════════════════════════════════════════════════

CODE WITH POP:                          CODE WITHOUT POP:
──────────────────────                   ──────────────────────
for l in letters {                       for l in letters {
    buf.push(*l);                            buf.push(*l);
    recurse();                               recurse();
    buf.pop();    ← THIS LINE                // no pop
}                                        }
```

```
═══════════════════════════════════════════════════════════════════════════════
TRACE WITH POP (CORRECT):
═══════════════════════════════════════════════════════════════════════════════

OUTER LOOP: letters = ['a','b','c']

ITER 1: l = 'a'
  ┌─────────────────────────────────────────────────────────────────────────┐
  │ buf.push('a')                                                           │
  │                                                                         │
  │ buf BEFORE: │       │  len=0                                            │
  │             └───────┘                                                   │
  │                                                                         │
  │ buf AFTER:  │   a   │  len=1                                            │
  │             └───────┘                                                   │
  └─────────────────────────────────────────────────────────────────────────┘
  
  RECURSE → digit_position=1, letters = ['y','z']
  
    INNER ITER 1: l = 'y'
      ┌───────────────────────────────────────────────────────────────────┐
      │ buf.push('y')                                                     │
      │                                                                   │
      │ buf BEFORE: │   a   │  len=1                                      │
      │             └───────┘                                             │
      │                                                                   │
      │ buf AFTER:  │   a   │   y   │  len=2                              │
      │             └───────┴───────┘                                     │
      │                                                                   │
      │ OUTPUT: "ay" ✓                                                    │
      └───────────────────────────────────────────────────────────────────┘
      
      ┌───────────────────────────────────────────────────────────────────┐
      │ buf.pop()                                                         │
      │                                                                   │
      │ buf BEFORE: │   a   │   y   │  len=2                              │
      │             └───────┴───────┘                                     │
      │                                                                   │
      │ buf AFTER:  │   a   │       │  len=1  ← 'y' REMOVED               │
      │             └───────┴───────┘                                     │
      └───────────────────────────────────────────────────────────────────┘
    
    INNER ITER 2: l = 'z'
      ┌───────────────────────────────────────────────────────────────────┐
      │ buf.push('z')                                                     │
      │                                                                   │
      │ buf BEFORE: │   a   │  len=1                                      │
      │             └───────┘                                             │
      │                                                                   │
      │ buf AFTER:  │   a   │   z   │  len=2                              │
      │             └───────┴───────┘                                     │
      │                                                                   │
      │ OUTPUT: "az" ✓                                                    │
      └───────────────────────────────────────────────────────────────────┘
      
      buf.pop() → buf = "a"
      
  RETURN TO OUTER LOOP
  
  ┌─────────────────────────────────────────────────────────────────────────┐
  │ buf.pop() ← OUTER POP                                                   │
  │                                                                         │
  │ buf BEFORE: │   a   │  len=1                                            │
  │             └───────┘                                                   │
  │                                                                         │
  │ buf AFTER:  │       │  len=0  ← 'a' REMOVED, READY FOR 'b'              │
  │             └───────┘                                                   │
  └─────────────────────────────────────────────────────────────────────────┘

ITER 2: l = 'b'
  ┌─────────────────────────────────────────────────────────────────────────┐
  │ buf.push('b')                                                           │
  │                                                                         │
  │ buf BEFORE: │       │  len=0  ← CLEAN!                                  │
  │             └───────┘                                                   │
  │                                                                         │
  │ buf AFTER:  │   b   │  len=1  ← CORRECT: just 'b'                       │
  │             └───────┘                                                   │
  └─────────────────────────────────────────────────────────────────────────┘
  
  ... produces "by", "bz" ...
```

```
═══════════════════════════════════════════════════════════════════════════════
TRACE WITHOUT POP (WRONG):
═══════════════════════════════════════════════════════════════════════════════

OUTER LOOP: letters = ['a','b','c']

ITER 1: l = 'a'
  buf.push('a') → buf = "a"
  
  RECURSE → digit_position=1
  
    INNER ITER 1: l = 'y'
      buf.push('y') → buf = "ay"
      OUTPUT: "ay" ✓
      NO POP ← MISSING!
    
    INNER ITER 2: l = 'z'
      ┌───────────────────────────────────────────────────────────────────┐
      │ buf.push('z')                                                     │
      │                                                                   │
      │ buf BEFORE: │   a   │   y   │  len=2  ← 'y' STILL HERE!           │
      │             └───────┴───────┘                                     │
      │                                                                   │
      │ buf AFTER:  │   a   │   y   │   z   │  len=3                      │
      │             └───────┴───────┴───────┘                             │
      │                                                                   │
      │ OUTPUT: "ayz" ← WRONG! EXPECTED "az"                              │
      └───────────────────────────────────────────────────────────────────┘
      
  RETURN TO OUTER LOOP
  NO OUTER POP ← MISSING!

ITER 2: l = 'b'
  ┌─────────────────────────────────────────────────────────────────────────┐
  │ buf.push('b')                                                           │
  │                                                                         │
  │ buf BEFORE: │   a   │   y   │   z   │  len=3  ← GARBAGE!                │
  │             └───────┴───────┴───────┘                                   │
  │                                                                         │
  │ buf AFTER:  │   a   │   y   │   z   │   b   │  len=4                    │
  │             └───────┴───────┴───────┴───────┘                           │
  │                                                                         │
  │ ... produces "ayzby", "ayzbyz", etc ← ALL GARBAGE!                      │
  └─────────────────────────────────────────────────────────────────────────┘
```

```
═══════════════════════════════════════════════════════════════════════════════
SUMMARY:
═══════════════════════════════════════════════════════════════════════════════

WITH POP:                               WITHOUT POP:
─────────                               ───────────
buf = ""                                buf = ""
buf = "a"   ← push                      buf = "a"   ← push
buf = "ay"  ← push                      buf = "ay"  ← push
OUTPUT "ay"                             OUTPUT "ay"
buf = "a"   ← pop 'y'                   buf = "ay"  ← NO POP, 'y' sticks
buf = "az"  ← push 'z'                  buf = "ayz" ← push 'z' on top of 'y'
OUTPUT "az" ✓                           OUTPUT "ayz" ✗
buf = "a"   ← pop 'z'                   buf = "ayz"
buf = ""    ← pop 'a'                   buf = "ayz"
buf = "b"   ← push 'b'                  buf = "ayzb" ← GARBAGE
OUTPUT "by", "bz" ✓                     OUTPUT GARBAGE

═══════════════════════════════════════════════════════════════════════════════
POP = UNDO THE PUSH
    = REMOVE THE LETTER WE JUST TRIED
    = SO NEXT ITERATION STARTS FROM SAME POSITION
═══════════════════════════════════════════════════════════════════════════════
```

---

## YOUR MISTAKES LOG

```
1. for letters in letter
   ↓
   SWAPPED variable names
   letters = collection (line 28)
   letter = loop variable (should be)
   YOU WROTE: for letters in letter ← BACKWARDS

2. current_buffer.push(*letters)
   ↓
   NO SEMICOLON
   RUST REQUIRES ; AFTER STATEMENTS

3. current_buffer.pop()
   ↓
   NO SEMICOLON

4. for l in letter
   ↓
   STILL WRONG: should be "letters" not "letter"
   letter does not exist, letters was defined

5. PYTHON SYNTAX: for x in y:
   ↓
   RUST SYNTAX: for x in y { }
   USED COLON INSTEAD OF BRACES
```
# ROAST OF YOUR ATTEMPTS

---

## ERROR #1: Your "Tree" is NOT a Tree

**YOU WROTE:**
```
d e f 
g h i 
```
**YOU COUNTED:** 6 leaves

---

**WHY THIS IS WRONG:**

You listed 6 letters. You did NOT draw connections. A tree has EDGES.

**CORRECT TREE FOR "23":**

```
                    "" (ROOT)
                    │
       ┌────────────┼────────────┐
       │            │            │
       d            e            f        ← Level 1 (3 nodes)
       │            │            │
   ┌───┼───┐    ┌───┼───┐    ┌───┼───┐
   │   │   │    │   │   │    │   │   │
  dg  dh  di   eg  eh  ei   fg  fh  fi   ← Level 2 (9 LEAVES)
```

**WHY 9 NOT 6:**
- 'd' branches to 3 children → dg, dh, di
- 'e' branches to 3 children → eg, eh, ei
- 'f' branches to 3 children → fg, fh, fi
- Total leaves = 3 + 3 + 3 = 9

You saw 3 letters d,e,f and 3 letters g,h,i, you added 3+3=6.
The operation is MULTIPLY not ADD.
Each 'd' COMBINES with EACH of g,h,i → 1 × 3 = 3 combinations from 'd' alone.
Three first-level letters × 3 combinations each = 3 × 3 = 9.

---

## ERROR #2: Nonsense Answer to Before/After Question

**QUESTION:** Output "eg" comes before or after "dh"?
**YOU WROTE:** "yes"

---

**WHY THIS IS NONSENSE:**

The question is "A or B?" not "yes or no?"
- "before" is one answer
- "after" is another answer
- "yes" answers nothing

**CORRECT ANALYSIS:**

```
Output sequence for "23":
  Position 1: dg
  Position 2: dh  ← "dh" is here
  Position 3: di
  Position 4: eg  ← "eg" is here
  Position 5: eh
  Position 6: ei
  Position 7: fg
  Position 8: fh
  Position 9: fi

"dh" is at position 2.
"eg" is at position 4.
4 > 2.
∴ "eg" comes AFTER "dh".
```

You listed the correct sequence "dg dh di eg eh ei fg fh fi" but did not READ it.
You did not LOCATE "dh" (position 2) and "eg" (position 4).
You typed the answer without COMPUTING anything.

---

## ERROR #3: Wrong Order of Operations

**QUESTION:** To go from "di" to "eg", what operations in what order?
**YOU WROTE:** "remove d add e remove i add g"

---

**WHY THIS IS IMPOSSIBLE:**

Current string: "di"
```
Position 0: 'd'
Position 1: 'i'
```

Your first operation: "remove d"

**PROBLEM:** You CANNOT remove 'd' first!

```
String "di" stored as:
┌───┬───┐
│ d │ i │
└───┴───┘
  0   1

To remove 'd' (position 0), you would need to shift 'i' left.
But that's expensive and NOT what happens in this algorithm.

What ACTUALLY happens:
  "di" → remove LAST char → "d"
  "d"  → remove LAST char → ""
  ""   → add 'e'          → "e"
  "e"  → add 'g'          → "eg"

Order: POP 'i', POP 'd', PUSH 'e', PUSH 'g'
```

**WHY POP always removes LAST:**

```
String as STACK:
      ┌───┐
      │ i │  ← TOP (last added)
      ├───┤
      │ d │  ← BOTTOM (first added)
      └───┘

POP removes from TOP → 'i' comes out first
POP again → 'd' comes out

PUSH 'e' → 'e' goes to TOP
PUSH 'g' → 'g' goes to TOP

Result:
      ┌───┐
      │ g │  ← TOP
      ├───┤
      │ e │  ← BOTTOM
      └───┘

String = "eg" ✓
```

Your answer "remove d add e remove i add g" is physically impossible.
You cannot remove the bottom of a stack while there's something on top.

---

## ERROR #4: Step 4 Missing Full String

**YOU WROTE:** Step 4: pick 'h', string = h

---

**WHY THIS IS WRONG:**

At Step 3, you needed to show the OPERATION.
At Step 4, string is NOT just "h".

**CORRECT TRACE:**

```
Step 0: string = ""
Step 1: PUSH 'd', string = "d"
Step 2: PUSH 'g', string = "dg"  → OUTPUT "dg"
Step 3: POP 'g',  string = "d"   ← YOU SKIPPED THIS
Step 4: PUSH 'h', string = "dh"  → OUTPUT "dh" ← NOT just "h"
Step 5: POP 'h',  string = "d"   ← YOU SKIPPED THIS
Step 6: PUSH 'i', string = "di"  → OUTPUT "di"
Step 7: POP 'i',  string = "d"   ← YOU SKIPPED THIS
Step 8: POP 'd',  string = ""    ← YOU SKIPPED THIS
Step 9: PUSH 'e', string = "e"
Step 10: PUSH 'g', string = "eg" → OUTPUT "eg"
...
```

You wrote "h" for step 4 string, but string is "dh".
The 'd' was pushed at Step 1 and NEVER POPPED until Step 8.
'd' stays in string for outputs "dg", "dh", "di".

---

## ERROR #5: Step 3, 5, 7 Left Blank

**YOU LEFT BLANK:** Steps 3, 5, 7, 8

---

**WHY THIS IS THE MEAT:**

Steps 3, 5, 7 are where the POP happens.
The POP is THE TRICKY PART of this problem.
You skipped EXACTLY the tricky steps.

```
Without POP:
  Step 1: string = "d"
  Step 2: string = "dg" → OUTPUT
  Step 3: ???
  Step 4: string = "dgh"  ← WRONG! You ADDED 'h' to "dg"

With POP:
  Step 1: string = "d"
  Step 2: string = "dg" → OUTPUT
  Step 3: POP → string = "d"
  Step 4: string = "dh" → OUTPUT ← CORRECT!
```

The difference between wrong output "dgh" and correct output "dh" is the POP at Step 3.
You left Step 3 blank.
This is the pattern: you skip what you don't understand, then wonder why code fails.

---

## ERROR #6: "dg" is Not a Letter

**QUESTION:** First output will start with which LETTER?
**YOU WROTE:** "dg"

---

**WHY THIS IS WRONG:**

"dg" is NOT a letter. "dg" is a STRING.
'd' is a letter.
'g' is a letter.

The question asked: "which LETTER?"
The answer is: 'd'

The first output "dg" starts with the letter 'd'.

You typed the whole output instead of answering the actual question.
This is: reading question quickly → typing response quickly → wrong answer.

---

## SUMMARY OF YOUR PATTERNS

| Error | Pattern |
|-------|---------|
| 6 leaves instead of 9 | ADD instead of MULTIPLY |
| "yes" to "before or after" | Not reading question |
| "remove d" first when string is "di" | Not visualizing data structure |
| Step 4 = "h" instead of "dh" | Forgetting state persists |
| Steps 3,5,7 blank | Skipping the tricky steps |
| "dg" instead of "d" | Not answering what was asked |

---

## YOUR TASK: REDO

Go back to the markdown. Fix:

1. Draw the TREE with edges, not just letters
2. Answer "before" or "after", not "yes"
3. Write POP/PUSH sequence from "di" to "eg" in correct order
4. Fill in Steps 3, 5, 7 with the POP operation
5. Fix Step 4 to show the full string "dh"

Do not proceed until these are correct.

