pub mod bubble_sort {
    pub fn sort<T: PartialOrd + Copy>(vec: &mut Vec<T>) {
        for outer in (0..vec.len()).rev() {
            for inner in 0..outer {
                if vec[inner] > vec[inner + 1] {
                    super::swap(vec, inner, inner + 1);
                }
            }
        }
    }
}

fn swap<T: PartialOrd + Copy>(vec: &mut Vec<T>, index_one: usize, index_two: usize) {
    let temp = vec[index_two];
    vec[index_two] = vec[index_one];
    vec[index_one] = temp;
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use std::fmt::Debug;
    use test_case::test_case;

    #[test_case(vec ! [1, 2], vec ! [2, 1]; "integers")]
    #[test_case(vec ! ["hello", "world"], vec ! ["world", "hello"]; "strings")]
    #[test_case(vec ! [5.0, 10.0], vec ! [10.0, 5.0]; "floats")]
    fn can_swap<T: PartialOrd + Copy + Debug>(mut actual: Vec<T>, expected: Vec<T>) {
        // Act
        swap(&mut actual, 0, 1);

        // Assert
        assert_eq!(actual, expected);
    }
}