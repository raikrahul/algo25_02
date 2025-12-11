# 26. String Permutation Dictionary Lookup

01. Write input s="abc", dict={"abc","bca","cab","xyz","dog"}, count |s|=3, count |dict|=5, goal=find all rearrangements of "abc" that appear in dict → output should be subset of dict where each word is anagram of s.
02. Calculate 3!=3×2×1=6, write all 6 permutations by hand on paper: abc,acb,bac,bca,cab,cba, verify count=6 matches 3!, circle ones in dict: abc✓ bca✓ cab✓, cross out rest: acb✗ bac✗ cba✗, output=["abc","bca","cab"].
03. Draw chars array at address 0x100: `[a|b|c]` with indices 0,1,2 below, this array will be mutated by swap operations during recursion, write `chars: &mut Vec<char> = s.chars().collect()` → chars=['a','b','c'] at 0x100.
04. Trace swap(0,0) at depth=0: swap(chars[0],chars[0]) → no change → chars=['a','b','c'], call backtrack(chars,1,dict,results), this explores all perms starting with 'a' at position 0.
05. At depth=1 with chars=['a','b','c']: swap(1,1)→no change→chars=['a','b','c'], call backtrack(2), at depth=2 swap(2,2)→no change, depth=3=len→emit chars.iter().collect()="abc"→dict.contains("abc")?✓→results.push("abc").
06. Return to depth=2, return to depth=1, now swap(1,2): chars[1]='b'↔chars[2]='c' → chars=['a','c','b'], call backtrack(2), depth=2 swap(2,2), depth=3=len→emit "acb"→dict.contains("acb")?✗→skip, MUST swap(1,2) back: chars=['a','b','c'].
07. Return to depth=0, now swap(0,1): chars[0]='a'↔chars[1]='b' → chars=['b','a','c'] at 0x100, call backtrack(1), this explores all perms starting with 'b'.
08. At depth=1 chars=['b','a','c']: swap(1,1)→emit "bac"→dict.contains?✗, swap(1,2)→chars=['b','c','a']→emit "bca"→dict.contains?✓→push, swap(1,2)back→['b','a','c'], return, MUST swap(0,1)back→['a','b','c'].
09. At depth=0, now swap(0,2): chars[0]='a'↔chars[2]='c' → chars=['c','b','a'], call backtrack(1), swap(1,1)→emit "cba"✗, swap(1,2)→chars=['c','a','b']→emit "cab"✓→push, swap back twice→['a','b','c'].
10. Final results=["abc","bca","cab"], verify |results|=3, verify each is in dict, verify no duplicates since input "abc" has 3 distinct chars.
11. Write function signature: `fn backtrack(chars: &mut Vec<char>, start: usize, dict: &HashSet<String>, results: &mut Vec<String>)` → 4 parameters, chars mutated in place, start=current recursion depth, dict for O(1) lookup, results accumulator.
12. Write base case inside backtrack: `if start == chars.len() { let s: String = chars.iter().collect(); if dict.contains(&s) { results.push(s); } return; }` → when start=3=len, emit current arrangement, check membership, push if found.
13. Write recursion loop: `for i in start..chars.len() { chars.swap(start, i); backtrack(chars, start+1, dict, results); chars.swap(start, i); }` → try each char at position start, recurse, UNDO swap after return.
14. Test s="" (empty): len=0, 0!=1, base case triggers immediately with empty string "", if ""∈dict→push, else empty result, write test: `assert_eq!(find("", &make_dict(&[""])), vec![""])`.
15. Test s="a" (single): len=1, 1!=1, swap(0,0)→emit "a", base case at depth=1, write test: `assert_eq!(find("a", &make_dict(&["a","b"])), vec!["a"])`.
16. Test s="ab": len=2, 2!=2, depth=0 swap(0,0)→depth=1 swap(1,1)→emit "ab", swap(0,1)→['b','a']→depth=1 swap(1,1)→emit "ba", write test with both in dict.
17. Test duplicates s="aab": 3!=6 raw perms, but unique=3!/2!=3, swap-based generates: depth=0 i=0→a, i=1→a (duplicate subtree!), i=2→b, you will get "aab" twice from i=0 and i=1 paths, MUST use HashSet<String> for results or check chars[i]==chars[start]∧i≠start→skip.
18. Calculate unique perms with formula: n!/∏(count[c]!) → s="aab" has a:2,b:1 → 3!/(2!×1!)=6/2=3 → {aab,aba,baa}, s="aabb" has a:2,b:2 → 4!/(2!×2!)=24/4=6.
19. Trace s="aab" with naive swap: [a,a,b] at 0x100, depth=0 i=0: swap(0,0)→[a,a,b]→subtree emits aab,aba, depth=0 i=1: swap(0,1)→[a,a,b] SAME! →subtree emits aab,aba again→DUPLICATES, depth=0 i=2: swap(0,2)→[b,a,a]→subtree emits baa,baa.
20. Fix duplicates option 1: wrap results in HashSet, convert to Vec at end → `let mut set: HashSet<String>` then `results = set.into_iter().collect()`, handles any duplicate pattern.
21. Fix duplicates option 2: skip if chars[i]==chars[start] for i>start → `if i > start && chars[i] == chars[start] { continue; }` BUT requires sorted input chars first, else misses some dupes.
22. Compare Method1(generate n! perms, lookup each) vs Method2(scan dict, check anagram): Method1=O(n!×n), Method2=O(|dict|×n), choose Method1 when n!<|dict|, choose Method2 when n!>|dict|.
23. Calculate crossover: n=5→5!=120, n=6→6!=720, n=7→7!=5040, n=8→8!=40320, if |dict|=10000 then crossover at n≈7, if |dict|=100000 then crossover at n≈8.
24. Write is_anagram(a,b): sort both strings, compare → `let mut a: Vec<char>=a.chars().collect(); a.sort(); let mut b: Vec<char>=b.chars().collect(); b.sort(); a==b` → O(n log n) per comparison.
25. Write Method2: `for word in dict { if word.len() == s.len() && is_anagram(word, s) { results.push(word.clone()); } }` → O(|dict|×n log n) total.
26. Write load_dictionary: `fn load_dict(path: &str) -> HashSet<String> { let file = File::open(path).unwrap(); BufReader::new(file).lines().filter_map(|l| l.ok()).collect() }` → reads file line by line into HashSet.
27. Draw memory at emit point depth=3 for s="abc": stack has 4 frames at 0x200,0x220,0x240,0x260, each frame stores start value 0,1,2,3, chars pointer 0x100 shared across all frames, heap has chars=['a','b','c'] at 0x100, results=["abc"] at 0x300.
28. F1: you will write `if start == chars.len()` inside the for loop instead of before → emits multiple times per base case → wrong output count.
29. F2: you will write `chars.swap(start, i)` once but forget the second swap after recursion returns → chars permanently mutated → wrong permutations generated.
30. F3: you will use `Vec<String>` for dict instead of `HashSet<String>` → O(n) lookup per iteration → O(n!×n×|dict|) total instead of O(n!×n).
31. F4: you will write `results.push(s)` which moves s, then try to use s again → borrow checker error, must use `results.push(s.clone())` or restructure.
32. F5: you will forget that chars is `&mut` and try to return it or clone it incorrectly → lifetime errors.
33. F6: you will test only distinct-char inputs like "abc" and miss duplicate-char edge cases like "aab","aaa","aabb".
34. F7: you will assume dict file is lowercase but it may have "ABC","Dog" → case mismatch → no matches found.
35. Verify by hand: s="dog", dict has "dog","god" → 3!=6 perms: dog,dgo,odg,ogd,gdo,god → in dict: dog✓,god✓ → output=["dog","god"], run test to confirm.

