fn count_zeroes_mixed_strategy(array: &[i32]) -> i32 {
    let n = array.len() as i32;
    if n == 0 { return 0; }

    let mut left = 0;
    let mut right = n - 1;

    while (right - left).abs() > 1 {
        let mid = left + (right - left) / 2;
        if array[mid as usize] == 0 {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    let mut janitor_count = 0;
    let mut i = left;
    while i <= right && i < n { 
        if array[i as usize] == 0 {
            janitor_count += 1;
        } else {
            break; 
        }
        i += 1;
    }

    left + janitor_count
}

fn main() {
    let test_cases = vec![
        vec![0, 0, 0, 3, 2, 8],
        vec![0, 0],
        vec![1, 2, 3],
        vec![0],
        vec![],
    ];

    for (idx, test) in test_cases.iter().enumerate() {
        println!("Test {}: {:?} -> Count: {}", idx + 1, test, count_zeroes_mixed_strategy(test));
    }
}
