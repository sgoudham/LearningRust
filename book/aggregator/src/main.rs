use aggregator::{NewsArticle, Summary, Tweet};

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_no_copy<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = list.get(0).expect("oh no");

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: \n\t{}", tweet.summarise());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available: \n\t{}", article.summarise());

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_no_copy(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_no_copy(&char_list);
    println!("The largest char is {}", result);
}