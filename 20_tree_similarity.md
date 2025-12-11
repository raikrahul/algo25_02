# Tree Similarity
⚠ line N uses only {line 1..N-1}
⚠ no new symbol without prior derivation
⚠ no new formula without prior calculation
⚠ each line: derive from previous lines only
01. input=(t1,t2) output={✓,✗}
02. t1∈{∅,Some} (2 states)
03. t2∈{∅,Some} (2 states)
04. (t1,t2)∈{∅,Some}×{∅,Some} = 2×2 = 4 cases
05. case₁: (∅,∅)
06. case₂: (∅,Some)
07. case₃: (Some,∅)
08. case₄: (Some,Some)
09. case₁: ∅=∅ → ✓ (nothing≠nothing)
10. case₂: ∅≠Some → ✗ (nothing≠something)
11. case₃: Some≠∅ → ✗ (something≠nothing)
12. case₄: Some=Some → ? (need more info)
13. Some=[val,left,right]
14. left∈{∅,Some}, right∈{∅,Some}
15. case₄ → compare(t1.left,t2.left) ∧ compare(t1.right,t2.right)
16. ∴ case₄ = f(t1.left,t2.left) ∧ f(t1.right,t2.right)
17. f = recursive (calls itself on subtrees)
18. 0x100:[val=5,left=∅,right=∅]
19. 0x100.val=5, 0x100.left=∅, 0x100.right=∅
20. f(∅,∅): case₁ → ✓
21. f(∅,0x100): case₂ → ✗
22. f(0x100,∅): case₃ → ✗
23. 0x100:[7,∅,∅] 0x200:[99,∅,∅]
24. f(0x100,0x200): case₄ → f(∅,∅)∧f(∅,∅)
25. f(∅,∅)=✓ (line 20)
26. f(∅,∅)=✓ (line 20)
27. ✓∧✓=✓
28. 0x100:[1,0x108,0x110]
29. 0x108:[2,∅,∅] 0x110:[3,∅,∅]
30. 0x200:[9,0x208,0x210]
31. 0x208:[8,∅,∅] 0x210:[7,∅,∅]
32. f(0x100,0x200): case₄
33. → f(0x108,0x208) ∧ f(0x110,0x210)
34. f(0x108,0x208): case₄
35. → f(∅,∅) ∧ f(∅,∅)
36. → ✓ ∧ ✓ = ✓ (lines 20,20)
37. f(0x110,0x210): case₄
38. → f(∅,∅) ∧ f(∅,∅)
39. → ✓ ∧ ✓ = ✓ (lines 20,20)
40. f(0x100,0x200) = ✓ ∧ ✓ = ✓
41. F1: 5≠9 → ✗? NO! line 15: compare structure, not val
42. F2: f(left)=✓ → return ✓? NO! line 15: need ∧, not early return
43. F3: check case₂ before case₁? NO! f(∅,∅) would hit case₂ first → ✗ wrong
44. F4: f(left) ∨ f(right)? NO! line 15: ∧ not ∨
45. 0x100:[A,0x108,∅] 0x200:[B,∅,0x210]
46. 0x108:[C,∅,∅] 0x210:[D,∅,∅]
47. f(0x100,0x200): case₄
48. → f(0x108,∅) ∧ f(∅,0x210)
49. f(0x108,∅): case₃ → ✗ (line 11)
50. f(∅,0x210): case₂ → ✗ (line 10)
51. ✗ ∧ ✗ = ✗
52. 0x100:[R,0x108,∅] 0x200:[R,0x208,0x210]
53. 0x108:[L,∅,∅] 0x208:[L,∅,∅] 0x210:[R,∅,∅]
54. f(0x100,0x200): case₄
55. → f(0x108,0x208) ∧ f(∅,0x210)
56. f(0x108,0x208): case₄ → f(∅,∅)∧f(∅,∅) = ✓∧✓ = ✓
57. f(∅,0x210): case₂ → ✗ (line 10)
58. ✓ ∧ ✗ = ✗
59. n₁=nodes(t1), n₂=nodes(t2)
60. each f() visits 1 node from each tree
61. max visits = min(n₁,n₂)
62. ∴ T = O(min(n₁,n₂))
63. h₁=height(t1), h₂=height(t2)
64. recursion depth = min(h₁,h₂)
65. ∴ S = O(min(h₁,h₂))
66. E1: (0,0) → case₁ → ✓
67. E2: (0,1) → case₂ → ✗
68. E3: (1,0) → case₃ → ✗
69. E4: (1,1) → case₄ → f(∅,∅)∧f(∅,∅) = ✓
70. E5: (←←←,→→→) left-skew vs right-skew → case₄ → f(Some,∅)∧f(∅,Some) = ✗∧✗ = ✗
71. (10⁶,10⁶,=) → O(n)
72. (10⁶,10⁶,≠@root) → O(1)

---

## ERRORS MADE

E1. line 84: `(None,None) => return false` → should be `true`
- sloppy: confused ∅=∅ with ∅≠∅
- missed: both empty = same structure
- prevent: trace (∅,∅) before writing

E2. line 84: `return true` → should be `true,`
- sloppy: mixed statement with expression
- missed: match arms need `,` not `;`
- prevent: rust match = expression, no `return`

E3. line 85-87: `return false;` → should be `false,`
- sloppy: copy-paste from line 84
- missed: same syntax error repeated
- prevent: fix pattern once, apply everywhere

E4. line 91: `lef1` → should be `left1`
- sloppy: typo, missing `t`
- missed: variable name misspelled
- prevent: compiler catches unused variable warning

E5. line 93: `t2.borrow().right` → should be `t1.borrow().right`
- sloppy: copy-paste from wrong line
- missed: right1 must come from t1, not t2
- prevent: draw which tree each variable comes from

E6. line 85: `Some(t2)` → should be `Some(_)`
- sloppy: shadowing outer t2
- missed: unused binding in pattern
- prevent: use `_` when not using captured value

---

## TRACE YOUR ERRORS

```
YOUR THOUGHT: (None,None) = one is None → false
CORRECT: (None,None) = both are None = ∅=∅ = same = true

YOUR THOUGHT: right1 = t2.right (copy from left2 line)
CORRECT: right1 = t1.right (stays on t1 tree)

DRAW:
t1:[left1,right1]    t2:[left2,right2]
     ↑        ↑           ↑        ↑
     t1       t1          t2       t2
```
