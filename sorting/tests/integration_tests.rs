use std::fmt::Debug;
use rand::{
    seq::SliceRandom,
    thread_rng,
};
use sorting;
use test_case::test_case;
use sorting::bubble_sort::sort;

#[test_case((0..20).rev().collect(), (0..20).collect(); "integers")]
#[test_case(vec ! ["c", "b", "a"], vec ! ["a", "b", "c"]; "strings")]
#[test_case(vec ! [6.0, 10.0, 2.0, 4.0, 8.0], vec ! [2.0, 4.0, 6.0, 8.0, 10.0]; "floats")]
fn can_bubble_sort<T: PartialOrd + Copy + Debug>(mut actual: Vec<T>, expected: Vec<T>) {
    // Arrange
    actual.shuffle(&mut thread_rng());

    // Act
    sort(&mut actual);

    // Assert
    assert_eq!(actual, expected);
}