// @Introduce  :
// @File       : countBadPairs.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/02/09 19:12
// @Description:

use core::num;
use std::collections::btree_set::Difference;

use practices::Solution;

pub trait CountBadPairs {
    fn count_bad_pairs(nums: Vec<i32>) -> i64;
    fn count_bad_pairs_(nums: Vec<i32>) -> i64;
}

impl CountBadPairs for Solution {
    fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut good_pairs = 0;
        let total_pairs = (n * (n - 1) / 2) as i64;
        let mut map = std::collections::HashMap::new();

        for i in 0..n {
            let diff = (nums[i] - i as i32) as i64;

            if let Some(&count) = map.get(&diff) {
                good_pairs += count;
            }

            *map.entry(diff).or_insert(0) += 1;
        }
        total_pairs - good_pairs
    }

    fn count_bad_pairs_(nums: Vec<i32>) -> i64 {
        let mut hash: std::collections::HashMap<i32, i64> = std::collections::HashMap::new();
        let mut result = 0;
        for idx in 0..nums.len() {
            result += idx as i64 - hash.get(&(idx as i32 - nums[idx])).unwrap_or(&0);
            *hash.entry(idx as i32 - nums[idx]).or_insert(0) += 1;
        }
        result
    }
}
