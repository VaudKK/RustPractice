use std::fmt::Display;

use chapt_10::{notify, Tweet};

struct Point<T>{
    x:T,
    y:T
}

impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }

    fn y(&self) -> &T{
        &self.y
    }
}


fn main() {
    let numbers = vec![10,20,30,405,30,50,60,80];

    println!("The larget number is: {}", largest(&numbers));

    let point = Point{x: 5, y:7};

    println!("The coordinates are x: {}, y: {}", point.x(), point.y());

    let tweet = Tweet{
        username: String::from("John Doe"),
        content: String::from("Testing rust traits"),
        reply: false,
        retweet: false
    };

    notify(tweet);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number
        }
    }

    return largest
}


// lifetimes

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    }else{
        y
    }
}

// all together

fn longest_with_an_announcement<'a,T>
(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where T : Display {
    println!("Anouncement {}", ann);
    if x.len() > y.len() {
        x
    }else{
        y
    }
}
