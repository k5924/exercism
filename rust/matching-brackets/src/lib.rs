pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = Vec::new();
    for character in string.chars() {
        match character {
            '(' | '{' | '[' => brackets.push(character),
            ')' => if brackets.pop() != Some('(') {return false},
            ']' => if brackets.pop() != Some('[') {return false},
            '}' => if brackets.pop() != Some('{') {return false},
            _ => ()
        }
    }
    brackets.is_empty()
}
