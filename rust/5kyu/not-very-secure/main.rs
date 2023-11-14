use regex::Regex;

fn alphanumeric(password: &str) -> bool {
    let pattern = Regex::new("^[a-zA-Z0-9]+$").unwrap();
    pattern.is_match(password)
}