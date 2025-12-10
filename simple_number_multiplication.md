# Simple Number Multiplication: O(n) Puzzle

## Problem Extraction

```
INPUT: two simple numbers, each n digits
SIMPLE NUMBER: single digit repeated n times
EXAMPLES: 4444 (digit=4, n=4), 6666 (digit=6, n=4), 77 (digit=7, n=2)
CONSTRAINT: n = power of 2 (n ∈ {1, 2, 4, 8, 16, 32, ...})
OUTPUT: product of the two simple numbers
TIME: O(n) where n = digit count
```

---

## Numerical Example 1: Breaking Down Simple Numbers

```
4444 = ?

4444 = 4000 + 400 + 40 + 4
     = 4×1000 + 4×100 + 4×10 + 4×1
     = 4×(1000 + 100 + 10 + 1)
     = 4×1111

∴ 4444 = 4 × 1111
```

```
WHY 1111?
1111 = 1000 + 100 + 10 + 1
     = 10³ + 10² + 10¹ + 10⁰
     = (10⁴ - 1) / (10 - 1)
     = (10000 - 1) / 9
     = 9999 / 9
     = 1111 ✓

∴ For n=4 digits: REPUNIT = (10⁴ - 1) / 9 = 1111
```

---

## Numerical Example 2: General Pattern Discovery

```
n=1: REPUNIT = (10¹ - 1)/9 = 9/9 = 1
n=2: REPUNIT = (10² - 1)/9 = 99/9 = 11
n=4: REPUNIT = (10⁴ - 1)/9 = 9999/9 = 1111
n=8: REPUNIT = (10⁸ - 1)/9 = 99999999/9 = 11111111
```

```
SIMPLE NUMBER = DIGIT × REPUNIT

3333 = 3 × 1111
7777 = 7 × 1111
22 = 2 × 11
88888888 = 8 × 11111111
```

---

## Numerical Example 3: The Multiplication You Will Skip

```
PROBLEM: 4444 × 6666 = ?

NAIVE APPROACH (what you will try first):
  4444
× 6666
------
This is O(n²) → 16 single-digit multiplications for n=4

BUT the puzzle asks for O(n) → something is HIDDEN
```

```
REWRITE:
4444 × 6666 = (4 × 1111) × (6 × 1111)
            = (4 × 6) × (1111 × 1111)
            = 24 × (1111)²

∴ The core problem becomes: compute (REPUNIT)² efficiently
```

---

## Numerical Example 4: REPUNIT² — The Hidden Monster

```
1111² = ?

BRUTE FORCE:
    1111
  × 1111
  ------
    1111      (1111 × 1)
   1111       (1111 × 10)
  1111        (1111 × 100)
 1111         (1111 × 1000)
------------
  1234321

WAIT → WHAT IS THE PATTERN IN 1234321?
```

```
11² = 121
111² = 12321
1111² = 1234321
11111² = 123454321

PATTERN: digits go 1,2,3,...,n,...,3,2,1
PEAK = n (the number of 1s)
```

---

## Numerical Example 5: Discovering The Structure of REPUNIT²

```
n=2: 11² = 121
     ↓
     1 → position 2 (10²)
     2 → position 1 (10¹)
     1 → position 0 (10⁰)
     = 100 + 20 + 1 = 121 ✓

n=4: 1111² = 1234321
     ↓
     1 → position 6 (10⁶) = 1000000
     2 → position 5 (10⁵) = 200000
     3 → position 4 (10⁴) = 30000
     4 → position 3 (10³) = 4000
     3 → position 2 (10²) = 300
     2 → position 1 (10¹) = 20
     1 → position 0 (10⁰) = 1
     -------------------------
     TOTAL = 1234321 ✓
```

---

## Numerical Example 6: THE TRAP — When n Gets Large

```
n=8: 11111111² = ?

EXPECTED PATTERN: 1234567887654321 ???

WAIT — WHAT HAPPENS AT POSITION 8?
EXPECTED DIGIT = 8
BUT 8 IS SINGLE DIGIT ✓

n=16: 1111111111111111² = ?

EXPECTED PEAK VALUE = 16
BUT 16 IS TWO DIGITS → CARRY OCCURS!

POSITION ANALYSIS:
...13, 14, 15, 16, 15, 14, 13...
         ↑
      CARRY!

16 at position p → write 6, carry 1 to position p+1
15 + 1 = 16 → write 6, carry 1
14 + 1 = 15 → write 5, carry 1
...

∴ THE SIMPLE PATTERN BREAKS FOR n ≥ 10
```

---

## Numerical Example 7: Carry Propagation — The True Difficulty

```
n=10: 1111111111² = ?

RAW COEFFICIENTS (before carry):
pos: 0  1  2  3  4  5  6  7  8  9  10 9  8  7  6  5  4  3  2  1  0
val: 1  2  3  4  5  6  7  8  9  10 9  8  7  6  5  4  3  2  1

WAIT — DOUBLE CHECK:
Length = 2n - 1 = 19 positions (0 to 18)

Position 0: coef = 1
Position 1: coef = 2
...
Position 9: coef = 10 (PEAK!)
Position 10: coef = 9
...
Position 18: coef = 1

CARRY PROCESSING (right to left):
pos 0: coef=1, carry_in=0 → digit=1, carry_out=0
pos 1: coef=2, carry_in=0 → digit=2, carry_out=0
...
pos 8: coef=9, carry_in=0 → digit=9, carry_out=0
pos 9: coef=10, carry_in=0 → digit=0, carry_out=1
pos 10: coef=9, carry_in=1 → total=10 → digit=0, carry_out=1
pos 11: coef=8, carry_in=1 → total=9 → digit=9, carry_out=0
...

∴ 1111111111² = 12345678900987654321
               NOT 123456789109876543211
```

