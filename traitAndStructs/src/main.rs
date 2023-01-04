use std::{fmt::format, iter::Sum};

pub struct NewsArticle {
    pub author: String, 
    pub headline: String, 
    pub content: String, 
}

pub struct Tweet {
    pub username : String, 
    pub content: String, 
    pub reply: bool, 
    pub retweet: bool, 
}

pub trait Summary {
    fn summarize (&self ) -> String; 
}

impl Summary for NewsArticle {
    fn summarize (&self) -> String {
        format!("{} by {}", self.author, self.headline)
    }
}

impl Summary for Tweet {
    fn summarize (&self ) -> String {
        format!("{}  by {}", self.username, self.content)
    }
}

fn main () {
    let tweet = Tweet {
        username: String::from("Asad Ali"), 
        content: String::from("Block chain developer"), 
        reply: false, 
        retweet: false, 
    };
    let newsArticle = NewsArticle {
        author: String::from("Ahmad Awan"), 
        headline: String::from("office is Open"), 
        content: String::from("Nothing is content"), 
    };
    println!("Tweet Summary {}", tweet.summarize()); 
    println!("News Article {}", newsArticle.summarize()); 
}