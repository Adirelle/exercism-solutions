pub fn translate(input: &str) -> String {
    input.split_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    if word.starts_with("xr") 
        || word.starts_with("yt")
        || word.starts_with(['a','e','i','o','u']) {
        return word.to_string() + "ay";
    }
    if let Some(mut pos) = word.find(['a','e','i','o','u']) {
        if pos > 0 {
            if word.chars().nth(pos) == Some('u') && word.chars().nth(pos-1) == Some('q') {
                pos += 1;
            }
            return word[pos..].to_string() + &word[..pos] + "ay";
        }
    } else if let Some(pos) = word.find('y') {
        return word[pos..].to_string() + &word[..pos] + "ay";        
    }
    word.to_string()
}
