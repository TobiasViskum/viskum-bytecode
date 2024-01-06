pub fn is_digit(c: &str) -> bool {
    let ch = c.chars().next().unwrap();

    ch >= '0' && ch <= '9'
}

pub fn is_alphabetic(c: &str) -> bool {
    let ch = c.chars().next().unwrap();

    ch.is_alphabetic() || ch == '_'
}
