use std::string::ToString;

pub trait Summary {
    fn summarize(&self) -> String;
    fn first_word(&self) -> &str;
    fn default(&self) -> String {
        "default words".to_string()
    }
}

// ブランケット実装
// Displayトレイトを実装したあらゆる型に対してToStringの実装を強制している
// impl<T: Display> ToString for T {
//     // --snip--
// }

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
