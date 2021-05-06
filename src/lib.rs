// サブディレクトリにモジュールを置く場合はpathアトリビュートでパスを指定しないと
// コンパイルエラーになる
#[path = "lib/traits.rs"]
pub mod traits;
use traits::Summary;

pub struct Tweet {
    pub author: String,
    pub text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        self.author.clone() + ". " + &self.text
    }
    fn first_word(&self) -> &str {
        let bytes = self.text.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &self.text[..i];
            }
        }
        &self.text[..]
    }
}

pub mod back_of_house {
    pub struct BreakFast {
        pub toast: String,
        #[allow(dead_code)]
        seasonal_fruit: String,
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple")
            }
        }
    }
}