/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars().all(|c| c.is_digit(10) || c.is_whitespace()) &&
    // More than one digit
    code.chars().filter(|c| c.is_digit(10)).count() > 1 &&
    // Has valid checksum
    code
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(index, number)| if index % 2 == 0 { number } else { 2 * number } )
        .map(|number| if number > 9 { number - 9 } else { number } )
        .sum::<u32>() % 10 == 0
}
