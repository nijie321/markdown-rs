
const UNORDERED_LIST: (&str, &str) = ("-", "+");

// const

pub fn converter(contents: &str) -> String{
    contents.split(' ')
        .map(|x| {
        if let Some(s) = bold(x) {
            return s
        }
        x.to_string()
    })
    .collect::<Vec<_>>()
    .join(" ")

}

fn ordered_list(s: &str) -> Option<String> {
    // if s.starts_with()

    None
}

pub fn bold(s: &str) -> Option<String> {
    
    // &line_contents
    //     .split(' ')
    //     .map(|x| {
    //         if let Some(s) = 
    //     })

    if (s.starts_with("**") && s.ends_with("**"))  ||
        (s.starts_with("__") && s.ends_with("__"))
    {
        let s_len = s.len();
        return Some(format!("{}{}{}", "<strong>", &s[2..s_len-2], "</strong>"))
    }
    None
}
