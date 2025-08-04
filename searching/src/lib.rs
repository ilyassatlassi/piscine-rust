pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut res = 0;
    let mut not_found = false;
    for i in 0..array.len() {
        if array[i] == key {
            res = i;
            not_found = true
        }
    }
    if !not_found {
        return None;
    }
    Some(res)
}

#[test]
fn test_search_a_value_in_an_array() {
    assert_eq!(search(&[6], 6), Some(0));
    assert_eq!(search(&[1, 2], 1), Some(0));
    assert_eq!(search(&[1, 2], 2), Some(1));
}
#[test]
fn test_middle_of_an_array() {
    assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
}

#[test]
fn test_beginning_of_an_array() {
    assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
}

#[test]
fn test_end_of_an_array() {
    assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
}

#[test]
fn test_long_array() {
    assert_eq!(
        search(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
        Some(9)
    );
    assert_eq!(
        search(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
        Some(5)
    );
}

#[test]
fn test_with_duplicates() {
    assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11, 1], 1), Some(7));
    assert_eq!(search(&[1, 3, 9, 6, 8, 9, 11], 9), Some(5));
}

#[test]
fn test_value_is_not_included() {
    assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 7), None);
    assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 0), None);
    assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 13), None);
    assert_eq!(search(&[], 1), None);
}