fn main() {
    // Create Vector
    let v1: Vec<i32> = Vec::new();

    // Create Vector with initial elements
    let v2 = vec![1, 2, 3];

    // Mutate Vector
    let mut v3 = vec![1, 2, 3];
    v3.push(4);

    // Iterating over immutable references of v3
    for i in &v3 {
        print!("{}", i);
    }
}
