/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { None }
    else {
        let distance = s1
            .chars()
            .zip(s2.chars())
            .filter(|(s1_c, s2_c)| s1_c != s2_c)
            .count();
        Some(distance)
    }
}
