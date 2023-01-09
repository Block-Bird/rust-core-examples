use std::{fmt::format, iter::Sum};
use std::fmt::Display; 
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
    fn summarize (&self ) -> String{
        String::from("Read More ....")
    }
    fn Summary (&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize (&self) -> String {
        format!("{} by {}", self.author, self.headline)
    }
    fn Summary (&self) -> String {
        format!("Default Loading ... ")
    }

}

impl Summary for Tweet {
    fn summarize (&self ) -> String {
        format!("{}  by {}", self.username, self.content)
    }
    fn Summary (&self ) -> String {
        format!("{} by {}", self.username, self.content)
    }
}

fn main () {
    pub fn printline() {
        println!("Here we Go");
    }
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
    // println!("Tweet Summary {}", tweet.summarize()); 
    // println!("News Article {}", newsArticle.summarize()); 

    // pub fn notify (item:  &impl Summary) {
    //     println!("Here's Notify Function {}", item.summarize());
    // }
    pub fn notify<T: Summary>(item: &T) {
        println!("Here is Notify Fun {:?}", item.summarize());
    }

}

