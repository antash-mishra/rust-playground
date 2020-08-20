//defining traits
pub trait Summary {
    fn summarize(&self) -> String;
}
//defining traits
pub trait Summary2 {
    fn summarize2_author(&self) -> String;
    fn summarize2(&self) -> String {
        format!("Read more from: {}", self.summarize2_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub  content: String,
}

//
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary2 for NewsArticle {
    fn summarize2_author(&self) -> String {
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
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary2 for Tweet {
    fn summarize2_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("{}", tweet.summarize2());
    let article = NewsArticle {
        headline: String::from("Dirty bitches"),
        location: String::from("Haveli"),
        author: String::from("Bhadwa"),
        content: String::from("Bitches are gonna fuck around. Fuck those bitches and those fuck boys"),
    };
    println!("{}", article.summarize2());

    let tweet2 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probaby know"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize2())

}


