// @Introduce  : 
// @File       : lib.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/19 16:32
// @Description:

use std::cmp::{max}

pub struct Solution;

pub trait MaxArea {
    fn max_area(height: Vec<i32>) -> i32 {
        height[0] * height.len() as i32
    }
}

