# Anagram Detection: Brute Force Derivation

## THE PROBLEM

```
INPUT:  s1 = "listen", s2 = "silent"
OUTPUT: true or false
```

## EXERCISE 1: COUNT BY HAND

CALCULATE: How many times does each character appear in s1?

```
s1 = "listen"
     ↓ ↓ ↓ ↓ ↓ ↓
     l i s t e n
     
     Position 0: ___
     Position 1: ___
     Position 2: ___
     Position 3: ___
     Position 4: ___
     Position 5: ___

YOUR COUNT TABLE FOR s1:
┌──────────┬───────────────────────────┐
│ Character│ How many times in s1?     │
├──────────┼───────────────────────────┤
│    l     │                           │
│    i     │                           │
│    s     │                           │
│    t     │                           │
│    e     │                           │
│    n     │                           │
└──────────┴───────────────────────────┘
```

NOW DO THE SAME FOR s2:

```
s2 = "silent"
     ↓ ↓ ↓ ↓ ↓ ↓
     s i l e n t
     
YOUR COUNT TABLE FOR s2:
┌──────────┬───────────────────────────┐
│ Character│ How many times in s2?     │
├──────────┼───────────────────────────┤
│    s     │                           │
│    l     │                           │
│    i     │                           │
│    e     │                           │
│    n     │                           │
│    t     │                           │
└──────────┴───────────────────────────┘
```

COMPARE YOUR TWO TABLES. WHAT DO YOU OBSERVE?

YOUR ANSWER: Same characters with same frequency _______________________________________________________________


---

## EXERCISE 2: HARDER INPUT

```
s1 = "anagram"
s2 = "nagaram"
```

COUNT EACH CHARACTER IN s1:

```
s1 = "anagram"
     a n a g r a m
     0 1 2 3 4 5 6
     
Walk through:
  Position 0: character = _a__, running tally for this char = 1___
  Position 1: character = ___n, running tally for this char = __1_
  Position 2: character = _a__, running tally for this char = __2_
  Position 3: character = __g_, running tally for this char = __1_
  Position 4: character = _r__, running tally for this char = _1__
  Position 5: character = __a_, running tally for this char = ___3
  Position 6: character = ___m, running tally for this char = 1___

FINAL COUNT FOR s1:
┌──────────┬───────────────────────────┐
│ Character│ Count                     │
├──────────┼───────────────────────────┤
│          │                           │
│          │                           │
│          │                           │
│          │                           │
│          │                           │
└──────────┴───────────────────────────┘
```

NOW DO s2 = "nagaram":

```
FINAL COUNT FOR s2:
┌──────────┬───────────────────────────┐
│ Character│ Count                     │
├──────────┼───────────────────────────┤
│          │                           │
│          │                           │
│          │                           │
│          │                           │
│          │                           │
└──────────┴───────────────────────────┘
```

ARE THEY ANAGRAMS? YOUR ANSWER: _Yes__


---

## EXERCISE 3: NOT ANAGRAMS

```
s1 = "aab"
s2 = "abb"
```

COUNT s1:
```
┌──────────┬───────────────────────────┐
│ Character│ Count                     │
├──────────┼───────────────────────────┤
│    a     │                           │
│    b     │                           │
└──────────┴───────────────────────────┘
```

COUNT s2:
```
┌──────────┬───────────────────────────┐
│ Character│ Count                     │
├──────────┼───────────────────────────┤
│    a     │                           │
│    b     │                           │
└──────────┴───────────────────────────┘
```

COMPARE. WHICH CHARACTER HAS DIFFERENT COUNTS? ___

ARE THEY ANAGRAMS? YOUR ANSWER: ___


---

## EXERCISE 4: DIFFERENT LENGTHS

```
s1 = "abc"
s2 = "ab"
```

LEN(s1) = ___
LEN(s2) = ___

ARE THE LENGTHS EQUAL? ___

IF LENGTHS ARE NOT EQUAL, CAN THEY EVER BE ANAGRAMS? 

YOUR REASONING (use numbers, not words):

_______________________________________________________________


---

## EXERCISE 5: YOU TELL ME HOW TO CHECK

You just did 4 manual comparisons. 

DESCRIBE YOUR PROCEDURE using only these words:
- count
- compare
- character
- s1
- s2
- equal
- not equal

YOUR PROCEDURE IN 3-5 STEPS:

1. count of they are same length    _______________________________________________________________

2.  use an extra array of 256 characters, with each num being 0, then when I see a character in first array, all I do is increment the value at the index which is ascii code of that character. THen what I do I see the second array, then I do the decrease of the numbers at thatt ascii code value index, then I run on the static array again and I see if any element is non zero and if it is they are not anargams. _______________________________________________________________

3. _______________________________________________________________

4. _______________________________________________________________

5. _______________________________________________________________


---

## EXERCISE 6: THE STORAGE QUESTION

You made "count tables" by hand on paper.

THE QUESTION: In code, WHERE do you store these counts?

OPTION A: Two arrays of size 26 (one for a-z in s1, one for a-z in s2)
OPTION B: One array of size 26
OPTION C: A HashMap
OPTION D: Sort both strings and compare
OPTION E: Something else

YOU PICK ONE. THEN CALCULATE WHY IT WORKS OR FAILS:

YOUR CHOICE: ___

YOUR CALCULATION SHOWING WHY IT WORKS:

