use std::fmt::{Debug, Display};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        String::from("test")
    }
}

// impl Clone for NewsArticle {
//     fn clone(&self) -> Self {
//         return self;
//     }
// }
//
impl Clone for NewsArticle {
    fn clone(&self) -> Self {
        *self
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
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl PartialEq for Tweet {
    fn eq(&self, other: &Self) -> bool {
        if self.content == other.content {
            return true
        } else {
            return false
        }
    }
}

impl PartialOrd for Tweet {
    fn gt(&self, other: &Self) -> bool {
        return false;
    }

    fn ge(&self, other: &Self) -> bool {
        return false;
    }

    fn le(&self, other: &Self) -> bool {
        return false;
    }

    fn lt(&self, other: &Self) -> bool {
        return false;
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!();
    }

}

pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

fn some_fn<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    todo!();
}

fn some_fn1<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Display + Debug,
{
    todo!();
}


// This doesn't compile 
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Tweet {
//             username: String::from("@laureanray"),
//             content: String::from("Hello Werld!"),
//             retweet: false,
//             reply: false,
//         }
//     } else {
//         NewsArticle {
//             headline: String::from("headline"),
//             location: String::from("mindoro"),
//             author: String::from("lr"),
//             content: String::from("news"),
//         }
//     }
// }


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tweet = Tweet {
            content: String::from("Hello World!"),
            username: String::from("@laureanray"),
            reply: false,
            retweet: false,
        };

        notify(&tweet);
    }

    #[test]
    fn pair() {
        let tweet1 = Tweet { 
            content: String::from("Hello World!"),
            username: String::from("@laureanray"),
            reply: false,
            retweet: false,
        };

        let tweet2 = Tweet { 
            content: String::from("Hello World!"),
            username: String::from("@laureanray"),
            reply: false,
            retweet: false,
        };

        let na = NewsArticle {
            headline: String::from("headline"),
            location: String::from("mindoro"),
            author: String::from("lr"),
            content: String::from("news"),
        };

        let pair = Pair { 
            x: tweet1,
            y: tweet2,
        };

        pair.cmp_display();
    }
}
