use std::process;
enum GroupType {
    Char,
    Digit
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
        } else { 
            Self {
                representation: "\\d".to_owned(),
                represents: GroupType::Digit
            }
        }
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
                let mut contains_number = false;
                for c in text.chars(){
                    if c.is_numeric(){
                        contains_number = true;
                        break
                    }
                }
                if (contains_number) {
                    process::exit(0)
                } else {
                    process::exit(1)
                }
            }
        }
    }
    
}