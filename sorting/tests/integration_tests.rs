use rand::seq::SliceRandom;
use sorting;
use rand::thread_rng;

#[test]
fn bubble_sort_ascending() {
    // Arrange
    let mut vec: Vec<usize> = (0..20).rev().collect();
    vec.shuffle(&mut thread_rng());

    // Act
    sorting::bubble_sort::sort(&mut vec);

    // Assert
    for i in 0..20 {
        assert_eq!(vec[i], i);
    }
}