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