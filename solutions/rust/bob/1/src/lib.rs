pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }
    let letters = message.chars().filter(|c| c.is_alphabetic()).collect::<String>();
    let is_yelling = !letters.is_empty() && letters.chars().all(|c| c.is_uppercase());
    let is_question = matches!(message.chars().last(), Some('?'));
    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        _ => "Whatever." 
    }
}
