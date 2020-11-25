#[allow(dead_code)]

/// You are given an array **arr[]** of **N** integers including 0. The task is to find the smallest positive number missing from the array.
/// Your Task:
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