---
AXIOM CHECK (placed at end per user request):
- Line 01-02: derived 3!=6 from definition, listed all 6 by hand, no new inference introduced.
- Line 03-09: traced swap operations step by step with actual array contents, each swap shown before/after.
- Line 11-13: code follows directly from trace, no new concepts introduced without prior derivation.
- Line 17-21: duplicate handling derived from observing trace of s="aab", not stated as rule first.
- Line 22-23: crossover calculation done numerically, compared actual factorials to dictionary sizes.
- Line 28-35: failures derived from common coding mistakes, each tied to specific code pattern.
- No memorization required: each formula used (factorial, multinomial) was calculated with actual numbers first.
- No adjectives, no adverbs, no filler words in step lines.
- Each step is self-contained action with specific data.

---
ERROR LOG (mistakes made during implementation):

E1. Line 63: wrote `dict.contains(&s)` → parameter named `dictionary` not `dict` → compiler error: cannot find value `dict` → fix: `dictionary.contains(&s)` → why sloppy: copied from comment without checking actual parameter name → missed: function signature 3 lines above → prevent: read function signature before writing body.

E2. Line 88: wrote `let chars: &mut Vec<char> = s.chars().collect()` → s.chars().collect() returns owned Vec<char>, not reference → type mismatch error → fix: `let mut chars: Vec<char> = s.chars().collect()` → why sloppy: confused reference with ownership → missed: collect() creates new owned value → prevent: trace type flow: iter→collect→Vec (owned).

