// @Introduce  :
// @File       : main.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/24 23:13
// @Description:

use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// use crate::Message::{ChangeColor, Move, Quit, Write};

fn main() {
    println!("Hello, world!");

    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    println!("x = {}", x);

    let m = MyBox(String::from("Rust"));
    hello(&m);
}
