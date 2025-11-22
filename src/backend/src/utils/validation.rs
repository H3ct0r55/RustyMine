use regex::Regex;

pub fn validate_name(input: &str) -> String {
    let re = Regex::new(r"[^A-Za-z0-9\-_]").unwrap();
    re.replace_all(input, "").to_string()
}

pub fn slugify(input: &str) -> String {
    let re = Regex::new(r"[^\w]+").unwrap();
    re.replace_all(input, "-").to_lowercase().to_string()
}
