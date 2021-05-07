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

pub enum Language {
    Japanese,
    English,
}

use std::str::FromStr;
impl FromStr for Language {
    type Err = ();
    fn from_str(lang: &str) -> Result<Self, Self::Err> {
        match lang {
            "ja" => Ok(Language::Japanese),
            "en" => Ok(Language::English),
            _ => Err(())
        }
    }
}

pub fn sample_function() -> u32 {
    239
}
