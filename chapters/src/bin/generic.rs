// @Introduce  : 
// @File       : generic.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/18 15:48
// @Description:


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
}