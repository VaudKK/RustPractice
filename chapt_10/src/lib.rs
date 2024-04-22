use std::fmt::Display;

pub struct Pair<T>{
    x: T,
    y: T
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub healdine: String,
    pub location: String,
    pub author: String,
    pub content: String
}


impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.healdine,self.author,self.location)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify_cleaner<T: Summary>(item: T){
    println!("Breaking news: {}",item.summarize());
}

pub fn notify<T>(item: T)
where T: Summary
{
    println!("Breaking news: {}",item.summarize());
}

pub fn notify_other_style(item: &impl Summary){
    println!("Breaking news: {}",item.summarize());
}

//conditional 

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}