E3. Line 88-91: wrote body without return statement → function signature says `-> Vec<String>` → compiler error: expected Vec<String>, found () → fix: add `results` as last expression → why sloppy: forgot Rust implicit return → missed: function must return value → prevent: always write return value first, then fill body.

E4. Lines 61-67: wrote base case `if start == chars.len() { ... return; }` with return inside if → works but early return inside base case, loop below still reachable for edge case len=0 → no bug here but pattern unclear → why sloppy: mixed return styles → missed: could omit return and use else → prevent: pick one style: early return OR if-else.

QUESTION YOUR BRAIN:
Q1. Why did you write `dict` when parameter is `dictionary`? → Answer: autopilot from reading old code, not checking current context.
Q2. Why did you confuse `&mut Vec` with `Vec`? → Answer: did not trace ownership, assumed reference because function takes `&mut`.
Q3. Why did you forget return statement? → Answer: wrote code top-to-bottom, forgot to close the loop with output.
Q4. Pattern: all 3 errors = not reading what exists, writing from memory of similar code → Brain defaults to cached patterns instead of current context.

ORTHOGONAL THOUGHT:
- Error E1: lexical error (wrong identifier) → fix = rename
- Error E2: type error (reference vs owned) → fix = change type annotation
- Error E3: structural error (missing expression) → fix = add expression
- All 3 are different categories, caught at different compiler stages
- None are logic errors, all are syntax/type errors
- Logic was correct from start, only transcription failed

---
NUMERICAL TRACE: backtrack() with line numbers

Line mapping:
L61: if start == chars.len()
L62: let s: String = chars.iter().collect()
L63: if dictionary.contains(&s)
L64: results.push(s)
L65: return
L68: for i in start..chars.len()
L69: chars.swap(start, i)
L70: backtrack(chars, start+1, dictionary, results)
L71: chars.swap(start, i)

---
EXAMPLE 1: s="ab", dict={"ab","ba"}
chars@0x100=['a','b'], len=2, results@0x200=[]

