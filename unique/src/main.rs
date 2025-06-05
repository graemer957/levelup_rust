use std::collections::HashSet;
use std::hash::Hash;

// advanced 1: use generic types
/*
fn unique<T>(mut a: Vec<T>) -> Vec<T>
where
    T: Ord,
{
    a.sort_unstable();
    a.dedup();
    a
}
*/

// advanced 2: keep items in order
// advanced 3: use iterators
fn unique<T: Eq + Hash + Copy>(a: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    a.into_iter().filter(|&item| seen.insert(item)).collect()
}

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {answer:?}");
}

#[test]
fn empty_list() {
    // Needed to specify type when switching to generics
    let input: Vec<i32> = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
// Added test for checking order is preserved
fn keep_order() {
    let input = vec![1, 5, 5, 2, 2, 3, 1];
    let expected_output = vec![1, 5, 2, 3];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
