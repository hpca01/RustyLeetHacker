/// You are given an array **arr[]** of **N** integers including 0. The task is to find the smallest positive number missing from the array.
/// The task is to complete the function missingNumber() which returns the smallest positive missing number in the array.
///
/// Expected Time Complexity: O(N).
/// Expected Auxiliary Space: O(1).
pub fn find_missing_int(mut arr: Vec<i32>, _: i32) -> i32 {
    arr.sort();
    let mut temp = 0;
    for num in arr {
        if num == temp + 1 {
            temp += 1
        }
    }
    temp + 1
}

/// Given an array **A[]** of **N** positive integers which can contain integers from 1 to N where elements can be repeated or can be absent from the array.
/// Your task is to count the frequency of all elements from 1 to N.
///
/// Expected Time Complexity: O(N)
/// Expected Auxiliary Space: O(1)
pub fn find_frequencies(arr: &mut Vec<i32>, size: usize) {
    for each in arr.iter_mut() {
        *each -= 1;
    }

    for i in 0..size {
        let v = *arr.get(i).unwrap() % size as i32;
        arr[v as usize] += size as i32;
    }

    for i in 0..size {
        arr[i] = arr[i] / size as i32;
    }
}
