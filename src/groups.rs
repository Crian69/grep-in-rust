use std::process;
enum GroupType {
    Char,
    Digit,
    Word
}
pub struct Group {
    representation: String,
    represents: GroupType
}

impl Group {
    pub fn new(pattern: String) -> Self {
        if pattern.chars().count() == 1 {
            Self {
                representation: pattern,
                represents: GroupType::Char
            }
        } else if pattern == "\\d" {
            Self {
                representation: "\\d".to_owned(),
                represents: GroupType::Digit
            }
        } else {
            Self {
                representation: "\\w".to_owned(),
                represents: GroupType::Word
            }
        }
    }

    fn contains_number(text: &str) -> bool {
        for c in text.chars(){
            if c.is_numeric(){
                return true
            }
        }
        false
    }

    fn contains_alphabet(text: &str) -> bool {
        for c in text.chars(){
            if c.is_alphabetic(){
                return true
            }
        }
        false
    }

    fn contains_underscore(text: &str) -> bool {
        text.contains('_')
    }

    pub fn match_text(&self, text: &str) -> () {
        match self.represents {
            GroupType::Char => {
                let str = self.representation.as_str();
                if text.contains(str) {
                    process::exit(0)
                } else{
                    process::exit(1)
                }
            }
            GroupType::Digit => {
                let contains_number = Self::contains_number(text);
                if contains_number {
                    process::exit(0)
                } else {
                    process::exit(1)
                }
            },
            GroupType::Word => {
                if Self::contains_alphabet(text) || Self::contains_underscore(text) || Self::contains_number(text) {
                    process::exit(0)
                } else {
                    process::exit(1)
                }
            }
        }
    }

}