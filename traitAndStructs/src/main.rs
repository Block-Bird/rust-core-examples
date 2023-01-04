use std::fmt::format;

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
    fn summarize (&self ) -> String {
        format("{} by {}", self.headline, self.author)
    } 
}