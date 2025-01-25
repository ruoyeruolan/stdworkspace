// @Introduce  :
// @File       : lib.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/25 15:41
// @Description:

struct CustomeSmartPointer {
    data: String,
}

impl Drop for CustomeSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomeSmartPointer with data `{}`!", self.data); // run when the instance is out of scope
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = CustomeSmartPointer {
            data: String::from("my stuff"),
        };

        let d = CustomeSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomeSmartPointer created.");
        println!("{:?}, {:?}", c.data, d.data);
        drop(c);
        // println!("{}",c.data); // error: value used here after move
    }
}
