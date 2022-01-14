fn main() {
    // Create Mutable String
    let mut s: String = String::new();

    // Create String From Literal
    let data: &str = "initial contents";
    let s: String = data.to_string();
    let s: String = "initial contents".to_string();

    // Create String
    let s: String = String::from("initial contents");

    // Update String
    let mut s: String = String::from("foo");
    let bar: &str = "bar";
    s.push_str(bar);
    println!("s is -> {}", s);

    // Concatenating Multiple String
    let s1: String= String::from("tic");
    let s2: String= String::from("tac");
    let s3: String = String::from("toe");

    let s: String = format!("{}-{}-{}", s1, s2, s3);
    println!("s is -> {}", s);

    // Iterating Over Strings
    for char in "Goudham".chars() {
        println!("{}", char);
    }
}
