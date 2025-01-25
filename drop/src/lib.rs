// @Introduce  :
// @File       : lib.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/25 15:22
// @Description:

struct CustomeSmartPointer {
    data: String,
}

impl Drop for CustomeSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomeSmartPointer with data `{}`!", self.data); // run when the instance is out of scope
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    pub fn new(meaaager: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messager: meaaager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max > 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max > 0.9 {
            self.messager
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max > 0.75 {
            self.messager
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

struct MockMessager {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessager {
    fn new() -> MockMessager {
        MockMessager {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messager for MockMessager {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = CustomeSmartPointer {
            data: String::from("my stuff"),
        };

        let d = CustomeSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomeSmartPointer created.");
        println!("{:?}, {:?}", c.data, d.data);
        drop(c);
        // println!("{}",c.data); // error: value used here after move
    }

    #[test]
    // fn test_list() {
    //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //     println!("count after creating a = {}", Rc::strong_count(&a));
    //     let b = Cons(3, Rc::clone(&a));
    //     println!("Count after creating b = {}", Rc::strong_count(&a));
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("Count after creating c = {}", Rc::strong_count(&a));
    // }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messager.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn it_works_() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {a:?}");
        println!("b after = {b:?}");
        println!("c after = {c:?}");
    }
}
