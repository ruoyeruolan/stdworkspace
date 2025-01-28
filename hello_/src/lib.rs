// @Introduce  :
// @File       : lib.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/28 17:31
// @Description:

pub struct ThreadPool;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
