// @Introduce  : 
// @File       : generic.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/18 15:48
// @Description:

use std::fmt::Display
use chapters::{NewsArticle, Summary, Tweet};
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut larget = &list[0];

//     for item in list {
//         if item > larget {
//             larget = item;
//         }
//     }
//     larget
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut larget = &list[0];

//     for item in list {

//         if item > larget {
//             larget = item;
//         }
//     }
//     larget
// }

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut larget = &list[0];

    for item in list {
        if item > larget {
            larget = item;
        }
    }
    larget
}


// struct
struct Point<T> {
    x: T,
    y: T,
}

// enum 
// enum Option<T> {
//     Some(T),
//     None,
// }

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// impl Point<i32> {
//     fn distance_from_origin(&self) -> i32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // 'a is a lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }  // x and y in different scope but the same lifetime
}


struct ImportantExcept<'a> {
    part: &'a str,
}


fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where T: Display {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    let largest = largest(&number_list);
    println!("The largest number is {}", largest);

    let intefer = Point { x: 5, y: 10 };
    let float = Point { x: 1.1, y: 4.2 };

    println!("integer.x = {}, integer.y = {}, float.x = {}, float.y = {}", intefer.x, intefer.y, float.x, float.y);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}, p.y = {}", p.x(), p.y());


    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result  = longest(string1.as_str(), &string2.as_str());
        println!("The longest string is {result}")
    }


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcept {
        part: first_sentence,
    };
    println!("{}", i.part);

    {
        let s: &'static str = "I have a static lifetime.";
        println!("{}", s);
    }
    // println!("{}", s); // {} is scope, scope is not equal to lifetime
}