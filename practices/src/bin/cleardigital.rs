// @Introduce  :
// @File       : cleardigital.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/02/10 19:00
// @Description:

use std::marker::PhantomPinned;

use practices::Solution;

pub trait ClearDigits {
    fn clear_digits(s: String) -> String;
}

impl ClearDigits for Solution {
    fn clear_digits(s: String) -> String {
        let mut result = Vec::new();

        for val in s.chars() {
            if val.is_ascii_digit() && !result.is_empty() {
                result.pop();
            } else {
                result.push(val);
            }
        }
        result.into_iter().collect()
    }
}
