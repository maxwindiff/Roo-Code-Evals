pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .filter_map(|word| {
            let (first_alpha, _) = word.char_indices().find(|(_, c)| c.is_alphabetic())?;
            let first = word[first_alpha..first_alpha+1].to_ascii_uppercase().to_string();
            if word.chars().all(|c| c.is_uppercase()) {
                Some(first)
            } else {
                Some(first + &word[first_alpha + 1..].chars().filter(|c| c.is_uppercase()).collect::<String>())
            }
        })
        .collect()
}
