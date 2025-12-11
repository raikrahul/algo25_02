01. Write 5 variables on paper: n1=1, n2=2, n3=3, n4=4, n5=5, these are node values, nothing else yet.
02. Assign memory addresses: n1→0x100, n2→0x110, n3→0x120, n4→0x130, n5→0x140, write these 5 pairs (value,addr) in a column.
03. Draw arrows from step 02 data: 0x100.left=0x110, 0x100.right=0x120, 0x110.left=0x130, 0x110.right=0x140, 0x120.left=∅, 0x120.right=∅, 0x130.left=∅, 0x130.right=∅, 0x140.left=∅, 0x140.right=∅.
04. You now have a tree on paper, verify: 0x100 has 2 children, 0x110 has 2 children, 0x120,0x130,0x140 have 0 children, total edges=4, nodes=5, edges=nodes-1 ✓.
05. Pick P=0x130, Q=0x140, circle these two addresses on your paper from step 02.
06. From step 03, trace 0x130 upward: 0x130 is left child of 0x110 (from step 03: 0x110.left=0x130), write 0x130→0x110.
07. From step 03, trace 0x110 upward: 0x110 is left child of 0x100 (from step 03: 0x100.left=0x110), append to step 06: 0x130→0x110→0x100, this is path(P).
08. From step 03, trace 0x140 upward: 0x140 is right child of 0x110 (from step 03: 0x110.right=0x140), write 0x140→0x110.
09. From step 03, trace 0x110 upward: 0x110 is left child of 0x100 (from step 03: 0x100.left=0x110), append to step 08: 0x140→0x110→0x100, this is path(Q).
10. Compare path(P) from step 07 and path(Q) from step 09: path(P)={0x130,0x110,0x100}, path(Q)={0x140,0x110,0x100}, common={0x110,0x100}.
11. From step 10, which common address appears first when traversing from leaf? 0x110 appears at position 1 in both paths, 0x100 appears at position 2, first common traversing up = 0x110, this is LCA by definition.
12. Open 24_lca.rs, find `pub fn find_lca`, cursor at line 85, function body is `todo!()`.
13. From step 12, first question: what should happen if t is null (tree empty or reached leaf)? From step 03, leaf nodes have .left=∅, when you call find_lca(∅,...) what should return? Think: if no tree, no ancestor exists, return ∅.
14. Write block 1 in find_lca: `if t.is_none() { return None; }` — this handles step 13, now t is guaranteed Some(...).
15. From step 05, P=0x130, Q=0x140, from step 11, LCA=0x110, question: when you reach node 0x130 in recursion, what should happen? 0x130==P, you found P, should you continue searching children of 0x130? Children of 0x130 from step 03: both ∅, searching them returns ∅, useless work.
16. From step 15, when current node equals P or Q, stop and return current node, no need to search children, write this observation next to step 15 on paper.
17. Write block 2 in find_lca: `let node = t.as_ref().unwrap();` — unwrap is safe because step 14 already returned if None, now node is Rc<RefCell<TreeNode>>.
18. Write block 3: `if Rc::ptr_eq(node, p.as_ref().unwrap_or(&Rc::new(RefCell::new(TreeNode::new(-1)))))` — wait, p might be None? Read problem: "p and q point to valid nodes", so p is always Some, simplify: `if Rc::ptr_eq(node, p.as_ref().unwrap()) { return t.clone(); }`.
19. Write block 4 same pattern for q: `if Rc::ptr_eq(node, q.as_ref().unwrap()) { return t.clone(); }` — from step 16, return immediately on match.
20. From step 03, if current node is not P and not Q, what next? Must check if P or Q is in left subtree, must check if P or Q is in right subtree, two separate checks needed.
21. Write block 5: `let left_result = find_lca(&node.borrow().left, p, q);` — recurse into left subtree using left pointer from step 03, this returns Option<Rc<...>>.
22. Write block 6: `let right_result = find_lca(&node.borrow().right, p, q);` — recurse into right subtree using right pointer from step 03.
23. From step 21 and 22, you now have left_result and right_result, both are Option<Rc<...>>, question: what combinations are possible? (Some,Some), (Some,None), (None,Some), (None,None).
24. From step 10, when P in left subtree (0x130∈{0x130,0x110}) and Q in right subtree (0x140∈{0x140,0x110}), both found, current node is LCA, from trace: at 0x110, left_result=Some(0x130), right_result=Some(0x140), 0x110 is LCA.
25. Write block 7: `if left_result.is_some() && right_result.is_some() { return t.clone(); }` — from step 24, current is LCA when both subtrees return non-null.
26. From step 23, case (Some,None): P or Q found in left, nothing in right, what to return? From trace at 0x100: left_result=Some(0x110)=LCA already, right_result=None, should return left_result to propagate LCA upward.
27. Write block 8: `if left_result.is_some() { return left_result; }` — from step 26, pass left result up.
28. From step 23, case (None,Some): symmetric to step 26, P or Q found in right only.
29. Write block 9: `if right_result.is_some() { return right_result; }` — from step 28, pass right result up.
30. From step 23, case (None,None): neither P nor Q in this subtree, return None.
31. Write block 10: `None` — final return, from step 30.
32. Combine blocks 1-10, remove `todo!()`, save file, run `cargo test --bin 24_lca`, observe output.
---
F1. If you wrote blocks 5,6 BEFORE blocks 2,3,4 (check current before recurse), in ancestor case P=0x110,Q=0x130, you would recurse into 0x130,0x140 before checking 0x110==P, wasted work, from step 15-16 checking current first avoids this.
F2. If block 7 returned left_result instead of t.clone(), you return P not LCA, from step 24 current node is the split point, must return current.
F3. If you forgot block 1 (null check), calling .as_ref().unwrap() on None panics, from step 03 leaf children are ∅.
F4. If you used == instead of Rc::ptr_eq, comparing Rc compares addresses not contents, but == on Rc<RefCell<TreeNode>> compares inner values if PartialEq implemented, you want address equality, use ptr_eq from step 05.
---
VERIFY AXIOMATIC:
Step 01: introduced n1-n5 ← from problem "binary tree with nodes".
Step 02: introduced 0x100-0x140 ← arbitrary addresses, offset 0x10 chosen for readability.
Step 03: introduced .left/.right ← from TreeNode struct in 24_lca.rs line 22-24.
Step 05: introduced P,Q ← from problem "nodes p and q".
Step 06-11: derived path() and LCA ← from step 03 arrows, no new concepts.
Step 13-31: each block derived from previous step observation.
No jumping ahead. ✓
No new variables without derivation. ✓

---
ERROR REPORT (2025-12-11):
E1. Line 107: `p.as_ref().unwrap` → missing () → `p.as_ref().unwrap()` | sloppy: typed method as field | missed: method syntax requires () | prevent: read compiler error "expected function, found method"
E2. Line 107 initial: copied block 3 template but forgot () on unwrap | sloppy: pattern matching without verification | missed: Rust method call syntax | prevent: compile after each block, not after all blocks
QUESTIONS FOR SLOPPY BRAIN:
Q1. Did you compile after writing block 3? If yes, you would catch E1 immediately.
Q2. Did you read the compiler error message? It says "expected function, found `&dyn Fn...`" pointing at unwrap.
Q3. Did you compare your block 3 to block 4? Block 4 had correct unwrap(), block 3 did not.
ORTHOGONAL THOUGHT:
You typed `unwrap` without () because your brain saw "unwrap" as a property not a method.
Rust has no properties, only methods.
Every access that returns a value is a method call requiring ().
If you see `x.foo` returning value, it is `x.foo()`.
TEST RESULT: 10/10 passed ✓
