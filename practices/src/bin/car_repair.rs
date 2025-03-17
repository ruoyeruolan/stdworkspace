// -*- encoding: utf-8 -*-*
// @Introduce  :
// @File       : car_repair.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/03/16 19:20
// @Description:

use practices::Solution;

pub trait RepairCars {
    fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64;
}

impl RepairCars for Solution {
    fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let (mut low, mut high) = (1, (cars * cars * ranks[0]) as i64);

        while low < high {
            let mid = (low + high) / 2;
            let cars_repaired: i64 = ranks
                .iter()
                .map(|&rank| (mid as f64 / rank as f64).sqrt() as i64)
                .sum();

            match cars_repaired.cmp(&(cars as i64)) {
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => high = mid,
                std::cmp::Ordering::Less => low = mid + 1,
            }
        }
        low as i64
    }
}
