pub trait Summary {
    fn summarise_author(&self) -> String;

    fn summarise(&self) -> String {
        format!("(Read more from {}...)", self.summarise_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarise_author(&self) -> String {
        format!("@{}", self.author)
    }
    // fn summarise(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
    //
    // fn summarise(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

fn returns_summarise() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}