---

## DATA STRUCTURE: What You Must Draw Before Coding

```
INPUT REPRESENTATION:

simple_num_1 = 4444
├── digit_1 = 4
└── n = 4

simple_num_2 = 6666
├── digit_2 = 6
└── n = 4

INTERMEDIATE:
product_of_digits = digit_1 × digit_2 = 4 × 6 = 24

REPUNIT_SQUARED coefficients (n=4):
┌─────────────────────────────────────────┐
│ pos:  0   1   2   3   4   5   6         │
│ val:  1   2   3   4   3   2   1         │
│       ↓   ↓   ↓   ↓   ↓   ↓   ↓         │
│      10⁰ 10¹ 10² 10³ 10⁴ 10⁵ 10⁶        │
└─────────────────────────────────────────┘
= 1 + 20 + 300 + 4000 + 30000 + 200000 + 1000000
= 1234321

FINAL MULTIPLICATION:
24 × 1234321 = ?

24 × 1234321:
  EACH COEFFICIENT × 24:
  pos 0: 1×24 = 24 → digit=4, carry=2
  pos 1: 2×24 + 2 = 50 → digit=0, carry=5
  pos 2: 3×24 + 5 = 77 → digit=7, carry=7
  pos 3: 4×24 + 7 = 103 → digit=3, carry=10
  pos 4: 3×24 + 10 = 82 → digit=2, carry=8
  pos 5: 2×24 + 8 = 56 → digit=6, carry=5
  pos 6: 1×24 + 5 = 29 → digit=9, carry=2
  pos 7: 0×24 + 2 = 2 → digit=2, carry=0

  RESULT: 29623704
  
VERIFICATION: 4444 × 6666 = 29623704 ✓
```

---

## BRUTE-FORCE DERIVATION: WHY COEFFICIENT = min(i+1, 2n-1-i)

```
SETUP: Multiplying REPUNIT × REPUNIT where REPUNIT has n ones

REPUNIT for n=4: 1111
  = 1×10³ + 1×10² + 1×10¹ + 1×10⁰
  = 1 at position 0
  = 1 at position 1
  = 1 at position 2
  = 1 at position 3

When we multiply 1111 × 1111, each position p in the result gets
contributions from (a,b) pairs where a+b=p, AND both a and b are
valid positions in the input (0 ≤ a ≤ 3, 0 ≤ b ≤ 3).

Since each input digit is 1, contribution = 1×1 = 1.
So coefficient[p] = COUNT of valid (a,b) pairs where a+b=p.
```

---

### EXHAUSTIVE ENUMERATION FOR n=4 (7 output positions: 0 to 6)

```
POSITION 0: need a+b=0, where 0≤a≤3, 0≤b≤3

  TRY a=0: b=0-0=0 → 0≤0≤3 ✓ → PAIR (0,0) VALID
  TRY a=1: b=0-1=-1 → -1<0 ✗
  TRY a=2: b=0-2=-2 → -2<0 ✗
  TRY a=3: b=0-3=-3 → -3<0 ✗

  VALID PAIRS: {(0,0)}
  COUNT = 1
  
  FORMULA CHECK: min(0+1, 2×4-1-0) = min(1, 7) = 1 ✓
```

```
POSITION 1: need a+b=1, where 0≤a≤3, 0≤b≤3

  TRY a=0: b=1-0=1 → 0≤1≤3 ✓ → PAIR (0,1) VALID
  TRY a=1: b=1-1=0 → 0≤0≤3 ✓ → PAIR (1,0) VALID
  TRY a=2: b=1-2=-1 → -1<0 ✗
  TRY a=3: b=1-3=-2 → -2<0 ✗

  VALID PAIRS: {(0,1), (1,0)}
  COUNT = 2
  
  FORMULA CHECK: min(1+1, 2×4-1-1) = min(2, 6) = 2 ✓
```

```
POSITION 2: need a+b=2, where 0≤a≤3, 0≤b≤3

  TRY a=0: b=2-0=2 → 0≤2≤3 ✓ → PAIR (0,2) VALID
  TRY a=1: b=2-1=1 → 0≤1≤3 ✓ → PAIR (1,1) VALID
  TRY a=2: b=2-2=0 → 0≤0≤3 ✓ → PAIR (2,0) VALID
  TRY a=3: b=2-3=-1 → -1<0 ✗

  VALID PAIRS: {(0,2), (1,1), (2,0)}
  COUNT = 3
  
  FORMULA CHECK: min(2+1, 2×4-1-2) = min(3, 5) = 3 ✓
```

