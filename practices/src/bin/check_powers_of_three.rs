// -*- encoding: utf-8 -*-*
// @Introduce  :
// @File       : check_powers_of_three.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/03/04 18:47
// @Description:

use practices::Solution;

pub trait CheckPoersOfThree {
    fn check_powers_of_three(n: i32) -> bool;
    fn check_powers_of_three_(n: i32) -> bool;
}

impl CheckPoersOfThree for Solution {
    fn check_powers_of_three(n: i32) -> bool {
        let mut n = n;

        while n > 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }

    fn check_powers_of_three_(n: i32) -> bool {
        if n == 0 {
            return true;
        }

        if n % 3 == 2 {
            return false;
        }
        Self::check_powers_of_three_(n / 3)
    }
}
