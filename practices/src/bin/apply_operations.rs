// @Introduce  :
// @File       : apply_operations.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/03/01 18:23
// @Description:

use practices::Solution;

pub trait ApplyOperations {
    fn apply_operations(nums: Vec<i32>) -> Vec<i32>;
}

impl ApplyOperations for Solution {
    fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();

        for idx in 0..n - 1 {
            if nums[idx] == nums[idx + 1] && nums[idx] != 0 {
                nums[idx] *= 2;
                nums[idx + 1] = 0;
            }
        }

        let mut res: Vec<i32> = nums.into_iter().filter(|&x| x != 0).collect();
        res.resize(n, 0);
        res
    }
}