```
POSITION 3 (PEAK): need a+b=3, where 0≤a≤3, 0≤b≤3

  TRY a=0: b=3-0=3 → 0≤3≤3 ✓ → PAIR (0,3) VALID
  TRY a=1: b=3-1=2 → 0≤2≤3 ✓ → PAIR (1,2) VALID
  TRY a=2: b=3-2=1 → 0≤1≤3 ✓ → PAIR (2,1) VALID
  TRY a=3: b=3-3=0 → 0≤0≤3 ✓ → PAIR (3,0) VALID

  VALID PAIRS: {(0,3), (1,2), (2,1), (3,0)}
  COUNT = 4
  
  FORMULA CHECK: min(3+1, 2×4-1-3) = min(4, 4) = 4 ✓
  
  ∴ PEAK occurs when both parts of min() are EQUAL
  ∴ PEAK is at position i where i+1 = 2n-1-i
  ∴ 2i = 2n-2 → i = n-1
  ∴ For n=4, PEAK at position 3 ✓
```

```
POSITION 4: need a+b=4, where 0≤a≤3, 0≤b≤3

  TRY a=0: b=4-0=4 → 4>3 ✗
  TRY a=1: b=4-1=3 → 0≤3≤3 ✓ → PAIR (1,3) VALID
  TRY a=2: b=4-2=2 → 0≤2≤3 ✓ → PAIR (2,2) VALID
  TRY a=3: b=4-3=1 → 0≤1≤3 ✓ → PAIR (3,1) VALID

  VALID PAIRS: {(1,3), (2,2), (3,1)}
  COUNT = 3
  
  FORMULA CHECK: min(4+1, 2×4-1-4) = min(5, 3) = 3 ✓
  
  NOW the SECOND part of min() is smaller → DECREASING phase
```

```
POSITION 5: need a+b=5, where 0≤a≤3, 0≤b≤3

  TRY a=0: b=5-0=5 → 5>3 ✗
  TRY a=1: b=5-1=4 → 4>3 ✗
  TRY a=2: b=5-2=3 → 0≤3≤3 ✓ → PAIR (2,3) VALID
  TRY a=3: b=5-3=2 → 0≤2≤3 ✓ → PAIR (3,2) VALID

  VALID PAIRS: {(2,3), (3,2)}
  COUNT = 2
  
  FORMULA CHECK: min(5+1, 2×4-1-5) = min(6, 2) = 2 ✓
```

```
POSITION 6: need a+b=6, where 0≤a≤3, 0≤b≤3

  TRY a=0: b=6-0=6 → 6>3 ✗
  TRY a=1: b=6-1=5 → 5>3 ✗
  TRY a=2: b=6-2=4 → 4>3 ✗
  TRY a=3: b=6-3=3 → 0≤3≤3 ✓ → PAIR (3,3) VALID

  VALID PAIRS: {(3,3)}
  COUNT = 1
  
  FORMULA CHECK: min(6+1, 2×4-1-6) = min(7, 1) = 1 ✓
```

```
POSITION 7 and beyond: need a+b≥7, where 0≤a≤3, 0≤b≤3

  MAXIMUM a+b = 3+3 = 6 < 7
  
  ∴ NO VALID PAIRS for position ≥ 7
  ∴ RESULT has exactly 2n-1 = 7 positions (0 through 6)
```

---

### SUMMARY TABLE FOR n=4

```
┌──────────┬────────────────────────────────┬───────┬─────────────────────────┐
│ Position │ Valid (a,b) pairs              │ Count │ Formula verification    │
├──────────┼────────────────────────────────┼───────┼─────────────────────────┤
│    0     │ (0,0)                          │   1   │ min(1,7)=1 ✓           │
│    1     │ (0,1), (1,0)                   │   2   │ min(2,6)=2 ✓           │
│    2     │ (0,2), (1,1), (2,0)            │   3   │ min(3,5)=3 ✓           │
│    3     │ (0,3), (1,2), (2,1), (3,0)     │   4   │ min(4,4)=4 ✓ ← PEAK    │
│    4     │ (1,3), (2,2), (3,1)            │   3   │ min(5,3)=3 ✓           │
│    5     │ (2,3), (3,2)                   │   2   │ min(6,2)=2 ✓           │
│    6     │ (3,3)                          │   1   │ min(7,1)=1 ✓           │
└──────────┴────────────────────────────────┴───────┴─────────────────────────┘

COEFFICIENTS: [1, 2, 3, 4, 3, 2, 1]
REPUNIT² = 1×10⁰ + 2×10¹ + 3×10² + 4×10³ + 3×10⁴ + 2×10⁵ + 1×10⁶
         = 1 + 20 + 300 + 4000 + 30000 + 200000 + 1000000
         = 1234321 ✓
```

---

### EXHAUSTIVE ENUMERATION FOR n=8 (15 output positions: 0 to 14)

