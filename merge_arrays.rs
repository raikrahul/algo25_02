// Problem: Merge Arrays
// Description:
// Given two sorted arrays, A1 (size M+N, with N empty slots) and A2 (size N),
// merge A2 into A1 such that A1 becomes entirely sorted.
// No extra space allowed.

pub fn merge(a1: &mut Vec<i32>, m: usize, a2: &Vec<i32>, n: usize) {
    // --------------------------------------------------------------------------------
    // Task 1: Break Down the Variables
    // --------------------------------------------------------------------------------
    
    // let k = ...; 
    // This variable 'k' represents the final destination index in the universe of the a1 array.
    // It will look like 5, 10, 1000 - the very edge of the known map.
    // Calculation: If m = 3 and n = 3, the total size is 6. The last valid index is 6 - 1 = 5.
    // The background is it comes from the problem statement wanting us to fill from the back to avoid collisions.

    // let i = ...;
    // This variable 'i' is the champion of the first tribe (a1).
    // It points to the largest active warrior in a1.
    // It will look like m - 1. E.g., if m=3, i starts at 2.
    // The background is we need to compare the largest available elements first.

    // let j = ...;
    // This variable 'j' is the champion of the second tribe (a2).
    // It points to the largest active warrior in a2.
    // It will look like n - 1. E.g., if n=3, j starts at 2.
    // The background is we need to compare the largest available elements first from the invader array.

    // --------------------------------------------------------------------------------
    // The Loop of War
    // --------------------------------------------------------------------------------
    // While both tribes still have warriors (i >= 0 and j >= 0)...
    // We must calculate: Who is stronger? a1[i] or a2[j]?
    
    // If a1[i] > a2[j]:
    // Calculation: The local power of a1 is greater.
    // Action: Place a1[i] at position k.
    // Decrement i (he is spent).
    
    // Else:
    // Calculation: The invader a2[j] is stronger or equal.
    // Action: Place a2[j] at position k.
    // Decrement j (he is spent).

    // Decrement k (the territory is claimed).

    // --------------------------------------------------------------------------------
    // The Aftermath (Logic Hole)
    // --------------------------------------------------------------------------------
    // What if tribe a1 runs out of warriors first? (i < 0)
    // But tribe a2 still has men? (j >= 0)
    // We must copy the remaining a2 into a1.
    
    // What if tribe a2 runs out first?
    // Then a1 is already in place. We do nothing.
    
    // YOUR CODE GOES HERE.
    // DO NOT USE SORT(). USE LOGIC.
    

    // ptr_a1: pointer to current element in a1's sorted region (moves left)
    // ptr_a2: pointer to current element in a2 (moves left)
    // write_pos: where we write the next largest element (moves left)
    let mut ptr_a1: Option<usize> = m.checked_sub(1);
    let mut ptr_a2: Option<usize> = n.checked_sub(1);
    let mut write_pos: usize = m + n - 1;

    // ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════
    // MAIN MERGE LOOP
    // ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════
    // PURPOSE: Compare largest remaining elements from BOTH arrays, place winner at write_pos, move pointers left.
    // LOOP CONDITION: (Some(i), Some(j)) means BOTH ptr_a1 AND ptr_a2 must be Some. If either is None, loop exits.
    //
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // MIDDLE-SCALE EXAMPLE: m=3, n=3, a1=[2,5,9,0,0,0], a2=[1,6,8]
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // Initial: ptr_a1=Some(2), ptr_a2=Some(2), write_pos=5
    // 
    // ITER 1: i=2, j=2. a1[2]=9, a2[2]=8. Is 9>8? 9-8=1>0 → YES.
    //         a1[5]=9. ptr_a1=2.checked_sub(1)=Some(1). write_pos=5-1=4.
    //         State: a1=[2,5,9,0,0,9], ptr_a1=Some(1), ptr_a2=Some(2), write_pos=4
    //
    // ITER 2: i=1, j=2. a1[1]=5, a2[2]=8. Is 5>8? 5-8=-3<0 → NO.
    //         a1[4]=8. ptr_a2=2.checked_sub(1)=Some(1). write_pos=4-1=3.
    //         State: a1=[2,5,9,0,8,9], ptr_a1=Some(1), ptr_a2=Some(1), write_pos=3
    //
    // ITER 3: i=1, j=1. a1[1]=5, a2[1]=6. Is 5>6? 5-6=-1<0 → NO.
    //         a1[3]=6. ptr_a2=1.checked_sub(1)=Some(0). write_pos=3-1=2.
    //         State: a1=[2,5,9,6,8,9], ptr_a1=Some(1), ptr_a2=Some(0), write_pos=2
    //
    // ITER 4: i=1, j=0. a1[1]=5, a2[0]=1. Is 5>1? 5-1=4>0 → YES.
    //         a1[2]=5. ptr_a1=1.checked_sub(1)=Some(0). write_pos=2-1=1.
    //         State: a1=[2,5,5,6,8,9], ptr_a1=Some(0), ptr_a2=Some(0), write_pos=1
    //
    // ITER 5: i=0, j=0. a1[0]=2, a2[0]=1. Is 2>1? 2-1=1>0 → YES.
    //         a1[1]=2. ptr_a1=0.checked_sub(1)=None. write_pos=1-1=0.
    //         State: a1=[2,2,5,6,8,9], ptr_a1=None, ptr_a2=Some(0), write_pos=0
    //
    // LOOP EXIT: (None, Some(0)) does not match (Some(i), Some(j)). Exit.
    // Leftover: ptr_a2=Some(0) → a2 still has element at index 0 (value=1).
    //
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // LARGE-SCALE EXAMPLE: m=5, n=4, a1=[100,200,300,400,500,0,0,0,0], a2=[150,250,350,450]
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // Initial: ptr_a1=Some(4)→500, ptr_a2=Some(3)→450, write_pos=8
    //
    // ITER 1: 500>450? 500-450=50>0→YES. a1[8]=500. ptr_a1=Some(3). write_pos=7.
    // ITER 2: a1[3]=400, a2[3]=450. 400>450? -50<0→NO. a1[7]=450. ptr_a2=Some(2). write_pos=6.
    // ITER 3: a1[3]=400, a2[2]=350. 400>350? 50>0→YES. a1[6]=400. ptr_a1=Some(2). write_pos=5.
    // ITER 4: a1[2]=300, a2[2]=350. 300>350? -50<0→NO. a1[5]=350. ptr_a2=Some(1). write_pos=4.
    // ITER 5: a1[2]=300, a2[1]=250. 300>250? 50>0→YES. a1[4]=300. ptr_a1=Some(1). write_pos=3.
    // ITER 6: a1[1]=200, a2[1]=250. 200>250? -50<0→NO. a1[3]=250. ptr_a2=Some(0). write_pos=2.
    // ITER 7: a1[1]=200, a2[0]=150. 200>150? 50>0→YES. a1[2]=200. ptr_a1=Some(0). write_pos=1.
    // ITER 8: a1[0]=100, a2[0]=150. 100>150? -50<0→NO. a1[1]=150. ptr_a2=None. write_pos=0.
    //
    // LOOP EXIT: (Some(0), None) → ptr_a2 is None. Exit.
    // Leftover from a1: ptr_a1=Some(0)→100. But 100 is ALREADY at a1[0]. No copy needed.
    // FINAL: [100,150,200,250,300,350,400,450,500] ✓
    //
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // EDGE CASE: m=0, n=3, a1=[0,0,0], a2=[1,2,3]
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // Initial: ptr_a1=0.checked_sub(1)=None, ptr_a2=Some(2), write_pos=2
    // LOOP CHECK: (None, Some(2)) → Does not match. Loop NEVER RUNS.
    // Leftover: ptr_a2=Some(2) → Must copy all of a2.
    //
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // SMALL-SCALE: m=1, n=1, a1=[5,0], a2=[3]
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // Initial: ptr_a1=Some(0)→5, ptr_a2=Some(0)→3, write_pos=1
    // ITER 1: 5>3? 5-3=2>0→YES. a1[1]=5. ptr_a1=None. write_pos=0.
    // LOOP EXIT. Leftover: ptr_a2=Some(0)→3. Copy: a1[0]=3.
    // FINAL: [3,5] ✓
    // ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════
    while let (Some(i), Some(j)) = (ptr_a1, ptr_a2) {
        // i: unwrapped index into a1. Example: i=2 means a1[2]=9 (from m=3 case).
        // j: unwrapped index into a2. Example: j=2 means a2[2]=8 (from n=3 case).
        // The pattern matching GUARANTEES both i and j are valid. No None panic possible here.
        
        if a1[i] > a2[j] {
            // CALCULATION: a1[i] wins the comparison.
            // Example: a1[2]=9, a2[2]=8. 9>8 → 9 goes to a1[write_pos].
            // Memory: a1[5] ← 9. Address calculation: base_addr + 5*4 bytes = base_addr + 20.
            // The original 9 at a1[2] is NOT erased yet. It will be overwritten in a future iteration
            // OR it stays if write_pos never reaches index 2.
            a1[write_pos] = a1[i];
            
            // DECREMENT ptr_a1: Move to next smaller element in a1.
            // Example: i=2. 2.checked_sub(1)=Some(1). Now ptr_a1 points to a1[1]=5.
            // Edge: i=0. 0.checked_sub(1)=None. ptr_a1 becomes None. a1 is exhausted.
            // Calculation: 2-1=1 (wrapping handled by checked_sub).
            ptr_a1 = i.checked_sub(1);
        } else {
            // CALCULATION: a2[j] wins OR ties the comparison (ties go to a2).
            // Example: a1[1]=5, a2[2]=8. 5≤8 → 8 goes to a1[write_pos].
            // Tie example: a1[i]=7, a2[j]=7. 7>7 is FALSE. a2[j]=7 wins.
            // This is arbitrary but consistent. Could also give tie to a1[i].
            a1[write_pos] = a2[j];
            
            // DECREMENT ptr_a2: Move to next smaller element in a2.
            // Example: j=2. 2.checked_sub(1)=Some(1). Now ptr_a2 points to a2[1]=6.
            // Edge: j=0. 0.checked_sub(1)=None. ptr_a2 becomes None. a2 is exhausted.
            ptr_a2 = j.checked_sub(1);
        }
        
        // DECREMENT write_pos: Move to next unfilled position (leftward).
        // Example: write_pos=5. 5-1=4. Next element will be written at a1[4].
        // Note: write_pos is usize. If it reaches 0 and we decrement, we get underflow.
        // But this only happens AFTER the last element is placed, and then the loop exits
        // because one of ptr_a1 or ptr_a2 becomes None.
        // Calculation: 5→4→3→2→1→0. At 0, if loop continues, 0-1=panic. But loop exits first.
        write_pos -= 1;
    }

    // ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════
    // LEFTOVER LOOP: a2 still has elements (a1 exhausted first)
    // ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════
    // WHEN DOES THIS RUN? When ptr_a1=None but ptr_a2=Some(j).
    // This means all a1 elements are placed, but a2 still has smaller elements remaining.
    //
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // CONTINUATION FROM MIDDLE-SCALE EXAMPLE:
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // State after main loop: a1=[2,2,5,6,8,9], ptr_a1=None, ptr_a2=Some(0), write_pos=0
    // a2[0]=1 is still not placed!
    //
    // LEFTOVER ITER 1: j=0. a1[0]=a2[0]=1. ptr_a2=0.checked_sub(1)=None.
    //                  is_some() = false → do NOT decrement write_pos.
    // LOOP EXIT: ptr_a2=None.
    // FINAL: a1=[1,2,5,6,8,9] ✓
    //
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // EDGE CASE CONTINUATION: m=0, n=3, a1=[0,0,0], a2=[1,2,3]
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // State: ptr_a1=None (never had elements), ptr_a2=Some(2), write_pos=2
    //
    // LEFTOVER ITER 1: j=2. a1[2]=a2[2]=3. ptr_a2=Some(1). is_some()=true → write_pos=1.
    // LEFTOVER ITER 2: j=1. a1[1]=a2[1]=2. ptr_a2=Some(0). is_some()=true → write_pos=0.
    // LEFTOVER ITER 3: j=0. a1[0]=a2[0]=1. ptr_a2=None. is_some()=false → write_pos stays 0.
    // FINAL: a1=[1,2,3] ✓
    //
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // WHY is_some() CHECK?
    // ───────────────────────────────────────────────────────────────────────────────────────────────────────────────
    // After placing the LAST element of a2 (at index 0), ptr_a2 becomes None.
    // If we blindly do write_pos -= 1, and write_pos=0, we get 0-1=underflow=panic.
    // The is_some() check prevents this. Only decrement if there are MORE elements coming.
    //
    // Calculation trace:
    // j=2: place, ptr_a2=Some(1), is_some()=true, write_pos=2→1
    // j=1: place, ptr_a2=Some(0), is_some()=true, write_pos=1→0
    // j=0: place, ptr_a2=None, is_some()=false, write_pos=0 (no decrement, no panic)
    // ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════
    while let Some(j) = ptr_a2 {
        // j: unwrapped index into a2. Example: j=0 means a2[0]=1.
        // ptr_a1 is None here (guaranteed by main loop exit condition).
        a1[write_pos] = a2[j];
        
        // Decrement ptr_a2.
        ptr_a2 = j.checked_sub(1);
        
        // Only decrement write_pos if more elements are coming.
        // Example: j=1, ptr_a2=Some(0), is_some()=true → decrement.
        // Example: j=0, ptr_a2=None, is_some()=false → do NOT decrement.
        if ptr_a2.is_some() {
            write_pos -= 1;
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════
    // LEFTOVER FROM a1: ALREADY IN PLACE
    // ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════
    // If ptr_a2=None but ptr_a1=Some(i), then a1's remaining elements [0..i] are ALREADY in a1.
    // They don't need to be copied anywhere. They're already in the correct position.
    //
    // Example: a1=[1,2,3,0,0,0], a2=[7,8,9]. After main loop: a1=[1,2,3,7,8,9], ptr_a1=Some(2), ptr_a2=None.
    // The [1,2,3] at indices 0,1,2 are ALREADY THERE. No action needed.
    // ═══════════════════════════════════════════════════════════════════════════════════════════════════════════════    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_standard() {
        // Example from problem
        // a1 (m=3, n=3): [2, 5, 9, 0, 0, 0] (0s are placeholders)
        // a2 (n=3): [1, 6, 8]
        // Result: [1, 2, 5, 6, 8, 9]
        let mut a1 = vec![2, 5, 9, 0, 0, 0];
        let m = 3;
        let a2 = vec![1, 6, 8];
        let n = 3;
        merge(&mut a1, m, &a2, n);
        assert_eq!(a1, vec![1, 2, 5, 6, 8, 9]);
    }

    #[test]
    fn test_merge_a2_empty() {
        // a1 (m=3, n=0): [1, 2, 3]
        // a2 (n=0): []
        let mut a1 = vec![1, 2, 3];
        let m = 3;
        let a2 = vec![];
        let n = 0;
        merge(&mut a1, m, &a2, n);
        assert_eq!(a1, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_a1_empty_part() {
        // a1 (m=0, n=3): [0, 0, 0]
        // a2 (n=3): [1, 2, 3]
        let mut a1 = vec![0, 0, 0];
        let m = 0;
        let a2 = vec![1, 2, 3];
        let n = 3;
        merge(&mut a1, m, &a2, n);
        assert_eq!(a1, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_interleaved() {
        // a1: [10, 30, 50, 0, 0, 0]
        // a2: [20, 40, 60]
        let mut a1 = vec![10, 30, 50, 0, 0, 0];
        let m = 3;
        let a2 = vec![20, 40, 60];
        let n = 3;
        merge(&mut a1, m, &a2, n);
        assert_eq!(a1, vec![10, 20, 30, 40, 50, 60]);
    }
    
    #[test]
    fn test_merge_a2_smaller_all() {
        // a1: [4, 5, 6, 0, 0, 0]
        // a2: [1, 2, 3]
        let mut a1 = vec![4, 5, 6, 0, 0, 0];
        let m = 3;
        let a2 = vec![1, 2, 3];
        let n = 3;
        merge(&mut a1, m, &a2, n);
        assert_eq!(a1, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_your_swap_idea_works() {
        // YOUR SPECIAL CASE: min(a1) = 10 > max(a2) = 3
        // m = 9, n = 3
        // a1: [10, 20, 30, 40, 50, 60, 70, 80, 90, 0, 0, 0]
        // a2: [1, 2, 3]
        // Expected: [1, 2, 3, 10, 20, 30, 40, 50, 60, 70, 80, 90]
        let mut a1 = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 0, 0, 0];
        let m = 9;
        let a2 = vec![1, 2, 3];
        let n = 3;
        merge(&mut a1, m, &a2, n);
        assert_eq!(a1, vec![1, 2, 3, 10, 20, 30, 40, 50, 60, 70, 80, 90]);
    }
}
