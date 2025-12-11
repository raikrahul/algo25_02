# 26. String Permutation Dictionary Lookup

STEP 01: chars=['a','b','c']@0x100, start=0, len=3, dict={"abc","bca","cab"}, results=[]@0x200 → L61: 0==3? → ✗ → skip base case → L68: for i in 0..3 → i∈{0,1,2}
STEP 02: i=0 at depth=0 → L69: swap(0,0) → chars=['a','b','c'] (no change) → L70: call backtrack(start=1) → push frame₁ onto stack
STEP 03: frame₁: start=1, len=3 → L61: 1==3? → ✗ → L68: for i in 1..3 → i∈{1,2}
STEP 04: i=1 at depth=1 → L69: swap(1,1) → chars=['a','b','c'] (no change) → L70: call backtrack(start=2) → push frame₂
STEP 05: frame₂: start=2, len=3 → L61: 2==3? → ✗ → L68: for i in 2..3 → i∈{2}
STEP 06: i=2 at depth=2 → L69: swap(2,2) → chars=['a','b','c'] → L70: call backtrack(start=3) → push frame₃
STEP 07: frame₃: start=3, len=3 → L61: 3==3? → ✓ BASE CASE → L62: s = chars.iter().collect() = "abc" → L63: "abc"∈dict? → ✓ → L64: results.push("abc") → results=["abc"] → L65: return → pop frame₃
STEP 08: back in frame₂ → L71: swap(2,2) → chars=['a','b','c'] (restore) → loop ends (i=2 was last) → return → pop frame₂
STEP 09: back in frame₁ → L71: swap(1,1) → chars=['a','b','c'] (restore) → next iteration i=2
STEP 10: i=2 at depth=1 → L69: swap(1,2) → chars=['a','c','b'] MUTATED! → L70: call backtrack(start=2) → push frame₂'
STEP 11: frame₂': start=2, len=3 → L68: i=2 → swap(2,2) → L70: call backtrack(start=3) → push frame₃'
STEP 12: frame₃': start=3 → L61: 3==3? → ✓ → L62: s="acb" → L63: "acb"∈dict? → ✗ → skip L64 → L65: return → pop frame₃'
STEP 13: back in frame₂' → L71: swap(2,2) → return → pop frame₂' → back in frame₁ → L71: swap(1,2) → chars=['a','b','c'] RESTORED! → loop ends → return → pop frame₁
STEP 14: back in frame₀ → L71: swap(0,0) → chars=['a','b','c'] → next iteration i=1
STEP 15: i=1 at depth=0 → L69: swap(0,1) → chars=['b','a','c'] MUTATED! → L70: call backtrack(start=1) → push frame₁''
STEP 16: frame₁'': start=1 → L68: i∈{1,2} → i=1: swap(1,1) → call(start=2) → call(start=3) → s="bac" → "bac"∈dict? → ✗
STEP 17: frame₁'': i=2: swap(1,2) → chars=['b','c','a'] → call(start=2) → call(start=3) → s="bca" → "bca"∈dict? → ✓ → results.push("bca") → results=["abc","bca"]
STEP 18: L71: swap(1,2) → chars=['b','a','c'] → return to frame₀ → L71: swap(0,1) → chars=['a','b','c'] RESTORED!
STEP 19: i=2 at depth=0 → L69: swap(0,2) → chars=['c','b','a'] → call backtrack(start=1)
STEP 20: i=1: swap(1,1) → call(2) → call(3) → s="cba" → ✗ → i=2: swap(1,2) → chars=['c','a','b'] → call(2) → call(3) → s="cab" → "cab"∈dict? → ✓ → results.push("cab") → results=["abc","bca","cab"]
STEP 21: L71: swap(1,2) → chars=['c','b','a'] → L71: swap(0,2) → chars=['a','b','c'] RESTORED! → loop ends → return from frame₀
STEP 22: FINAL: results=["abc","bca","cab"], |results|=3, verify 3 matches found in dict of 3 target words, all 6 permutations checked, time=O(3!×3)=O(18)

---
TRACE s="ab", dict={"ab","ba"}:
STEP 01: chars=['a','b'], start=0, len=2 → L61: 0==2? ✗ → L68: i∈{0,1}
STEP 02: i=0: swap(0,0)→['a','b'] → call(1) → i=1: swap(1,1) → call(2) → 2==2✓ → s="ab"∈dict✓ → push → results=["ab"]
STEP 03: i=1 at depth=0: swap(0,1)→['b','a'] → call(1) → i=1: swap(1,1) → call(2) → s="ba"∈dict✓ → push → results=["ab","ba"]
STEP 04: swap(0,1)→['a','b'] RESTORED → return → FINAL: ["ab","ba"]