```
REPUNIT for n=8: 11111111 (8 ones, positions 0-7)

POSITION 0:  a+b=0  → only (0,0) → COUNT=1
POSITION 1:  a+b=1  → (0,1),(1,0) → COUNT=2
POSITION 2:  a+b=2  → (0,2),(1,1),(2,0) → COUNT=3
POSITION 3:  a+b=3  → (0,3),(1,2),(2,1),(3,0) → COUNT=4
POSITION 4:  a+b=4  → (0,4),(1,3),(2,2),(3,1),(4,0) → COUNT=5
POSITION 5:  a+b=5  → (0,5),(1,4),(2,3),(3,2),(4,1),(5,0) → COUNT=6
POSITION 6:  a+b=6  → (0,6),(1,5),(2,4),(3,3),(4,2),(5,1),(6,0) → COUNT=7
POSITION 7:  a+b=7  → (0,7),(1,6),(2,5),(3,4),(4,3),(5,2),(6,1),(7,0) → COUNT=8 ← PEAK
POSITION 8:  a+b=8  → (1,7),(2,6),(3,5),(4,4),(5,3),(6,2),(7,1) → COUNT=7
             NOTE: (0,8) invalid because 8>7
POSITION 9:  a+b=9  → (2,7),(3,6),(4,5),(5,4),(6,3),(7,2) → COUNT=6
POSITION 10: a+b=10 → (3,7),(4,6),(5,5),(6,4),(7,3) → COUNT=5
POSITION 11: a+b=11 → (4,7),(5,6),(6,5),(7,4) → COUNT=4
POSITION 12: a+b=12 → (5,7),(6,6),(7,5) → COUNT=3
POSITION 13: a+b=13 → (6,7),(7,6) → COUNT=2
POSITION 14: a+b=14 → (7,7) → COUNT=1

COEFFICIENTS: [1, 2, 3, 4, 5, 6, 7, 8, 7, 6, 5, 4, 3, 2, 1]

11111111² = 123456787654321 ✓
```

---

## HARD PUZZLE: n=16 WITH MASSIVE CARRY CASCADE

```
n = 16
2n - 1 = 31
a ∈ {0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15}
b ∈ {0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15}

i=0: a+b=0
  a=0 → b=0 → 0∈{0..15} ✓
  a=1 → b=-1 → -1∉{0..15} ✗
  ∴ {(0,0)} → 1

i=1: a+b=1
  a=0 → b=1 ✓
  a=1 → b=0 ✓
  a=2 → b=-1 ✗
  ∴ {(0,1),(1,0)} → 2

i=2: a+b=2
  a=0 → b=2 ✓
  a=1 → b=1 ✓
  a=2 → b=0 ✓
  a=3 → b=-1 ✗
  ∴ 3

i=3: a+b=3 → {(0,3),(1,2),(2,1),(3,0)} → 4

i=4: a+b=4 → {(0,4),(1,3),(2,2),(3,1),(4,0)} → 5

i=5: a+b=5 → {(0,5),(1,4),(2,3),(3,2),(4,1),(5,0)} → 6

i=6: a+b=6 → 7

i=7: a+b=7 → 8

i=8: a+b=8 → 9

i=9: a+b=9 → 10

i=10: a+b=10 → 11

i=11: a+b=11 → 12

i=12: a+b=12 → 13

i=13: a+b=13 → 14

i=14: a+b=14 → 15

i=15: a+b=15
  a=0 → b=15 → 15∈{0..15} ✓
  a=1 → b=14 ✓
  a=2 → b=13 ✓
  a=3 → b=12 ✓
  a=4 → b=11 ✓
  a=5 → b=10 ✓
  a=6 → b=9 ✓
  a=7 → b=8 ✓
  a=8 → b=7 ✓
  a=9 → b=6 ✓
  a=10 → b=5 ✓
  a=11 → b=4 ✓
  a=12 → b=3 ✓
  a=13 → b=2 ✓
  a=14 → b=1 ✓
  a=15 → b=0 ✓
  ∴ 16 ← PEAK

i=16: a+b=16
  a=0 → b=16 → 16∉{0..15} ✗
  a=1 → b=15 → 15∈{0..15} ✓
  a=2 → b=14 ✓
  ...
  a=15 → b=1 ✓
  a=16 → 16∉{0..15} ✗
  ∴ {(1,15),(2,14),...,(15,1)} → 15

i=17: a+b=17
  a=0 → b=17 ✗
  a=1 → b=16 ✗
  a=2 → b=15 ✓
  a=3 → b=14 ✓
  ...
  a=15 → b=2 ✓
  ∴ {(2,15),(3,14),...,(15,2)} → 14

i=18: a+b=18 → {(3,15),...,(15,3)} → 13

i=19: a+b=19 → 12

i=20: a+b=20 → 11

i=21: a+b=21 → 10

i=22: a+b=22 → 9

i=23: a+b=23 → 8

i=24: a+b=24 → 7

i=25: a+b=25 → 6

i=26: a+b=26 → 5

i=27: a+b=27 → 4

i=28: a+b=28 → {(13,15),(14,14),(15,13)} → 3

i=29: a+b=29 → {(14,15),(15,14)} → 2

i=30: a+b=30
  a=14 → b=16 ✗
  a=15 → b=15 ✓
  a=16 → ✗
  ∴ {(15,15)} → 1

i=31: a+b=31
  a=15 → b=16 ✗
  a=16 → ✗
  ∴ {} → 0

∴ coef[] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1]
           ↑                                    ↑                                    ↑
          i=0                                  i=15                                 i=30
```

```
d₁ = 9
d₂ = 9
d₁ × d₂ = 81
```

