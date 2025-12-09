05 Cyclic Permutation Detection

PROBLEM
Check if s2 is a rotation of s1.
Example: "CDAB" is rotation of "ABCD" (rotated left by 2).
Example: "ACBD" is NOT rotation of "ABCD" (different order).

FIGURE 1: Rotation Definition

s1 = "ABCD"  (Length 4)

Rotation 0: ABCD (original)
Rotation 1: BCDA (move A to end)
Rotation 2: CDAB (move AB to end)
Rotation 3: DABC (move ABC to end)
Rotation 4: ABCD (full circle)

4 possible starting points = 4 rotations.

FIGURE 2: The Paper Ring Model

          ╭─── A ───╮
         D           B
          ╰─── C ───╯

Read clockwise from A: ABCD
Read clockwise from B: BCDA
Read clockwise from C: CDAB
Read clockwise from D: DABC

DCAB is NOT possible: After D comes A on ring, not C.

FIGURE 3: Concatenation Trick Derivation

Problem: Match "CDAB" against "ABCD"
  Match C at index 2. OK.
  Match D at index 3. OK.
  Match A at index 4. Index 4 does not exist.
  Must wrap to index 0. Annoying.

Solution: Concatenate s1+s1 to remove the edge.

Indices:   0   1   2   3   4   5   6   7
         [ A , B , C , D , A , B , C , D ]

Match C at index 2. D at 3. A at 4. B at 5.
Continuous match. No wraparound math.

FIGURE 4: All Rotations as Substrings

s1+s1 = "ABCDABCD"

Position 0-3: ABCD (rotation 0)
Position 1-4: BCDA (rotation 1)
Position 2-5: CDAB (rotation 2) <-- s2 found
Position 3-6: DABC (rotation 3)

If s2 is a rotation, it appears as substring of s1+s1.

FIGURE 5: Length Check Requirement

s1 = "ABCD" (len 4)
s2 = "CD"   (len 2)

s1+s1 = "ABCDABCD"
"CD" found at position 2. But 4 != 2.
"CD" is NOT a rotation of "ABCD".

Check len(s1) == len(s2) BEFORE searching.

FIGURE 6: False Positive Prevention

s1 = "ABCD"  s2 = "ACBD"

s1+s1 = "ABCDABCD"
Position 0: ABCD vs ACBD (mismatch at B vs C)
Position 1: BCDA vs ACBD (mismatch at B vs A)
Position 2: CDAB vs ACBD (mismatch at C vs A)
Position 3: DABC vs ACBD (mismatch at D vs A)

"ACBD" not found. Return false.
Same characters but different order. Not a rotation.

IMPLEMENTATION

fn is_cyclic_perm(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let concat = format!("{}{}", s1, s1);
    concat.contains(s2)
}
