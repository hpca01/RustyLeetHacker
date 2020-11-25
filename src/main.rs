mod missing_num;

fn main() {
    println!("Hello, world!");
}

#[test]
fn find_missing_int() {
    let result = missing_num::find_missing_int(vec![1, 2, 3, 4, 5], 5);
    assert_eq!(result, 6);
}

#[test]
fn find_missing_positive_int_with_negatives() {
    let result = missing_num::find_missing_int(vec![0, -10, 1, 3, -20], 5);
    assert_eq![result, 2];
}
