// @Introduce  : 
// @File       :  Vectors.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/14 18:43
// @Description:  Storing Lists of Values with Vectors

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];  // immutable borrow occurs here, cannot modify `v`
    let first = &mut v[0];
    // v.push(6);
    *first = 6;
    println!("The first element is: {first}");
}