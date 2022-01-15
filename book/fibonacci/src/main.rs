use std::io;

fn main() {
    fibonacci();
}

fn fibonacci() {
    let (start, end): (u32, u32) = get_fibonacci_range();
    print!("{}..{} Fibonacci Numbers: [", start, end);

    for index in start..=end {
        if index == end {
            print!("{}", get_next_num(index));
        } else {
            print!("{}, ", get_next_num(index));
        }
    }

    print!("]");
}

fn get_next_num(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    return get_next_num(n - 1) + get_next_num(n - 2);
}

fn get_fibonacci_range() -> (u32, u32) {
    let mut buffer: String = String::new();

    println!("Enter Range of Fibonacci Numbers To View\nFormat -> [u32 u32] E.g 1 5");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed To Read In Line");

    let ranges: Vec<&str> = buffer.split(" ").collect();
    return (ranges[0].trim().parse().unwrap(), ranges[1].trim().parse().unwrap());
}