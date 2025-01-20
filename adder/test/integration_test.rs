// @Introduce  : 
// @File       : integration_test.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/20 20:51
// @Description:

use adder::add_two;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}