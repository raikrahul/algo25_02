01. input→output: [4,2,3,1]→[1,2,3,4], constraint=no_copy_values, only_rewire_pointers
02. draw: 0x100(4)→0x108(2)→0x110(3)→0x118(1)→∅
03. constraint: external pointers→0x108 must still point to val=2 after sort
04. ∴ cannot do: 0x100.val=1, 0x108.val=2 → ✗ breaks external references
05. ∴ must do: rearrange .next pointers, node addresses stay same, values stay in nodes
06. what: input=head, output=sorted_head, nodes rearranged by .next
07. why: applications hold 0x108, if val moves from 0x108→0x100, app sees wrong val
08. where: linked list, no array indices, no random access
09. when: O(n log n) possible via merge sort, O(n²) via bubble/insertion
10. which: merge sort→split+merge, insertion sort→traverse+insert
11. N=4: [4,2,3,1], draw 0x100(4)→0x108(2)→0x110(3)→0x118(1)→∅
12. split₁: find mid, slow/fast, s=0x100 f=0x100
13. iter₁: s→0x108, f→0x110, iter₂: s→0x110, f→∅, stop
14. mid=0x110, split: left=0x100→0x108→∅, right=0x110→0x118→∅
15. ∴ break at 0x108.next=∅, left ends at 0x108
16. recurse left [4,2]: split→[4],[2], merge→[2,4]
17. recurse right [3,1]: split→[3],[1], merge→[1,3]
18. merge [2,4]+[1,3]: compare 2 vs 1, 1<2, pick 1, advance right
19. compare 2 vs 3, 2<3, pick 2, advance left
20. compare 4 vs 3, 3<4, pick 3, advance right
21. right empty, pick 4
22. result: 1→2→3→4, nodes: 0x118→0x108→0x110→0x100→∅
23. verify: external ptr→0x108 still sees val=2 ✓
24. F1: copy vals to array, sort, copy back → ✗ constraint violation
25. F2: swap node.val directly → ✗ constraint violation, external refs break
26. F3: odd N split wrong → N=5: slow stops at 3rd, must cut before mid
27. F4: forget to break left tail → left.last.next still points to right, merge loops
28. F5: merge without dummy head → need prev pointer to track, easy to miss first node
29. F6: single node base case → return immediately, no split
30. F7: two nodes → one comparison, wire directly
31. N=7: [7,4,9,2,5,1,8]
32. split: find mid at position 3 (0-indexed)
33. s=0,f=0→s=1,f=2→s=2,f=4→s=3,f=6→s=4,f=∅, mid=4, left=[7,4,9,2], right=[5,1,8]
34. wait: f→6 is last node, f.next=∅, stop
35. left=[7,4,9,2], right=[5,1,8]
36. recurse left: split→[7,4],[9,2]→merge→[4,7],[2,9]→merge→[2,4,7,9]
37. recurse right: split→[5],[1,8]→[1,8]→merge→[1,5,8]
38. merge: [2,4,7,9]+[1,5,8]
39. 2 vs 1→1, 2 vs 5→2, 4 vs 5→4, 7 vs 5→5, 7 vs 8→7, 9 vs 8→8, 9→∅
40. result: [1,2,4,5,7,8,9]
41. N=1: [5]→[5], no split, return head
42. N=2: [3,1]→split [3],[1]→merge 3 vs 1→[1,3]
43. N=0: ∅→∅, return None
44. F8: merge two sorted lists, both same length — ensure pointer wiring correct
45. F9: merge unequal lengths — leftover tail must be appended
46. F10: mutation during recursion — must pass ownership or clone Rc
47. merge algorithm: dummy→∅, curr=dummy
48. while L≠∅ ∧ R≠∅: if L.val≤R.val: curr.next=L, L=L.next, else curr.next=R, R=R.next, curr=curr.next
49. if L≠∅: curr.next=L, else: curr.next=R
50. return dummy.next
51. time: T(n)=2T(n/2)+O(n), master theorem→O(n log n)
52. space: O(log n) recursion stack, or O(1) bottom-up
53. N=8: [8,4,6,2,7,3,5,1]
54. split→[8,4,6,2],[7,3,5,1]
55. split→[8,4],[6,2],[7,3],[5,1]
56. split→[8],[4],[6],[2],[7],[3],[5],[1]
57. merge→[4,8],[2,6],[3,7],[1,5]
58. merge→[2,4,6,8],[1,3,5,7]
59. merge→[1,2,3,4,5,6,7,8]
60. verify: 8 nodes, log₂(8)=3 levels, 8+8+8=24 comparisons worst
61. N=5 fractional: [5,1,4,2,3]
62. split: s→1,2,3 (3rd), mid=3, left=[5,1,4], right=[2,3]
63. trap: 5/2=2.5, floor→2, ceil→3, which index?
64. slow/fast gives left_len=⌈n/2⌉ or ⌊n/2⌋ depending on stop condition
65. insertion sort alternative: O(n²), maintains sorted prefix
66. sorted=∅, curr=head
67. while curr≠∅: next=curr.next, insert curr into sorted, curr=next
68. insert: traverse sorted, find position where prev.val≤curr.val<curr_sorted.val
69. N=4 insertion: [4,2,3,1]
70. step₁: sorted=[4], curr=2
71. step₂: 2<4, insert before 4, sorted=[2,4], curr=3
72. step₃: 2<3<4, insert between, sorted=[2,3,4], curr=1
73. step₄: 1<2, insert before 2, sorted=[1,2,3,4], curr=∅
74. comparisons: 1+2+3=6 for N=4, O(n²) worst
75. address trace insertion: 0x100(4)→sorted_head=0x100
76. insert 0x108(2): 2<4, prev=∅→sorted_head=0x108, 0x108.next=0x100
77. insert 0x110(3): 2<3, 3<4, prev=0x108→0x108.next=0x110, 0x110.next=0x100
78. insert 0x118(1): 1<2, prev=∅→sorted_head=0x118, 0x118.next=0x108
79. final: 0x118(1)→0x108(2)→0x110(3)→0x100(4)→∅
80. external ptr 0x108→val=2 ✓, 0x100→val=4 ✓
81. F11: insertion at head — must update sorted_head
82. F12: insertion at tail — must traverse entire sorted list
83. F13: equal values — stable sort? depends on ≤ vs <
84. bottom-up merge sort: O(1) space, iterative, merge sublists of size 1,2,4,8,...
85. pass₁: merge pairs (1,2),(3,4),(5,6),(7,8) → sublists of 2
86. pass₂: merge pairs (1-2,3-4),(5-6,7-8) → sublists of 4
87. pass₃: merge pairs (1-4,5-8) → sorted
88. implementation: split_at(head, size) → left, rest
89. N=6 bottom-up: [6,3,5,1,4,2]
90. pass₁ size=1: [3,6],[1,5],[2,4]
91. pass₂ size=2: [1,3,5,6],[2,4]
92. pass₃ size=4: [1,2,3,4,5,6]
93. trap: rightmost sublist may have fewer elements
94. F14: bottom-up merge forgets to relink tail of merged list to next sublist
95. F15: miscounting size when n not power of 2
96. |N|split_count|merge_count|comparisons_max| → |4|3|3|5|, |8|7|7|17|, |16|15|15|49|
97. pattern: split_count=n-1, merge_count=n-1, comparisons≤n×log₂(n)
98. while fast.is_some() && fast.as_ref().unwrap().borrow().next.is_some() → type trace:
99. fast : Option<Rc<RefCell<Node>>> → Some(Rc→0x100) or None
100. fast.is_some() : bool → true if Some, false if None, guard for unwrap
101. fast.as_ref() : Option<&Rc<RefCell<Node>>> → borrow without moving fast
102. .unwrap() : &Rc<RefCell<Node>> → extract &Rc, safe because is_some()=true
103. .borrow() : Ref<Node> → RefCell runtime borrow check, shared reference
104. .next : Link=Option<Rc<...>> → field access, NOT method, no parentheses
105. .is_some() : bool → true if next exists, false if next=None
106. && short-circuit: if is_some()=false, right side not evaluated, no panic
107. trap: .next() ✗ vs .next ✓ → next is field, not method
108. if let Some(node) = &left_tail { node.borrow_mut().next = None; } → type trace:
109. left_tail : Option<Rc<RefCell<Node>>> → Some(Rc→0x108)
110. &left_tail : &Option<...> → borrow, don't consume left_tail
111. if let Some(node) = &... → pattern match, node : &Rc<RefCell<Node>>
112. node.borrow_mut() : RefMut<Node> → write access to inner Node
113. .next = None : assign None, old Some(Rc→0x110) dropped
114. drop of old Rc: 0x110.strong 2→1, node NOT deallocated (middle holds ref)
115. RefMut dropped at block end: borrow_flag -1→0
116. before: 0x108.next=Some(Rc→0x110), after: 0x108.next=None
117. why &left_tail: if let Some(x)=left_tail moves, if let Some(x)=&left_tail borrows
118. why borrow_mut not borrow: borrow()=read-only, borrow_mut()=write access
119. 
120. # Mistakes Made
121. 
122. M1. .next() vs .next
123. wrote: fast.as_ref().unwrap().borrow().next().is_some()
124. fix: fast.as_ref().unwrap().borrow().next.is_some()
125. next is field, not method. () calls method, no () accesses field.
126. Q: did you read struct Node definition before typing?
127. 
128. M2. (prev,.slow) typo
129. wrote: (prev,.slow)
130. fix: (prev, slow)
131. extra period. syntax error.
132. Q: did you proofread before moving on?
133. 
134. M3. head vs _head
135. param = _head, body used head
136. 1 character mismatch.
137. Q: did you read function signature before typing body?
138. 
139. M4. clone confusion
140. asked: "slow is already an option and we clone the option again"
141. answer: clone() needed to avoid move. prev=slow moves slow. prev=slow.clone() keeps slow valid.
142. Q: do you understand Rust ownership? move vs clone?
143. 
144. M5. as_ref confusion
145. asked: "_head is an option, right? how can I do clone on an option?"
146. asked: "_head.clone does what?"
147. answer: Option<T> implements Clone if T implements Clone. as_ref() borrows without moving.
148. Q: did you trace types before asking? Option → as_ref() → Option<&T> → unwrap() → &T
149. 
150. M6. unwrap() everywhere
151. wrote: .unwrap() 7 times in 4 functions
152. fix: match or if let patterns
153. unwrap() panics on None. production code must handle None.
154. Q: why default to unwrap() instead of match?
155. 
156. M7. borrow checker violations
157. wrote: curr = curr.borrow().next.clone().unwrap()
158. error: cannot assign to curr because it is borrowed
159. fix: let next = curr.borrow().next.clone(); curr = next.unwrap();
160. Q: did you understand RefCell borrow lifetime extends to end of statement?
161. 
162. M8. while let Some(ref x) = curr
163. wrote: while let Some(ref curr_node) = curr { curr = ... }
164. error: curr borrowed by ref, cannot reassign
165. fix: loop { let curr_node = match &curr { Some(n) => n.clone(), None => break }; }
166. Q: did you trace borrow scope before using ref pattern?
167. 
168. # Orthogonal Analysis
169. 
170. Pattern: 8 errors, 0 type traces done before writing
171. 
172. You think: "I know what to write"
173. Orthogonal: "Trace types first, then write"
174. 
175. You think: "This should work"
176. Orthogonal: "Compiler will reject, trace borrow lifetimes"
177. 
178. You think: "Option has clone"
179. Orthogonal: "Option<Rc> clones Rc, refcount+1, not deep copy"
180. 
181. You think: "I'll unwrap, it's Some"
182. Orthogonal: "Production code panics, match handles None"
183. 
184. You think: ".next() gets next"
185. Orthogonal: "next is field, () calls nonexistent method"
186. 
187. You think: "ref pattern is idiomatic"
188. Orthogonal: "ref borrows for entire match scope, blocks reassignment"
189. 
190. You think: "One statement is fine"
191. Orthogonal: "RefCell borrow lives until semicolon, split into two statements"
192. 
193. Root cause: typing before tracing. Eyes saw code, hands typed, brain skipped type analysis.
194. 
195. Fix: trace type chain on paper before each line. Read function signature first. Check borrow scope before ref pattern.

