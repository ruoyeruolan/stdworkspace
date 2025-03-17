// -*- encoding: utf-8 -*-*
// @Introduce  :
// @File       : house_robber.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/03/15 18:54
// @Description:

use practices::Solution;

pub trait MinCapability {
    fn min_capability(nums: Vec<i32>, k: i32) -> i32;
}

impl MinCapability for Solution {
    fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let houses = nums.len();
        let (mut min_, mut max_) = (1, *nums.iter().max().unwrap());

        while min_ < max_ {
            let mid = (min_ + max_) / 2;
            let mut house_stealed = 0;

            let mut idx = 0;
            while idx < houses {
                if nums[idx] <= mid {
                    house_stealed += 1;
                    idx += 2;
                } else {
                    idx += 1;
                }
            }

            if house_stealed >= k {
                max_ = mid;
            } else {
                min_ = mid + 1;
            }
        }
        min_
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::min_capability(vec![2, 3, 5, 9], 2), 5)
    }
}

fn main() {
    let nums = vec![2, 3, 5, 9];
    let res = Solution::min_capability(nums, 2);
    println!("{}", res)
}