CALL₀: backtrack(0x100, 0, dict, 0x200)
│ L61: 0==2? → 0≠2 → ✗ → skip base
│ L68: i∈{0,1}
│ i=0:
│   L69: swap(0,0) → ['a','b'] (no change)
│   L70: CALL₁
│   │ L61: 1==2? → ✗
│   │ L68: i∈{1}
│   │ i=1:
│   │   L69: swap(1,1) → ['a','b']
│   │   L70: CALL₂
│   │   │ L61: 2==2? → ✓
│   │   │ L62: s="ab"
│   │   │ L63: "ab"∈dict? → ✓
│   │   │ L64: results.push("ab") → results=["ab"]
│   │   │ L65: return
│   │   L71: swap(1,1) → ['a','b']
│   L71: swap(0,0) → ['a','b']
│ i=1:
│   L69: swap(0,1) → ['b','a'] ← SURPRISE: first actual mutation!
│   L70: CALL₃
│   │ L61: 1==2? → ✗
│   │ L68: i∈{1}
│   │ i=1:
│   │   L69: swap(1,1) → ['b','a']
│   │   L70: CALL₄
│   │   │ L61: 2==2? → ✓
│   │   │ L62: s="ba"
│   │   │ L63: "ba"∈dict? → ✓
│   │   │ L64: results.push("ba") → results=["ab","ba"]
│   │   │ L65: return
│   │   L71: swap(1,1) → ['b','a']
│   L71: swap(0,1) → ['a','b'] ← SURPRISE: restored to original!
RESULT: ["ab","ba"], |results|=2=2! ✓

---
EXAMPLE 2: s="a", dict={"a"}
chars@0x100=['a'], len=1, results=[]

CALL₀: backtrack(0x100, 0, dict, 0x200)
│ L61: 0==1? → ✗
│ L68: i∈{0}
│ i=0:
│   L69: swap(0,0) → ['a']
│   L70: CALL₁
│   │ L61: 1==1? → ✓ ← SURPRISE: base case at depth=1, not depth=0
│   │ L62: s="a"
│   │ L63: "a"∈dict? → ✓
│   │ L64: results.push("a")
│   │ L65: return
│   L71: swap(0,0) → ['a']
RESULT: ["a"], |results|=1=1! ✓

---
EXAMPLE 3: s="", dict={""}
chars@0x100=[], len=0, results=[]

CALL₀: backtrack(0x100, 0, dict, 0x200)
│ L61: 0==0? → ✓ ← SURPRISE: base case triggers immediately!
│ L62: s=""
│ L63: ""∈dict? → ✓
│ L64: results.push("")
│ L65: return
│ L68: NEVER REACHED (already returned)
RESULT: [""], |results|=1=0! ✓ (0!=1)

---
EXAMPLE 4: s="abc", dict={"bca"}
chars=['a','b','c'], len=3, results=[]

CALL₀(0):
│ L68: i∈{0,1,2}
│ i=0: swap(0,0)→['a','b','c']
│   CALL₁(1): i∈{1,2}
│     i=1: swap(1,1)→['a','b','c']
│       CALL₂(2): i∈{2}
│         i=2: swap(2,2)
│           CALL₃(3): 3==3✓ → s="abc" → "abc"∈dict? ✗ → skip
│     i=2: swap(1,2)→['a','c','b']
│       CALL₂(2): CALL₃(3): s="acb" → ✗
│       swap(1,2)→['a','b','c']
│ i=1: swap(0,1)→['b','a','c'] ← SURPRISE: 'b' now at position 0
│   CALL₁(1): i∈{1,2}
│     i=1: swap(1,1)→['b','a','c']
│       CALL₂(2): CALL₃(3): s="bac" → ✗
│     i=2: swap(1,2)→['b','c','a']
│       CALL₂(2): CALL₃(3): s="bca" → "bca"∈dict? ✓ → push!
│       swap(1,2)→['b','a','c']
│   swap(0,1)→['a','b','c']
│ i=2: swap(0,2)→['c','b','a']
│   ... s="cba"✗, s="cab"✗
│   swap(0,2)→['a','b','c']
RESULT: ["bca"], found at call depth sequence 0→1→2→3

---
EXAMPLE 5: s="aab", dict={"aab","aba","baa"} (duplicates!)
chars=['a','a','b'], len=3, results=[]

