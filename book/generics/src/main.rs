use core::alloc;

struct List<T> {
    list: Vec<T>,
}

impl List<usize> {
    fn sum(&self) -> usize {
        self.list.iter().sum()
    }
}

impl List<String> {
    fn sum(&self) -> String {
        let mut merged_string = String::new();
        for element in &self.list {
            merged_string.push_str(element.as_str());
        }
        merged_string
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let list_one = List { list: vec![1, 2, 3, 4, 5] };
    let list_two = List { list: vec![String::from("hello"), String::from(" world")] };

    let sum = list_one.sum();
    let merged = list_two.sum();

    println!("Sum -> {}", sum);
    println!("Merged -> {}", merged);

    let p_one = Point { x: 5, y: 10 };
    let p_two = Point { x: 5.0, y: 10.0 };

    println!("p_one.x = {}", p_one.x());
    println!("p_two.distance_from_origin = {}", p_two.distance_from_origin());
}
