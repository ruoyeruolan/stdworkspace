// @Introduce  : 
// @File       : lib.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/21 05:00
// @Description:

use std::fs;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub file_path: String
}


impl Config {
    // fn new(args: &[String]) -> Config {

    //     if args.len() < 3 {
    //         panic!("Not enough arguments");
    //     }

    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
        
    //     Config { query, file_path }
    // }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {

            if args.len() < 3 {
                return Err("Not enough arguments");
            }

            let query = args[1].clone();
            let file_path = args[2].clone();

            Ok(Config { query, file_path })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        // let contents = fs::read_to_string(config.file_path) {
        //     Ok(contents) => contents,
        //     Err(e) => return Err(e);
        // };
        let contents = fs::read_to_string(config.file_path)?;  // ? will return the error value from the current function for the caller to handle, simple and clean.
        println!("With text:\n{contents}");
        
        Ok(())
}


// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

