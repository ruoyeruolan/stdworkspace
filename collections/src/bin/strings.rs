// @Introduce  : 
// @File       : strings.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/15 22:45
// @Description:

use std::fmt::format;

fn main() {
    let s = String::new();
    println!("{}", s);

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents".to_string();
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);
    
    let mut s = String::from("foo");
    s.push_str(" bar");
    // s.push('3');
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    let s3 = String::from("bar");
    println!("s2 is {}; s1 is {}", s2, s1);
    s1.push_str(s2);
    print!("s2 is {}; s1 is {} \n", s2, s1);
    s1.push_str(&s3);
    println!("s3 is {}; s1 is {}", s2, s1);

    let mut s = String::from("lo");
    s.push('l');  // must be a single character

    let s1 = String::from("Hello, ");
    let mut s2 = String::from("world!");
    let s3 = s1 + &s2;  // note s1 has been moved here and can no longer be used
    // println!("{}", s1);  // Error
    println!("{}", s3);
    // println!("{}", s2);
    s2.push_str("AAA");
    println!("{}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);


    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    for c in hello.chars() {
        println!("{}", c);
    }

    for  b in hello.bytes() {
        println!("{}", b);
    }
}