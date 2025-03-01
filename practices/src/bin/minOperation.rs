// @Introduce  :
// @File       : minOperation.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/02/13 19:31
// @Description:

use core::num;
use std::{cmp::Reverse, collections::BinaryHeap};

use practices::Solution;

pub trait MinOperations {
    fn min_operations(nums: Vec<i64>, k: i64) -> i32;
}

impl MinOperations for Solution {
    fn min_operations(nums: Vec<i64>, k: i64) -> i64 {
        let mut min_heap: BinaryHeap<Reverse<i64>> = nums.into_iter().map(Reverse).collect();
        let mut ops = 0;

        while let Some(&Reverse(min_val)) = min_heap.peek() {
            if min_val >= k {
                break;
            }

            if min_heap.len() < 2 {
                return -1;
            }

            let Reverse(x) = min_heap.pop().unwrap();
            let Reverse(y) = min_heap.pop().unwrap();

            min_heap.push(Reverse(2 * x + y));
            ops += 1;
        }
        ops
    }
}
