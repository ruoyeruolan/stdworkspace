// -*- encoding: utf-8 -*-*
// @Introduce  :
// @File       : allocate_candies.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/03/14 18:38
// @Description:

use practices::Solution;

pub trait MaximumCandies {
    fn maximum_candies(candies: Vec<i32>, k: i64) -> i32;
    fn can_allocate_candies(candies: &[i32], k: i64, num_candies: i32) -> bool;
}

impl MaximumCandies for Solution {
    fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut candies: Vec<i32> = candies;
        candies.sort();

        let (mut left, mut right) = (0, *candies.last().unwrap());

        while left < right {
            let mid: i32 = (left + right + 1) / 2;

            if Self::can_allocate_candies(&candies, k, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }

    fn can_allocate_candies(candies: &[i32], k: i64, num_candies: i32) -> bool {
        let mut max_num_of_children: i64 = 0;

        for &pile in candies {
            max_num_of_children += (pile / num_candies) as i64;
        }
        max_num_of_children >= k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let candies = vec![5, 8, 6];
        assert_eq!(Solution::maximum_candies(candies, 3), 5);
    }
}

fn main() {
    let candies = vec![2, 5];
    let num = Solution::maximum_candies(candies, 11);
    println!("{}", num)
}