```
coef[i] × 81 = val[i]:

coef[0]=1   → 1×81   = 1×80 + 1×1 = 80+1 = 81
coef[1]=2   → 2×81   = 2×80 + 2×1 = 160+2 = 162
coef[2]=3   → 3×81   = 3×80 + 3×1 = 240+3 = 243
coef[3]=4   → 4×81   = 4×80 + 4×1 = 320+4 = 324
coef[4]=5   → 5×81   = 5×80 + 5×1 = 400+5 = 405
coef[5]=6   → 6×81   = 6×80 + 6×1 = 480+6 = 486
coef[6]=7   → 7×81   = 7×80 + 7×1 = 560+7 = 567
coef[7]=8   → 8×81   = 8×80 + 8×1 = 640+8 = 648
coef[8]=9   → 9×81   = 9×80 + 9×1 = 720+9 = 729
coef[9]=10  → 10×81  = 10×80 + 10×1 = 800+10 = 810
coef[10]=11 → 11×81  = 11×80 + 11×1 = 880+11 = 891
coef[11]=12 → 12×81  = 12×80 + 12×1 = 960+12 = 972
coef[12]=13 → 13×81  = 13×80 + 13×1 = 1040+13 = 1053
coef[13]=14 → 14×81  = 14×80 + 14×1 = 1120+14 = 1134
coef[14]=15 → 15×81  = 15×80 + 15×1 = 1200+15 = 1215
coef[15]=16 → 16×81  = 16×80 + 16×1 = 1280+16 = 1296 ← max
coef[16]=15 → 15×81  = 1215
coef[17]=14 → 14×81  = 1134
coef[18]=13 → 13×81  = 1053
coef[19]=12 → 12×81  = 972
coef[20]=11 → 11×81  = 891
coef[21]=10 → 10×81  = 810
coef[22]=9  → 9×81   = 729
coef[23]=8  → 8×81   = 648
coef[24]=7  → 7×81   = 567
coef[25]=6  → 6×81   = 486
coef[26]=5  → 5×81   = 405
coef[27]=4  → 4×81   = 324
coef[28]=3  → 3×81   = 243
coef[29]=2  → 2×81   = 162
coef[30]=1  → 1×81   = 81

val[] = [81,162,243,324,405,486,567,648,729,810,891,972,1053,1134,1215,1296,1215,1134,1053,972,891,810,729,648,567,486,405,324,243,162,81]
```

```
i=0:  val[0]=81,   c=0   → t=81+0=81     → 81÷10=8r1   → d[0]=1, c=8
i=1:  val[1]=162,  c=8   → t=162+8=170   → 170÷10=17r0 → d[1]=0, c=17
i=2:  val[2]=243,  c=17  → t=243+17=260  → 260÷10=26r0 → d[2]=0, c=26
i=3:  val[3]=324,  c=26  → t=324+26=350  → 350÷10=35r0 → d[3]=0, c=35
i=4:  val[4]=405,  c=35  → t=405+35=440  → 440÷10=44r0 → d[4]=0, c=44
i=5:  val[5]=486,  c=44  → t=486+44=530  → 530÷10=53r0 → d[5]=0, c=53
i=6:  val[6]=567,  c=53  → t=567+53=620  → 620÷10=62r0 → d[6]=0, c=62
i=7:  val[7]=648,  c=62  → t=648+62=710  → 710÷10=71r0 → d[7]=0, c=71
i=8:  val[8]=729,  c=71  → t=729+71=800  → 800÷10=80r0 → d[8]=0, c=80
i=9:  val[9]=810,  c=80  → t=810+80=890  → 890÷10=89r0 → d[9]=0, c=89
i=10: val[10]=891, c=89  → t=891+89=980  → 980÷10=98r0 → d[10]=0, c=98
i=11: val[11]=972, c=98  → t=972+98=1070 → 1070÷10=107r0 → d[11]=0, c=107
i=12: val[12]=1053,c=107 → t=1053+107=1160 → 1160÷10=116r0 → d[12]=0, c=116
i=13: val[13]=1134,c=116 → t=1134+116=1250 → 1250÷10=125r0 → d[13]=0, c=125
i=14: val[14]=1215,c=125 → t=1215+125=1340 → 1340÷10=134r0 → d[14]=0, c=134
i=15: val[15]=1296,c=134 → t=1296+134=1430 → 1430÷10=143r0 → d[15]=0, c=143 ← max(c)
i=16: val[16]=1215,c=143 → t=1215+143=1358 → 1358÷10=135r8 → d[16]=8, c=135
i=17: val[17]=1134,c=135 → t=1134+135=1269 → 1269÷10=126r9 → d[17]=9, c=126
i=18: val[18]=1053,c=126 → t=1053+126=1179 → 1179÷10=117r9 → d[18]=9, c=117
i=19: val[19]=972, c=117 → t=972+117=1089  → 1089÷10=108r9 → d[19]=9, c=108
i=20: val[20]=891, c=108 → t=891+108=999   → 999÷10=99r9  → d[20]=9, c=99
i=21: val[21]=810, c=99  → t=810+99=909    → 909÷10=90r9  → d[21]=9, c=90
i=22: val[22]=729, c=90  → t=729+90=819    → 819÷10=81r9  → d[22]=9, c=81
i=23: val[23]=648, c=81  → t=648+81=729    → 729÷10=72r9  → d[23]=9, c=72
i=24: val[24]=567, c=72  → t=567+72=639    → 639÷10=63r9  → d[24]=9, c=63
i=25: val[25]=486, c=63  → t=486+63=549    → 549÷10=54r9  → d[25]=9, c=54
i=26: val[26]=405, c=54  → t=405+54=459    → 459÷10=45r9  → d[26]=9, c=45
i=27: val[27]=324, c=45  → t=324+45=369    → 369÷10=36r9  → d[27]=9, c=36
i=28: val[28]=243, c=36  → t=243+36=279    → 279÷10=27r9  → d[28]=9, c=27
i=29: val[29]=162, c=27  → t=162+27=189    → 189÷10=18r9  → d[29]=9, c=18
i=30: val[30]=81,  c=18  → t=81+18=99      → 99÷10=9r9   → d[30]=9, c=9

c=9 ≠ 0 → continue:
i=31: val=0, c=9 → t=0+9=9 → 9÷10=0r9 → d[31]=9, c=0

c=0 → stop

d[] = [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9]
       ↑                             ↑                               ↑
      i=0                           i=16                            i=31

d[31..0] → 9999999999999998 0000000000000001
           ←──── 16 ────→   ←──── 16 ────→
           
∴ 9999999999999999² = 99999999999999980000000000000001
```

