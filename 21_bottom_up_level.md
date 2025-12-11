01. draw T=[3,[4,[5,∅,∅],[1,∅,∅]],[7,[6,∅,∅],[8,∅,∅]]] → 7 boxes, arrows L/R, root=3 at top
02. write levels by hand: count depth from root → L₀={3}, L₁={4,7}, L₂={5,1,6,8} → verify |L₀|=1, |L₁|=2, |L₂|=4, total=7
03. write expected output: reverse level order → L₂ first, L₁ second, L₀ last → [5,1,6,8,4,7,3] → verify length=7 ✓
04. draw empty Q box → write Q=[] → draw empty S box → write S=[]
05. push root into Q → Q=[3] → |Q|=1 → write this on paper
06. save |Q|=1 before loop → loop_count=1 → pop 3 → write 3 into level_vec=[] → level_vec=[3]
07. check 3.left → 3.left=4≠∅ → push 4 into Q → Q=[4]
08. check 3.right → 3.right=7≠∅ → push 7 into Q → Q=[4,7]
09. loop done (1 iteration) → push level_vec=[3] into S → S=[[3]] → verify |S|=1
10. save |Q|=2 before loop → loop_count=2 → iter₁: pop 4 → level_vec=[4]
11. check 4.left → 4.left=5≠∅ → push 5 → Q=[7,5]
12. check 4.right → 4.right=1≠∅ → push 1 → Q=[7,5,1]
13. iter₂: pop 7 → level_vec=[4,7]
14. check 7.left → 7.left=6≠∅ → push 6 → Q=[5,1,6]
15. check 7.right → 7.right=8≠∅ → push 8 → Q=[5,1,6,8]
16. loop done (2 iterations) → push level_vec=[4,7] into S → S=[[3],[4,7]] → verify |S|=2
17. save |Q|=4 before loop → loop_count=4 → iter₁: pop 5 → level_vec=[5] → 5.left=∅, 5.right=∅ → no push
18. iter₂: pop 1 → level_vec=[5,1] → 1.left=∅, 1.right=∅ → no push
19. iter₃: pop 6 → level_vec=[5,1,6] → 6.left=∅, 6.right=∅ → no push
20. iter₄: pop 8 → level_vec=[5,1,6,8] → 8.left=∅, 8.right=∅ → no push
21. loop done (4 iterations) → push level_vec=[5,1,6,8] into S → S=[[3],[4,7],[5,1,6,8]] → verify |S|=3
22. check Q → Q=[] → |Q|=0 → outer loop terminates
23. pop S → S.pop()=[5,1,6,8] → print 5,1,6,8 → S=[[3],[4,7]]
24. pop S → S.pop()=[4,7] → print 4,7 → S=[[3]]
25. pop S → S.pop()=[3] → print 3 → S=[]
26. verify output=[5,1,6,8,4,7,3] → matches line_03 expected ✓
27. draw T=∅ → Q=[] → |Q|=0 → outer loop never runs → S=[] → output=[] → verify empty ✓
28. draw T=[1,∅,∅] → Q=[1] → |Q|=1 → pop 1 → 1.left=∅, 1.right=∅ → level_vec=[1] → S=[[1]] → Q=[] → S.pop()=[1] → output=[1] ✓
29. draw T=[1,[2,∅,∅],∅] → Q=[1] → pop 1 → push 2 → S=[[1]] → Q=[2] → pop 2 → S=[[1],[2]] → S.pop()=[2] → S.pop()=[1] → output=[2,1] ✓
30. draw T=[1,[2,∅,∅],[3,∅,∅]] → Q=[1] → pop 1 → push 2,3 → S=[[1]] → Q=[2,3] → |Q|=2 → pop 2, pop 3 → S=[[1],[2,3]] → output=[2,3,1] ✓
31. try wrong approach: BFS_flat=[3,4,7,5,1,6,8] → reverse=[8,6,1,5,7,4,3] → expected=[5,1,6,8,4,7,3] → 8≠5 at index 0 ✗ → flat reverse fails
32. try wrong approach: forget |Q| before loop → pop all at once → levels merge → [3,4,7,5,1,6,8] → no level boundary → wrong ✗
33. try wrong approach: forget S.push(level_vec) → S stays empty → output=[] → expected≠[] ✗
34. draw memory: 0x100=[3,L→0x110,R→0x120], 0x110=[4,L→0x130,R→0x140], 0x120=[7,L→0x150,R→0x160], 0x130=[5,∅,∅], 0x140=[1,∅,∅], 0x150=[6,∅,∅], 0x160=[8,∅,∅]
35. trace with addresses: Q=[0x100] → pop 0x100 → 0x100.L=0x110 → push 0x110 → 0x100.R=0x120 → push 0x120 → Q=[0x110,0x120]
36. count operations: each node pushed once (7) + each node popped once (7) = 14 → O(N) time ✓
37. count space: Q max width = 4 (level 2) + S stores all 7 nodes = O(N) space ✓
38. draw skew left T=[1,[2,[3,[4,∅,∅],∅],∅],∅] → L₀={1}, L₁={2}, L₂={3}, L₃={4} → output=[4,3,2,1] → Q max width=1 → trace by hand

---
axiomatic_check: line_01→03 define input/output from problem statement ✓
axiomatic_check: line_04→26 trace algorithm step by step, no jumps ✓
axiomatic_check: line_27→30 edge cases derived same way ✓
axiomatic_check: line_31→33 failures derived by negating correct steps ✓
axiomatic_check: line_34→37 memory/complexity derived from counts in prior lines ✓
new_things_introduced: S (stack) introduced in line_04 as storage, used from line_09 onward ✓
jumping_ahead: none, each line follows from previous ✓
