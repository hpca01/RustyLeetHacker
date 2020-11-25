#![allow(dead_code)]

mod arrays;

fn main() {
    println!("Hello, world!");
}

#[test]
fn find_missing_int() {
    let result = arrays::find_missing_int(vec![1, 2, 3, 4, 5], 5);
    assert_eq!(result, 6);
}

#[test]
fn find_missing_positive_int_with_negatives() {
    let result = arrays::find_missing_int(vec![0, -10, 1, 3, -20], 5);
    assert_eq![result, 2];
}

#[test]
fn find_frequencies() {
    let mut array: Vec<i32> = vec![2, 3, 2, 3, 5];
    let size = array.len();
    arrays::find_frequencies(&mut array, size); // passed by ref
    assert_eq!(array, vec![0, 2, 2, 0, 1]);
}
