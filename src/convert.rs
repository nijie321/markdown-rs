

pub fn bold(s: &str) -> Option<String> {
    if (s.starts_with("**") && s.ends_with("**"))  ||
        (s.starts_with("__") && s.ends_with("__"))
    {
        let s_len = s.len();
        return Some(format!("{}{}{}", "<strong>", &s[2..s_len-2], "</strong>"))
    }
    None
}