```
10¹⁶ = 10000000000000000
10¹⁶ - 1 = 9999999999999999 ✓

(10¹⁶ - 1)² = (10¹⁶)² - 2×10¹⁶ + 1
            = 10³² - 2×10¹⁶ + 1

10³² = 1 00000000000000000000000000000000
2×10¹⁶ =               20000000000000000

10³² - 2×10¹⁶ = 99999999999999980000000000000000

99999999999999980000000000000000 + 1 = 99999999999999980000000000000001 ✓
```

---

## THE CARRY CASCADE INSIGHT

```
OBSERVATION 1: Carry grows during the ASCENDING phase (positions 0 to n-1)

At position 15 (peak for n=16):
  carry_in = 134
  val = 1296
  total = 1430
  carry_out = 143

The carry ITSELF is 143, which is larger than the coefficient (16)!
```

```
OBSERVATION 2: Carry decays during the DESCENDING phase (positions n to 2n-2)

At position 16:  carry drops from 143 to 135
At position 20:  carry drops to 99
At position 25:  carry drops to 54
At position 30:  carry drops to 9
At position 31:  carry becomes 0, DONE
```

```
OBSERVATION 3: Number of output zeros depends on how fast carry grows

For d₁×d₂ = 81 and n=16:
  - Positions 0-15 all produce digit 0 (except position 0 which is 1)
  - This creates 15 consecutive zeros in the middle of the number
  - Pattern: 999...998000...0001
             ↑       ↑
             n-1     n zeros in middle
```

---

## YOUR PREDICTED FAILURES (UPDATED)

```
FAILURE 1: You will compute 4444 × 6666 directly using O(n²) multiplication
           → WRONG because problem asks O(n)

FAILURE 2: You will forget that REPUNIT² has 2n-1 coefficients, not n
           → 1111² has 7 coefficient positions (0-6), not 4

FAILURE 3: You will assume pattern 1,2,3,...,n,...,3,2,1 works directly
           → This is the COEFFICIENT pattern, not the FINAL DIGIT pattern
           → Carries transform coefficients into completely different digits

FAILURE 4: You will forget the FINAL STEP: multiply each coefficient by (d₁×d₂)
           → For 4444×6666: multiply [1,2,3,4,3,2,1] by 24, then propagate

FAILURE 5: You will miscalculate coefficient at position i
           → WRONG: coef[i] = i+1 for all i
           → RIGHT: coef[i] = min(i+1, 2n-1-i)
           → The formula SWITCHES behavior at position n-1

FAILURE 6: You will process carries in wrong direction
           → WRONG: start at position 2n-2 (high end)
           → RIGHT: start at position 0 (low end)
           → Reason: carry flows from low to high positions

FAILURE 7: You will stop carry propagation too early
           → WRONG: stop at position 2n-2
           → RIGHT: continue while carry > 0
           → Result may have 2n digits, not 2n-1

FAILURE 8: You will use u8 for intermediate calculations
           → WRONG: 16×81 = 1296 > 255 (u8 max)
           → RIGHT: use u16 or u32 for val×(d₁×d₂)+carry

FAILURE 9: You will forget to handle carry > 99
           → At n=16, carry reaches 143 at the peak
           → Single carry propagation step may produce carry > 99
           → But output digit is always < 10 (single digit)
```

---

## FORMULA DERIVATION: Why count = min(i+1, 2n-1-i)

```
GIVEN: position i, need pairs (a,b) where a+b=i, 0≤a≤n-1, 0≤b≤n-1

REWRITE: b = i - a
CONSTRAINT ON b: 0 ≤ i-a ≤ n-1
SOLVE FOR a:
  i - a ≥ 0  → a ≤ i
  i - a ≤ n-1 → a ≥ i - n + 1

CONSTRAINT ON a: 0 ≤ a ≤ n-1

COMBINE ALL CONSTRAINTS:
  a ≥ max(0, i-n+1)
  a ≤ min(i, n-1)

COUNT = min(i, n-1) - max(0, i-n+1) + 1
```

