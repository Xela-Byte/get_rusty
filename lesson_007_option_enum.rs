pub fn test_option_type() -> Option<u8> {
    let mut num: Option<u8> = None;
    num = Some(8);
    return num;
}

pub fn test_option_string() -> Option<String> {
    let name: Option<String> = Some(String::from("goalllllll"));
    return name;
}

pub enum CharacterType {
    Mage,
    Archer,
    Warrior,
}

pub fn test_option_chartype() -> Option<CharacterType> {
    let mut my_char: Option<CharacterType> = None;
    my_char = Some(CharacterType::Archer);
    return my_char;
}

impl ToString for CharacterType {
    fn to_string(&self) -> String {
        match self {
            CharacterType::Archer => String::from("Archer"),
            CharacterType::Mage => String::from("Mage"),
            CharacterType::Warrior => String::from("Warrior"),
        }
    }
}