---
TRACE s="a", dict={"a"}:
STEP 01: chars=['a'], start=0, len=1 → L61: 0==1? ✗ → L68: i=0
STEP 02: i=0: swap(0,0) → call(1) → 1==1✓ → s="a"∈dict✓ → push → FINAL: ["a"]

---
TRACE s="", dict={""}:
STEP 01: chars=[], start=0, len=0 → L61: 0==0? ✓ → s=""∈dict✓ → push → FINAL: [""]

---
TRACE s="aab", dict={"aab","aba","baa"} (DUPLICATES):
STEP 01: chars=['a','a','b'], start=0 → L68: i∈{0,1,2}
STEP 02: i=0: swap(0,0)→['a','a','b'] → subtree: s="aab"✓,s="aba"✓ → results=["aab","aba"]
STEP 03: i=1: swap(0,1)→['a','a','b'] SAME AS i=0! chars[0]=chars[1]='a' → subtree: s="aab"✓,s="aba"✓ → results=["aab","aba","aab","aba"] DUPLICATES!
STEP 04: i=2: swap(0,2)→['b','a','a'] → subtree: s="baa"✓,s="baa"✓ → results=[...,"baa","baa"] MORE DUPLICATES!
STEP 05: FINAL: 6 items in results, only 3 unique → FIX: use HashSet or skip when chars[i]==chars[start] for i>start

---
COUNTING:
n=0 → 0!=1 → 1 base check → 0 swaps
n=1 → 1!=1 → 1 base check → 0 real swaps (swap(0,0) is no-op)
n=2 → 2!=2 → 2 base checks → 2 swaps (swap(0,1) and swap(0,1) back)
n=3 → 3!=6 → 6 base checks → 2×(3+6+6)=30 swap operations
n=4 → 4!=24 → 24 base checks → 2×64=128 swap operations

---
MEMORY at s="abc" depth=3:
Stack: frame₀(start=0)→frame₁(start=1)→frame₂(start=2)→frame₃(start=3,BASE)
Heap: chars@0x100=['a','b','c'] shared, results@0x200=["abc"]
Space: O(n) stack + O(n!×n) results

---
FULL CALL TRACE (each line = step, call number, params, action):

INPUT: chars=['a','b','c'], dict={"abc","bca","cab"}, results=[]