```
CASE 1: i < n (positions before the peak)

  max(0, i-n+1) = max(0, negative) = 0
  min(i, n-1) = i  (since i < n means i ≤ n-1)
  
  COUNT = i - 0 + 1 = i + 1
```

```
CASE 2: i ≥ n (positions at or after peak)

  max(0, i-n+1) = i-n+1  (positive since i ≥ n)
  min(i, n-1) = n-1  (since i ≥ n means i > n-1)
  
  COUNT = (n-1) - (i-n+1) + 1
        = n - 1 - i + n - 1 + 1
        = 2n - 1 - i
```

```
UNIFIED FORMULA:
  count = min(i+1, 2n-1-i)
  
  For i < n:   min(i+1, 2n-1-i) = i+1       (i+1 < 2n-1-i when i < n-1)
  For i = n-1: min(n, n) = n               (PEAK: both parts equal)
  For i > n-1: min(i+1, 2n-1-i) = 2n-1-i   (2n-1-i < i+1 when i > n-1)
```

---

## HARDER EXAMPLES TO BREAK YOUR CODE

```
CASE 1: n=1
  3 × 7 = 21
  REPUNIT = 1
  1² = 1
  3 × 7 × 1 = 21 ✓

CASE 2: n=2
  33 × 77 = 2541
  d₁ × d₂ = 21
  REPUNIT² = 11² = 121
  21 × 121 = 2541 ✓

CASE 3: n=8 (power of 2)
  88888888 × 22222222 = ?
  d₁ × d₂ = 8 × 2 = 16
  REPUNIT = 11111111
  REPUNIT² = 123456787654321 (15 digits)
  16 × 123456787654321 = 1975308602469136

CASE 4: n=4, same digits
  5555 × 5555 = ?
  d₁ × d₂ = 25
  REPUNIT² = 1234321
  25 × 1234321 = 30858025
  CHECK: 5555² = 30858025 ✓

CASE 5: d₁ × d₂ causes cascading carries
  9999 × 9999 = ?
  d₁ × d₂ = 81
  REPUNIT² = 1234321
  81 × 1234321 = 99999801

  VERIFY: 9999² = 99980001 
  WAIT — MISMATCH!
  
  RECALCULATE:
  9999 × 9999 = (10000 - 1)² = 100000000 - 20000 + 1 = 99980001
  
  81 × 1234321 = ?
    1234321
  ×      81
  ---------
    1234321  (× 1)
   9874568   (× 80)
  ---------
   99980001 ✓
```

---

## EDGE CASES THAT WILL DESTROY YOU

```
EDGE 1: n=1, d₁=1, d₂=1
  1 × 1 = 1
  REPUNIT² = 1² = 1
  1 × 1 × 1 = 1 ✓

EDGE 2: n=1, d₁=9, d₂=9
  9 × 9 = 81
  2-digit result from 1-digit inputs ✓

EDGE 3: d₁ = 0 (is 0000 a valid simple number?)
  If digit = 0, then 0000 = 0
  0 × anything = 0
  SPECIAL CASE or INVALID INPUT?

EDGE 4: Maximum n that won't overflow u64
  9999...9 (n nines) × 9999...9 (n nines)
  ≈ 10^n × 10^n = 10^(2n)
  u64 max ≈ 1.8 × 10^19
  2n ≤ 19 → n ≤ 9
  BUT n must be power of 2 → n ∈ {1, 2, 4, 8}
  n=16 will OVERFLOW u64

EDGE 5: What if inputs have different n?
  Problem says "n digits each" → same n
  BUT what if 333 × 4444? → UNDEFINED BEHAVIOR
```

---

## WORK THESE BY HAND BEFORE CODING

```
EXERCISE 1: 22 × 33 = ?
  d₁=2, d₂=3, n=2
  d₁×d₂ = 6
  REPUNIT² = 11² = 121
  6 × 121 = 726
  CHECK: 22 × 33 = 726 ✓

EXERCISE 2: 1111 × 2222 = ?
  YOUR TURN — CALCULATE EVERY STEP

EXERCISE 3: 44444444 × 55555555 = ?
  n=8
  d₁×d₂ = 20
  REPUNIT² = 11111111² = ?
  CALCULATE THE 15 COEFFICIENTS
  THEN MULTIPLY BY 20
  THEN PROPAGATE CARRIES
```

---

## THE FUNCTION YOU MUST IMPLEMENT

```
fn multiply_simple(n: usize, d1: u8, d2: u8) -> Vec<u8>
```

```
WHERE:
  n = number of digits (power of 2)
  d1 = repeated digit of first number (1-9)
  d2 = repeated digit of second number (1-9)
  RETURNS = digits of product, least significant first

EXAMPLE:
  multiply_simple(4, 4, 6) → [4, 0, 7, 3, 2, 6, 9, 2]
  (representing 29623704)
```

---

## COMPLEXITY YOU MUST HIT

