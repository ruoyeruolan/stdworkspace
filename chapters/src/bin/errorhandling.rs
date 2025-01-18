// @Introduce  : 
// @File       : errorhandling.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/18 15:30
// @Description:

use std::fs::File;

fn main() {
    let greeeting_file_result = File::open("hello.txt");

    let greeting_file = match greeeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}