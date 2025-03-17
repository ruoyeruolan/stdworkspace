// -*- encoding: utf-8 -*-*
// @Introduce  :
// @File       : divide_equal_pairs.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/03/17 17:27
// @Description:

use practices::Solution;

pub trait DivideArray {
    fn divide_array(nums: Vec<i32>) -> bool;
    fn divide_array_(nums: Vec<i32>) -> bool;
}

impl DivideArray for Solution {
    fn divide_array(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let mut hash_map = std::collections::HashMap::new();

        for idx in 0..n {
            *hash_map.entry(nums[idx]).or_insert(0) += 1;
        }

        for (_, &val) in hash_map.iter() {
            if val % 2 != 0 {
                return false;
            }
        }
        true
    }

    fn divide_array_(nums: Vec<i32>) -> bool {
        let mut freq = std::collections::HashMap::new();

        nums.iter()
            .for_each(|&num| *freq.entry(num).or_insert(0) += 1);
        freq.values().all(|&count| count % 2 == 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]), true)
    }
}

fn main() {
    let nums = vec![3,2,3,2,2,2];
    println!("{}", Solution::divide_array_(nums))
}
