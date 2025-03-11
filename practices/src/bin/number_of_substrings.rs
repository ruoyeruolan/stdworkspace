// -*- encoding: utf-8 -*-*
// @Introduce  :
// @File       : number_of_substrings.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/03/11 18:04
// @Description:

use std::collections::HashMap;

use practices::Solution;

pub trait CountOfSubstring {
    fn count_of_substrings(s: String) -> i32;
    fn count_of_substrings_(s: String) -> i32;
}

impl CountOfSubstring for Solution {
    fn count_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let (mut count, mut start, mut end) = (0, 0, 0);
        let mut freq = vec![0; 3];

        while end < n {
            freq[(s[end] - b'a') as usize] += 1;

            while freq[0] > 0 && freq[1] > 0 && freq[2] > 0 {
                count += (n - end) as i32;

                freq[(s[start] - b'a') as usize] -= 1;
                start += 1;
            }
            end += 1;
        }
        count
    }

    fn count_of_substrings_(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let (mut count, mut start, mut end) = (0, 0, 0);
        let mut freq = HashMap::new();

        while end < n {
            *freq.entry(s[end]).or_insert(0) += 1;

            while freq.len() == 3 {
                count += n - end;

                let start_letter = s[start];
                if let Some(val) = freq.get_mut(&start_letter) {
                    *val -= 1;

                    if *val == 0 {
                        freq.remove(&start_letter);
                    }
                }
                start += 1;
            }
            end += 1;
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::count_of_substrings("abcabc".to_string()), 10);
    }

    // #[test]
    // #[should_panic]
    // fn test_example_2() {
    //         // Solution::count_of_substrings("aaaabbbbcccc".to_string())
    //         Solution::count_of_substrings("abcabc".to_string())
    // }
}

fn main() {
    let a = Solution::count_of_substrings("abcabc".to_string());
    println!("{}", a)
}
