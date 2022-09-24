/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let clean_isbn = str::replace(isbn, "-", "");
    if clean_isbn.len() != 10 {
        return false;
    }
    let mut ans = 0;
    for (i, c) in clean_isbn.char_indices() {
        if c.is_numeric() {
            ans += c.to_digit(10).unwrap() * (10 - i as u32);
        } else if i == 9 && c == 'X' {
            ans += 10;
        } else {
            return false;
        }
    }
    ans % 11 == 0
}
