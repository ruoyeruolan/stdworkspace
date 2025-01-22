// @Introduce  :
// @File       : main.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/22 21:03
// @Description: closure in rust is a function that like a lambda function in python
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stacked())
    }

    fn most_stacked(&self) -> ShirtColor {
        let mut nred = 0;
        let mut nblue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => nred += 1,
                ShirtColor::Blue => nblue += 1,
            }
        }

        if nred > nblue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}

fn main() {
    // println!("Hello, world!");
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let list = vec![1, 2, 3];
    println!("list: {:?}", list);

    let only_borrow = || println!("list: {:?}", list);
    only_borrow();
    println!("list: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // println!("After defining closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("Inside closure: {list:?}")).join().unwrap();
}
