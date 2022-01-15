pub mod bubble_sort {
    pub fn sort(vec: &mut Vec<usize>) {
        for outer in (0..vec.len()).rev() {
            for inner in 0..outer {
                if vec[inner] > vec[inner + 1] {
                    super::swap(vec, inner, inner + 1);
                }
            }
        }
    }
}

fn swap(vec: &mut Vec<usize>, index_one: usize, index_two: usize) {
    let temp = vec[index_two];
    vec[index_two] = vec[index_one];
    vec[index_one] = temp;
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn can_swap_numbers() {
        // Arrange
        let mut vec = vec![1, 2];

        // Act
        swap(&mut vec, 0, 1);

        // Assert
        assert_eq!(vec[0], 2);
        assert_eq!(vec[1], 1);
    }
}