_______________________________________________________________

_______________________________________________________________


---

## EXERCISE 7: IMPLEMENT OPTION A (TWO ARRAYS)

If you pick two arrays:

```
Array for s1:  [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
               [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z]
                0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25

s1 = "listen"
```

CALCULATE: Which index does 'l' go to?

'l' ASCII = 108
'a' ASCII = 97
Index = 108 - 97 = ___

CALCULATE: Which index does 'i' go to?

'i' ASCII = ___
Index = ___ - 97 = ___

FILL THE ARRAY FOR s1 = "listen":

```
After processing 'l':
[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, _, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                               ↑
                          index 11

After processing 'i':
[_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]

After processing 's':
[_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]

After processing 't':
[_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]

After processing 'e':
[_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]

After processing 'n':
[_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]
```

NOW DO THE SAME FOR s2 = "silent":

```
Array for s2 (FILL IT):
[_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _]
```

COMPARE THE TWO ARRAYS ELEMENT BY ELEMENT:

s1_arr[0] == s2_arr[0]? ___
s1_arr[1] == s2_arr[1]? ___
...
s1_arr[11] == s2_arr[11]? ___  (the 'l' position)
...

ALL EQUAL? ___


---

## EXERCISE 8: IMPLEMENT OPTION B (ONE ARRAY)

Can you use ONE array instead of two?

HINT: What if you INCREMENT for s1 and DECREMENT for s2?

CALCULATE FOR s1="ab", s2="ba":

```
Start: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        a  b
        0  1

Process s1[0]='a': INCREMENT arr[0]
Result: [_, 0, 0, ...]

Process s1[1]='b': INCREMENT arr[1]
Result: [_, _, 0, ...]

Process s2[0]='b': DECREMENT arr[1]
Result: [_, _, 0, ...]

Process s2[1]='a': DECREMENT arr[0]
Result: [_, _, 0, ...]
```

CALCULATE: What should all values be if they are anagrams?

YOUR ANSWER: ___


---

## EXERCISE 9: THE SORTING APPROACH

SORT s1 = "listen":

```
Original: l, i, s, t, e, n
Sorted:   _, _, _, _, _, _
```

SORT s2 = "silent":

```
Original: s, i, l, e, n, t
Sorted:   _, _, _, _, _, _
```

COMPARE SORTED STRINGS. ARE THEY EQUAL? ___

DOES THIS TELL YOU THEY ARE ANAGRAMS? ___


---

## EXERCISE 10: WHICH DO YOU PICK AND WHY?

OPTION A: Two arrays of size 26
OPTION B: One array of size 26 (increment/decrement)
OPTION C: HashMap
OPTION D: Sort both strings

WHICH ONE REQUIRES THE LEAST MEMORY? CALCULATE:

Option A memory: ___ bytes (26 * 4 * 2 arrays = ?)
Option B memory: ___ bytes
Option C memory: ___ bytes (HashMap overhead + entries)
Option D memory: ___ bytes (need to copy strings to sort)

WHICH ONE IS FASTEST? CALCULATE TIME COMPLEXITY:

Option A time: ___
Option B time: ___
Option C time: ___
Option D time: ___


---

## YOU DECIDE. THEN IMPLEMENT IN THE .RS FILE.

---

## MISTAKE LOG

```
1. counter_array[u8; 256]
   You wrote [ where : goes.
   Did you look at the syntax before typing?
```

```
2. u8
   Range: 0..255.
   1 - 1 - 1 = -1.
   -1 fits in u8?
   You did not calculate.
```

```
3. [0, 256]
   You typed comma.
   Comma = list of elements.
   Semicolon = repetition.
   [0, 256] = 2 elements.
   [0; 256] = 256 zeros.
   You confused two operators.
```

```
4. for (a,b) in (s1,s2)
   (s1,s2) is a tuple.
   A tuple is not iterable.
   You assumed Python syntax works in Rust.
   It does not.
```

```
5. uszie
   You typed uszie.
   The type is usize.
   You did not read what you typed.
   Twice.
```

```
6. +=2
   Your algorithm: s1 → +1, s2 → -1.
   You typed +=2.
   +2 is not -1.
   You did not read your own algorithm.
```

```
7. [i32 : 256]
   You changed u8 to i32.
   You did not fix the colon.
   Same error. Same position. Second time.
   You fixed one thing. You ignored another.
```

```
8. for ( c in arr)
   You wrapped c in parentheses.
   Rust for-loop: for PAT in ITER { }.
   No parentheses around PAT in.
   You cargo-culted C/Java syntax.
```

```
9. if (c != 0)
   Parentheses around condition.
   Rust does not need them.
   Warning issued.
   You ignored warning style from other languages.
```

---

## PATTERN ANALYSIS

```
ERRORS 1, 7: Same syntax error. Different lines. You fix locally. You do not generalize.

ERRORS 2, 6: You derived algorithm. You did not follow it. Gap between thinking and typing.

ERRORS 3, 4, 8, 9: You imported syntax from other languages. You did not verify Rust syntax.

ERROR 5: You did not proofread. Twice.
```

---

## QUESTIONS

```
Q1: Before typing, do you pause to verify syntax?
Q2: After deriving algorithm, do you trace each line against it?
Q3: When fixing one error on a line, do you check rest of line?
Q4: Do you read compiler output or guess?
```

