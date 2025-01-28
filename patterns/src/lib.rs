// @Introduce  :
// @File       : lib.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/28 14:33
// @Description:

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Hello { id: i32 },
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn ignore_part_value() {
        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }

    #[test]
    fn ignore_part_value_() {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
    }

    #[test]
    fn match_guard() {
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("{x} is even"),
            Some(x) => println!("{}", x),
            None => (),
        }
    }

    #[test]
    fn foo() {
        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { 
                id: id_variable @ 3..=7,
             } => println!("Found an id in range: {}", id_variable),

             Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
    }
}
