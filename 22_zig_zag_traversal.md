Step 01. Input Data: Root=3 [0x100]. Children: L=4 [0x200], R=7 [0x300]. 4.L=5, 4.R=1. 7.L=6, 7.R=8. All others=NULL.
Step 02. Definition ZigZag: L0(L→R) → L1(R→L) → L2(L→R) → Alternate.
Step 03. Manual Trace L0 (L→R): Nodes=[3]. Output requires: 3. ✓.
Step 04. Manual Trace L1 (R→L): Nodes=[4, 7]. Access 3.L=4, 3.R=7. Output requires: 7, 4. ✓.
Step 05. Manual Trace L2 (L→R): Nodes level 2. From 4: 5, 1. From 7: 6, 8. Layout: 5, 1, 6, 8. Output requires: 5, 1, 6, 8. ✓.
Step 06. Hypothesis 1 Queue(FIFO): Q=[3]. Pop 3. Output 3. Push L(4), R(7). Q=[4, 7].
Step 07. Queue Step 2: Pop 4. Output 4. Push 5, 1. Pop 7. Output 7. Push 6, 8. Out: 3, 4, 7. Expected: 3, 7, 4. ✗ (Order Fail).
Step 08. Fix L1: Queue pops 4, 7. Need 7, 4. ∴ Use Stack(LIFO)? Or Deque?
Step 09. Hypothesis 2 Stack: S=[3]. Pop 3. Push 4, 7. S=[4, 7]. Top=7.
Step 10. Stack S=[4, 7] → Pop 7. Output 7. Children 6, 8. Push? S=[4, 6, 8]? (Mixed Levels ✗).
Step 11. Separation: Needs 2 containers. CurrLevel, NextLevel.
Step 12. Run L0 (L→R) with 2 Stacks: S1=[3], S2=[].
Step 13. L0 Op: Pop 3. Print 3. Target L1 order: 7, 4. S2 is LIFO. ∴ Push 4 then 7. S2=[4, 7].
Step 14. L0 Rule: If Dir=L→R → Push Children L then R to NextStack.
Step 15. Check S2: Pop 7. Pop 4. Matches L1 Target. ✓.
Step 16. Run L1 (R→L): S2=[4, 7] (Top=7). S1=[].
Step 17. L1 Op 1: Pop 7. Print 7. Children 6, 8. Target L2 order: 5, 1, 6, 8.
Step 18. L2 Target Analysis: 5, 1 come from 4. 6, 8 come from 7.
Step 19. Stack S1 Logic: To pop 5 first, 5 must be pushed LAST.
Step 20. Order of processing: 7 then 4.
Step 21. Process 7: Children 6, 8. Push to S1. S1=[...].
Step 22. Process 4: Children 5, 1. Push to S1. S1=[..., 1, 5] (Since 5 is top).
Step 23. ∴ S1 state must become [8, 6, 1, 5]. (Pop order: 5, 1, 6, 8).
Step 24. Backtrack to Step 21 (Process 7): Want S1 bottom=[8, 6]. ∴ Push 8 then 6. S1=[8, 6].
Step 25. Backtrack to Step 22 (Process 4): Want S1 top=[1, 5]. ∴ Push 1 then 5. S1=[8, 6, 1, 5].
Step 26. L1 Rule: If Dir=R→L → Push Children R then L to NextStack.
Step 27. Summary Rules: L→R passes push L, R. R→L passes push R, L.
Step 28. Iteration 1 (L0): S1=[3]. Pop 3. L→R. Push 4, 7. S2=[4, 7]. Output: 3.
Step 29. Iteration 2 (L1): S2=[4, 7]. Pop 7. R→L. Push 8, 6. S1=[8, 6]. Output: 7.
Step 30. Iteration 2 cont: Pop 4. R→L. Push 1, 5. S1=[8, 6, 1, 5]. Output: 4. Total: 3, 7, 4.
Step 31. Iteration 3 (L2): S1=[8, 6, 1, 5]. Pop 5. L→R. Push children(5)? Empty. Output: 5.
Step 32. Iteration 3 cont: Pop 1. Push children? Empty. Output: 1.
Step 33. Iteration 3 cont: Pop 6. Output: 6. Pop 8. Output: 8. Total: 5, 1, 6, 8.
Step 34. Check Edge Case 1: Root=NULL. Return [].
Step 35. Check Edge Case 2: Node without children. Push nothing.
Step 36. Check Edge Case 3: Node with 1 child. Logic holds? L→R push L? If R missing, push nothing.
Step 37. Code Struct: `curr_stack`, `next_stack`, `left_to_right` bool.
Step 38. Outer Loop: while `!curr_stack.is_empty()`.
Step 39. Inner Loop: `node = curr_stack.pop()`. Print `node.val`.
Step 40. Inside Inner: if `left_to_right`: push `left`, `right`. else: push `right`, `left`.
Step 41. End Inner: Swap `curr_stack`, `next_stack`. Toggle `left_to_right`.
Step 42. Complexity Space: Max width of tree W. 2 stacks. O(W).
Step 43. Complexity Time: Visit each node once. Push/Pop O(1). Total O(N).
Step 44. Memory Addresses Trace: 3@100, 4@200, 7@300.
Step 45. Trace Step 1: S1=[100].
Step 46. Trace Step 2: Pop 100. Push 200, 300. S2=[200, 300].
Step 47. Trace Step 3: Pop 300. Push 300->R, 300->L. S1=[R8, L6].
Step 48. Trace Step 4: Pop 200. Push 200->R, 200->L. S1=[R8, L6, R1, L5].
Step 49. Verification Complete.
Step 50. Error Log 1: Logic Nesting Failure.
Step 51. Line 58 RUST: `} else {` attached to wrong `if`.
Step 52. Mistake: `if let Some(right)` ... `else` (Implicitly attached to inner `if`).
Step 53. Reality: `if left_to_right` ... `else`.
Step 54. Why: Visual indentation ignored. Brace matching ignored.
Step 55. Prevention: Count braces. Explicitly format code before writing logic.
Step 56. Error Log 2: Swap/Toggle Scope Failure.
Step 57. Line 64 RUST: `swap` inside `while let Some`.
Step 58. Mistake: Swapped stacks after EACH node.
Step 59. Reality: Must swap after ALL nodes in current level (Outer loop).
Step 60. Why: Rushing to "finish" loop. Ignored loop boundary semantics.
Step 61. Prevention: Draw loop scope box. Place actions strictly outside/inside box.
Step 62. Error Log 3: Loop Termination Failure.
Step 63. Line 66 RUST: `swap` outside ALL loops.
Step 64. Mistake: Outer loop terminates immediately (S1 becomes empty). Swap never runs or runs too late.
Step 65. Reality: Swap must happen to REFILL S1 from S2 for next outer iteration.
Step 66. Why: Forward thinking "after loop" vs "during loop".
Step 67. Prevention: Trace halting condition. If S1 empty, who refills it? S2. When? Before check.
