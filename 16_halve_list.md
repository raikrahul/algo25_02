# 16

```
[2,4,5,3,8,7,6,1,9] → [7,6,1,9,2,4,5,3,8]
[1,3,5,7,2,4,6,8] → [2,4,6,8,1,3,5,7]
```

---

```
01. 2,4,5,3,8,7,6,1,9
02. 9
03. 9÷2=4 R1
04. ⌈4.5⌉=5
05. 9-5=4
06. 5+4=9 ✓
07. {0,1,2,3,4}=5
08. {5,6,7,8}=4
09. 5>4 ✓
```

```
┌─┐ ┌─┐ ┌─┐ ┌─┐ ┌─┐ ┌─┐ ┌─┐ ┌─┐ ┌─┐
│2│→│4│→│5│→│3│→│8│→│7│→│6│→│1│→│9│→∅
└─┘ └─┘ └─┘ └─┘ └─┘ └─┘ └─┘ └─┘ └─┘
 0   1   2   3   4   5   6   7   8
```

```
10. pass₁: 0→8 = 9
11. pass₂: 0→4 = 5
12. 9+5=14
13. 14÷9=1.55
14. 1.55>1 ∴ 2 passes
15. constraint: 1 pass
16. 2>1 ✗
```

```
17. position₈: ✓ (at end)
18. position₄: ✗ (need, don't have)
19. 8-4=4
20. backwards: ✗ (singly linked)
21. save position₄ at time₄
```

```
22. time | A | B
     0  | 0 | 0
     1  | 1 | ?
     2  | 2 | ?
     3  | 3 | ?
     4  | 4 | ?

23. A=8 at t=4 → speed_A=8/4=2
24. B=4 at t=4 → speed_B=4/4=1
25. 2/1=2
26. speed_A=2×speed_B
```

```
27. t | slow | fast
    0 |  0   |  0
    1 |  1   |  2
    2 |  2   |  4
    3 |  3   |  6
    4 |  4   |  8

28. fast=8 → slow=4 ✓
```

```
29. 0x100:2 0x108:4 0x110:5 0x118:3 0x120:8 0x128:7 0x130:6 0x138:1 0x140:9

30. slow=0x100 fast=0x100
31. slow→0x108 fast→0x110
32. slow→0x110 fast→0x120
33. slow→0x118 fast→0x130
34. slow→0x120 fast→0x140
35. 0x140.next=∅ → stop
36. slow=0x120=idx4 ✓
```

```
37. 1,3,5,7,2,4,6,8
38. 8
39. 8÷2=4
40. {0,1,2,3}=4
41. {4,5,6,7}=4
42. split_idx=3
```

```
43. 0x200:1 0x208:3 0x210:5 0x218:7 0x220:2 0x228:4 0x230:6 0x238:8

44. slow=0x200 fast=0x200
45. slow→0x208 fast→0x210
46. slow→0x210 fast→0x220
47. slow→0x218 fast→0x230
48. 0x230.next=0x238 0x238.next=∅
49. slow→0x220 fast→∅
50. slow=idx4 ≠ idx3 ✗
```

```
51. iter3: slow=3 fast=6
52. iter4: slow=4 fast=8(∅)
53. stop at iter3
54. iter3: fast.next.next=0x238 ≠∅
55. iter4: fast.next.next=∅
56. condition: fast.next≠∅ ∧ fast.next.next≠∅
```

```
57. 0x100→0x108→0x110→0x118→0x120→0x128→0x130→0x138→0x140→∅
                              ↑slow
58. second_head=0x120.next=0x128
59. 0x120.next=∅
60. 0x100→0x108→0x110→0x118→0x120→∅
61. 0x128→0x130→0x138→0x140→∅

62. curr=0x128
63. 0x128.next≠∅→0x130
64. 0x130.next≠∅→0x138
65. 0x138.next≠∅→0x140
66. 0x140.next=∅→tail=0x140

67. 0x140.next=0x100
68. 0x128→0x130→0x138→0x140→0x100→0x108→0x110→0x118→0x120→∅
69. return 0x128
```

```
70. N=1: [5]
71. fast.next=∅→skip loop
72. second_head=∅
73. return head

74. N=2: [10,20]
75. fast.next≠∅ fast.next.next=∅→stop
76. slow=head second_head=head.next
77. 0x408→0x400→∅
78. [20,10]

79. N=0: []
80. head=∅
81. return ∅
```

```
F1. 9/2=4→first=4 second=5→4<5 ✗
F2. count+traverse=2 passes ✗
F3. no lookahead→even overshoot ✗
F4. overwrite head→lost ✗
F5. no break→cycle ✗
F6. fast=tail assumed→even fails ✗
F7. N=1 second_head.next→crash ✗
```

```
82. pointers: 5
83. 5×8=40 bytes
84. O(1)
```
