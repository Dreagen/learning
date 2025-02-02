use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!(
        "Breaking news! {} and {}",
        item1.summarize(),
        item2.summarize()
    );
}

// pub fn notify<T>(item1: &T, item2: &T)
// where
//     T: Summary + Display,
// {
// }

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello twittersphere!"),
        reply: false,
        retweet: false,
    };

    let news_article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling."),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("News Article summary: {}", news_article.summarize());

    notify(&news_article, &tweet);
}
