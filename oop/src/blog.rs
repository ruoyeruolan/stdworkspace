// @Introduce  :
// @File       : blog.rs
// @Author     : ryrl
// @Email      : ryrl970311@gmail.com
// @Time       : 2025/01/27 16:50
// @Description:

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        // println!("post content: {}", post.content()); // DraftPost does not have a content method， Only Post has a content method

        let post = post.request_review();
        // println!("post content: {}", post.content());

        let post = post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
