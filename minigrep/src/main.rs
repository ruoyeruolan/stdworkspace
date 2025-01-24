// @Introduce  :
// @File       : main.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/21 00:05
// @Description:

use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // let config = Config::new(&args);
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Serach for {}", config.query);
    // println!("In file {}", config.file_path);

    // let context = fs::read_to_string(config.file_path).expect(
    //     "Should have been able to read the file"
    // );
    // println!("With text:\n{context}");
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    // run(config);
}