```
TIME: O(n)
  - Generating coefficients: O(n) ← 2n-1 coefficients
  - Multiplying by (d₁×d₂): O(n) ← 2n-1 multiplications
  - Carry propagation: O(n) ← 2n-1 additions

SPACE: O(n)
  - Storing 2n-1 intermediate values
  - Storing 2n result digits

ONE ADDITION/MULTIPLICATION = O(1)
```

---

## MISTAKES LOG

```
MISTAKE 1: todo!() before code

WROTE:
  todo!("Implement multiply_simple")
  let product : u16 = ...

PROBLEM:
  todo!() → panic → code after = unreachable
  
FIX:
  delete todo!()
```

```
MISTAKE 2: typo in variable name

WROTE:
  let coef = std::cmp::min(i+1, num_coff - 1);
                               ↑
                               num_coff (missing 's')

CORRECT:
  num_coffs
```

```
MISTAKE 3: wrong formula

WROTE:
  min(i+1, num_coffs - 1)
                     ↑
                     -1 (constant)

CORRECT:
  min(i+1, num_coffs - i)
                     ↑
                     -i (varies with i)

NUMERICAL PROOF:

  WRONG FORMULA (num_coffs - 1 = 6):
  i=0: min(1, 6) = 1 ← happens to work
  i=3: min(4, 6) = 4 ← happens to work
  i=6: min(7, 6) = 6 ✗ should be 1

  CORRECT FORMULA (num_coffs - i):
  i=0: min(1, 7-0) = min(1, 7) = 1 ✓
  i=3: min(4, 7-3) = min(4, 4) = 4 ✓
  i=6: min(7, 7-6) = min(7, 1) = 1 ✓
```

```
MISTAKE 4: confusion about 2n-1

QUESTION: why 2n-1?

FAILED TO DERIVE:
  jumped to formula without paper calculation

CORRECT DERIVATION:
  1111 × 1111 on paper:
  
        1 1 1 1
      × 1 1 1 1
      ---------
  col:  6 5 4 3 2 1 0
  
  row1:       1 1 1 1  (cols 0-3)
  row2:     1 1 1 1    (cols 1-4)
  row3:   1 1 1 1      (cols 2-5)
  row4: 1 1 1 1        (cols 3-6)
  
  columns used: 0, 1, 2, 3, 4, 5, 6
  count = 7 = 2×4 - 1
  
  min(a+b) = 0+0 = 0
  max(a+b) = 3+3 = 6 = 2×(n-1) = 2n-2
  count = (2n-2) - 0 + 1 = 2n-1
```

```
MISTAKE 5: confusion about min formula origin

FAILED TO DERIVE:
  accepted min(i+1, num_coffs-i) without proof

CORRECT DERIVATION:
  count 1s at each column from paper:
  
  col 0: 1 = 0+1
  col 1: 2 = 1+1
  col 2: 3 = 2+1
  col 3: 4 = 3+1
  col 4: 3 = 7-4
  col 5: 2 = 7-5
  col 6: 1 = 7-6
  
  formula1 = i+1 (works for i < peak)
  formula2 = num_coffs-i (works for i ≥ peak)
  
  correct = smaller of (formula1, formula2)
          = min(i+1, num_coffs-i)
```

```
MISTAKE 6: confusion about col 0 position

CONFUSION:
  "col 0 on right" vs "col 0 on left"

CLARIFICATION:
  PAPER: col 0 = rightmost = 10⁰ = units
  ARRAY: coef[0] = count at col 0 = 1
  
  no conflict: array index = column number = power of 10
```

---

## ORTHOGONAL THOUGHT PROCESS

```
SLOPPY PATTERN:
  accept formula → use formula → fail

ORTHOGONAL PATTERN:
  paper calculation → count → observe pattern → derive formula → verify
```

```
SLOPPY PATTERN:
  min(i+1, num_coffs-1) "looks similar"

ORTHOGONAL PATTERN:
  test at boundary:
  i=6: min(7, 6) = 6 ≠ 1 → WRONG
```

```
SLOPPY PATTERN:
  confuse visual position with array index

ORTHOGONAL PATTERN:
  write explicit mapping:
  col 0 (paper) = coef[0] (array) = 10⁰
  col 6 (paper) = coef[6] (array) = 10⁶
```

---

## FINAL IMPLEMENTATION

```rust
fn multiply_simple(n: usize, d1: u8, d2: u8) -> Vec<u8> {
    let product: u16 = (d1 as u16) * (d2 as u16);
    let num_coffs = 2 * n - 1;
    let mut vals: Vec<u32> = vec![0; num_coffs];

    for i in 0..num_coffs {
        let coef = std::cmp::min(i + 1, num_coffs - i);
        vals[i] = (coef as u32) * (product as u32);
    }

    let mut result: Vec<u8> = Vec::new();
    let mut carry: u32 = 0;

    for &val in vals.iter() {
        let total = val + carry;
        let digit = (total % 10) as u8;
        carry = total / 10;
        result.push(digit);
    }

    while carry > 0 {
        let digit = (carry % 10) as u8;
        carry = carry / 10;
        result.push(digit);
    }

    result
}
```

```
TEST RESULTS: 11/11 passed ✓
```
