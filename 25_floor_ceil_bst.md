01. Draw this tree by hand on paper now: 0x100→{val:13, left:0x200, right:0x300}, 0x200→{val:9, left:0x400, right:0x500}, 0x300→{val:16, left:0x600, right:0x700}, 0x400→{val:5, left:None, right:None}, 0x500→{val:10, left:None, right:None}, 0x600→{val:14, left:None, right:None}, 0x700→{val:18, left:None, right:None}; Verify 7 nodes total; Write "x=17" next to paper.
02. Write down the set of all values in tree: {5, 9, 10, 13, 14, 16, 18}; Circle all values ≤ 17: {5, 9, 10, 13, 14, 16}; What is the maximum of circled set? Write it down. Do not proceed until you write it.
03. Your answer from step 02 is the Floor(17); You computed this by scanning ALL 7 nodes; This is O(N) brute force; Now we derive O(H) using BST structure; Do NOT look ahead.
04. Point your finger at 0x100 on paper; val=13; Is 13 ≤ 17? Write ✓ or ✗ on paper next to 13.
05. You wrote ✓ because 13 ≤ 17 is TRUE; 13 is a candidate for Floor(17); But is 13 the MAXIMUM of all nodes ≤ 17? You do NOT know yet; What you DO know: All nodes in LEFT subtree of 13 are < 13 < 17; So LEFT subtree nodes are also ≤ 17, but they are smaller than 13; They cannot beat 13; What about RIGHT subtree of 13?
06. RIGHT subtree of 13 contains vals {14, 16, 18}; Some of these might be ≤ 17 AND > 13; Write down which ones: {14, 16}; These could beat 13 as Floor candidate; So we MUST check RIGHT subtree; We do NOT discard 13, we STORE it.
07. Open `25_floor_ceil_bst.rs` line 27; Write this inside `floor` function: `let mut curr = root;` → This is your finger pointing at 0x100; Write: `let mut res: Option<i32> = None;` → This stores candidate (None for now because we haven't compared yet).
08. Now write the loop header: `while let Some(node) = curr {` → This checks if finger is pointing at a valid node; At step 04 your finger was at 0x100 which is Some; So loop body executes.
09. Inside loop, write: `let val = node.borrow().val;` → At 0x100, val = 13; Write this on paper: curr=0x100, val=13.
10. Now the branch: We need to compare val with x; Write: `if val == x { return Some(x); }` → Is 13 == 17? No; This branch does NOT execute; Write ✗ next to this line on paper.
11. Next branch: `if val > x {` → Is 13 > 17? Is 13 > 17? No, 13 < 17; This branch does NOT execute; Write ✗.
12. Else branch executes because val < x (13 < 17); What do we do? From step 05: We store 13 as candidate AND move RIGHT; Write: `} else { res = Some(val); curr = node.borrow().right.clone(); }`
13. After step 12: res = Some(13), curr = 0x300; Write on paper: res=13, curr=0x300; Loop continues because curr is Some.
14. Iteration 2: curr=0x300, val=16; Is 16 == 17? No ✗; Is 16 > 17? No ✗; Else: res = Some(16), curr = 0x700; Write on paper: res=16, curr=0x700.
15. Iteration 3: curr=0x700, val=18; Is 18 == 17? No ✗; Is 18 > 17? YES ✓; What now? 18 cannot be Floor because 18 > 17; What about RIGHT subtree of 18? All nodes there are > 18 > 17; So RIGHT subtree cannot have Floor; What about LEFT subtree of 18? It contains nodes between 16 and 18; Could have Floor candidate; Write: `curr = node.borrow().left.clone();`
16. Wait: We need to complete the if-else; Go back to step 11; Fill in: `if val > x { curr = node.borrow().left.clone(); } else { ... }`
17. After step 15: curr = None (0x700 has no left child); Loop condition fails; Return res; res = 16; Floor(17) = 16 ✓.
18. Verify: You computed Floor(17) by visiting 0x100→0x300→0x700→None; That is 3 nodes, not 7; Height of tree is 3; O(H) ✓.
19. Write complete floor function now by combining steps 07-16; Do not copy-paste; Type each line and verify against paper trace.
20. TRAP WARNING: What if x < all nodes? Example x=4; Trace: 0x100(13>4)→Left→0x200(9>4)→Left→0x400(5>4)→Left→None; res never updated; res = None; Floor(4) = None ✓.
21. TRAP WARNING: What if x == some node? Example x=10; Trace: 0x100(13>10)→Left→0x200(9<10,res=9)→Right→0x500(10==10)→return Some(10); Floor(10) = 10 ✓.
22. Now CEIL: Definition is OPPOSITE; Ceil(x) = min of all nodes ≥ x; For Ceil, if val < x, go RIGHT (too small); if val ≥ x, store candidate, go LEFT (might find smaller valid).
23. Trace Ceil(17): 0x100(13<17)→Right→0x300(16<17)→Right→0x700(18≥17,res=18)→Left→None; return 18 ✓.
24. Write `ceil` function by swapping the branching logic from `floor`; val < x means go right; val >= x means store and go left.