CALL₀(0): i∈{0,1,2}
│ i=0: swap(0,0)→['a','a','b']
│   CALL₁(1): i∈{1,2}
│     i=1: swap(1,1)→['a','a','b'] → CALL₂→CALL₃: s="aab"✓ → push
│     i=2: swap(1,2)→['a','b','a'] → CALL₂→CALL₃: s="aba"✓ → push
│ i=1: swap(0,1)→['a','a','b'] ← SURPRISE: same as i=0! chars[0]=chars[1]='a'
│   CALL₁(1): same subtree → s="aab"✓, s="aba"✓ → DUPLICATES!
│ i=2: swap(0,2)→['b','a','a']
│   CALL₁(1):
│     i=1: swap(1,1)→['b','a','a'] → s="baa"✓ → push
│     i=2: swap(1,2)→['b','a','a'] ← SURPRISE: chars[1]=chars[2]='a', no change!
│       s="baa"✓ → push AGAIN → DUPLICATE!
RESULT: ["aab","aba","aab","aba","baa","baa"] → 6 items, but only 3 unique!
∴ naive swap generates n!=6 outputs, unique=n!/k!=3

---
EXAMPLE 6: s="xyz", dict={"cat","dog"}
chars=['x','y','z'], len=3, results=[]

CALL₀→CALL₁→CALL₂→CALL₃: s="xyz" → ✗
... (all 6 permutations)
s="xyz"✗, s="xzy"✗, s="yxz"✗, s="yzx"✗, s="zxy"✗, s="zyx"✗
RESULT: [] ← SURPRISE: 6 calls to dict.contains(), 0 matches, empty output

---
EXAMPLE 7: s="abcd", dict={"dcba"} (4! = 24 permutations)
chars=['a','b','c','d'], len=4

Call tree depth: 0→1→2→3→4 (5 levels for len=4)
Total CALL₄ invocations = 4! = 24
Total swap operations = 2×(4+4×3+4×3×2+4×3×2×1) = 2×(4+12+24+24) = 2×64 = 128

Trace to find "dcba":
CALL₀(0) i=3: swap(0,3)→['d','b','c','a']
  CALL₁(1) i=3: swap(1,3)→['d','a','c','b']
    CALL₂(2) i=3: swap(2,3)→['d','a','b','c']
      CALL₃(3) i=3: swap(3,3)→['d','a','b','c']
        CALL₄(4): s="dabc" → ✗
... many more paths...
CALL₀(0) i=3: swap(0,3)→['d','b','c','a']
  CALL₁(1) i=2: swap(1,2)→['d','c','b','a']
    CALL₂(2) i=2: swap(2,2)→['d','c','b','a']
      CALL₃(3) i=3: swap(3,3)→['d','c','b','a']
        CALL₄(4): s="dcba" → "dcba"∈dict? ✓ → push!
RESULT: ["dcba"], found after 24 base-case checks

---
COUNTING SURPRISES:

n=1: calls=1, swaps=0, base_checks=1
n=2: calls=1+2+2=5, swaps=2×2=4, base_checks=2
n=3: calls=1+3+3×2+3×2=1+3+6+6=16, swaps=2×(3+6+6)=30, base_checks=6
n=4: calls=1+4+4×3+4×3×2+4×3×2=1+4+12+24+24=65, swaps=2×64=128, base_checks=24

Formula verification:
calls(n) = Σᵢ₌₀ⁿ n!/(n-i)! = n!×Σᵢ₌₀ⁿ 1/(n-i)! ≈ n!×e
calls(3) = 3!×(1/3! + 1/2! + 1/1! + 1/0!) = 6×(1/6+1/2+1+1) = 6×(0.17+0.5+1+1) = 6×2.67 ≈ 16 ✓

---
MEMORY TRACE at deepest point s="abc":

Stack (grows down):
┌─────────────────────────────┐ 0x200
│ CALL₀: start=0, i=0         │
├─────────────────────────────┤ 0x220
│ CALL₁: start=1, i=1         │
├─────────────────────────────┤ 0x240
│ CALL₂: start=2, i=2         │
├─────────────────────────────┤ 0x260
│ CALL₃: start=3 (base case)  │←SP
└─────────────────────────────┘

Heap:
0x100: chars = ['a','b','c'] ← shared across all frames
0x300: results = ["abc"] ← accumulator

∴ stack depth = n+1 = 4 frames for n=3
∴ space = O(n) stack + O(n!×n) results