STEP01, CALL₁, backtrack(chars=['a','b','c'], start=0, dict, results=[]), L68: 0==3? → ✗, L75: for i in 0..3, i=0: L76: swap(0,0)→['a','b','c'], L77: call backtrack(start=1)
STEP02, CALL₂, backtrack(chars=['a','b','c'], start=1, dict, results=[]), L68: 1==3? → ✗, L75: for i in 1..3, i=1: L76: swap(1,1)→['a','b','c'], L77: call backtrack(start=2)
STEP03, CALL₃, backtrack(chars=['a','b','c'], start=2, dict, results=[]), L68: 2==3? → ✗, L75: for i in 2..3, i=2: L76: swap(2,2)→['a','b','c'], L77: call backtrack(start=3)
STEP04, CALL₄, backtrack(chars=['a','b','c'], start=3, dict, results=[]), L68: 3==3? → ✓, L69: s="abc", L70: "abc"∈dict? → ✓, L71: results.push("abc"), results=["abc"], L72: return
STEP05, CALL₃ resumed, L78: swap(2,2)→['a','b','c'], loop ends (i=2 was last), return
STEP06, CALL₂ resumed, L78: swap(1,1)→['a','b','c'], i=2: L76: swap(1,2)→['a','c','b'], L77: call backtrack(start=2)
STEP07, CALL₅, backtrack(chars=['a','c','b'], start=2, dict, results=["abc"]), L68: 2==3? → ✗, L75: i=2: swap(2,2), L77: call backtrack(start=3)
STEP08, CALL₆, backtrack(chars=['a','c','b'], start=3, dict, results=["abc"]), L68: 3==3? → ✓, L69: s="acb", L70: "acb"∈dict? → ✗, skip L71, L72: return
STEP09, CALL₅ resumed, L78: swap(2,2), loop ends, return
STEP10, CALL₂ resumed, L78: swap(1,2)→['a','b','c'], loop ends (i=2 was last), return
STEP11, CALL₁ resumed, L78: swap(0,0)→['a','b','c'], i=1: L76: swap(0,1)→['b','a','c'], L77: call backtrack(start=1)
STEP12, CALL₇, backtrack(chars=['b','a','c'], start=1, dict, results=["abc"]), L68: 1==3? → ✗, L75: i=1: swap(1,1)→['b','a','c'], L77: call backtrack(start=2)
STEP13, CALL₈, backtrack(chars=['b','a','c'], start=2, dict, results=["abc"]), L68: 2==3? → ✗, L75: i=2: swap(2,2), L77: call backtrack(start=3)
STEP14, CALL₉, backtrack(chars=['b','a','c'], start=3, dict, results=["abc"]), L68: 3==3? → ✓, L69: s="bac", L70: "bac"∈dict? → ✗, skip L71, L72: return
STEP15, CALL₈ resumed, L78: swap(2,2), return
STEP16, CALL₇ resumed, L78: swap(1,1), i=2: L76: swap(1,2)→['b','c','a'], L77: call backtrack(start=2)
STEP17, CALL₁₀, backtrack(chars=['b','c','a'], start=2, dict, results=["abc"]), L68: 2==3? → ✗, L75: i=2: swap(2,2), L77: call backtrack(start=3)
STEP18, CALL₁₁, backtrack(chars=['b','c','a'], start=3, dict, results=["abc"]), L68: 3==3? → ✓, L69: s="bca", L70: "bca"∈dict? → ✓, L71: push("bca"), results=["abc","bca"], L72: return
STEP19, CALL₁₀ resumed, L78: swap(2,2), return
STEP20, CALL₇ resumed, L78: swap(1,2)→['b','a','c'], return
STEP21, CALL₁ resumed, L78: swap(0,1)→['a','b','c'], i=2: L76: swap(0,2)→['c','b','a'], L77: call backtrack(start=1)
STEP22, CALL₁₂, backtrack(chars=['c','b','a'], start=1, dict, results=["abc","bca"]), L68: 1==3? → ✗, L75: i=1: swap(1,1), L77: call backtrack(start=2)
STEP23, CALL₁₃, backtrack(chars=['c','b','a'], start=2, dict, results=["abc","bca"]), L68: 2==3? → ✗, L75: i=2: swap(2,2), L77: call backtrack(start=3)
STEP24, CALL₁₄, backtrack(chars=['c','b','a'], start=3, dict, results=["abc","bca"]), L68: 3==3? → ✓, L69: s="cba", L70: "cba"∈dict? → ✗, return
STEP25, CALL₁₃ resumed, return
STEP26, CALL₁₂ resumed, L78: swap(1,1), i=2: swap(1,2)→['c','a','b'], L77: call backtrack(start=2)
STEP27, CALL₁₅, backtrack(chars=['c','a','b'], start=2, dict, results=["abc","bca"]), L68: 2==3? → ✗, i=2: swap(2,2), L77: call backtrack(start=3)
STEP28, CALL₁₆, backtrack(chars=['c','a','b'], start=3, dict, results=["abc","bca"]), L68: 3==3? → ✓, L69: s="cab", L70: "cab"∈dict? → ✓, L71: push("cab"), results=["abc","bca","cab"], return
STEP29, CALL₁₅ resumed, return
STEP30, CALL₁₂ resumed, L78: swap(1,2)→['c','b','a'], return
STEP31, CALL₁ resumed, L78: swap(0,2)→['a','b','c'], loop ends (i=2 was last), return

OUTPUT: results=["abc","bca","cab"], 16 calls total, 6 base cases, 3 matches

---
ERRORS MADE:

E1. L63: dict.contains(&s) → dictionary.contains(&s)
    wrote: dict
    param: dictionary
    sloppy: autopilot from comment
    missed: fn signature 3 lines above
    prevent: read signature before body

E2. L88: let chars: &mut Vec<char> = collect()
    wrote: &mut Vec<char> (reference)
    collect(): Vec<char> (owned)
    sloppy: confused reference vs owned
    missed: collect() creates new value
    prevent: trace type: iter→collect→Vec

E3. L88-91: no return statement
    wrote: backtrack(...);
    needed: backtrack(...); results
    sloppy: forgot implicit return
    missed: fn → Vec<String>
    prevent: write return first

BRAIN QUESTIONS:
Q1: dict vs dictionary → copied from comment, not from signature
Q2: &mut Vec vs Vec → assumed reference because fn takes &mut
Q3: no return → wrote top-to-bottom, forgot output
PATTERN: all 3 = not reading current context, using cached patterns

ORTHOGONAL:
E1 = lexical (wrong name)
E2 = type (reference vs owned)
E3 = structural (missing expression)
3 categories, 3 compiler stages, 0 logic errors

