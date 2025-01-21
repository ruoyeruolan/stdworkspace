// @Introduce  :
// @File       : gridgame.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/21 20:04
// @Description:

use practices::Solution;
use std::cmp::{max, min};

pub trait GridGame {
    fn grid_game(grid: Vec<Vec<i32>>) -> i64;
}

impl GridGame for Solution {
    fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut firstrowsum = grid[0].iter().map(|&x| x as i64).sum();
        let mut secondrowsum: i64 = 0;
        let mut result = i64::MAX;

        for i in 0..grid[0].len() {
            firstrowsum -= grid[0][i] as i64;
            result = min(result, max(firstrowsum, secondrowsum));
            secondrowsum += grid[1][i] as i64;
        }
        result
    }
}

fn main() {
    let grid = vec![vec![2, 5, 4], vec![1, 5, 1]];
    let result = Solution::grid_game(grid);
    assert_eq!(result, 4);
}
