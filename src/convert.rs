
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

pub fn block_quote(s: &str) -> Option<(String, String)> {
    if s == ">" {
        return Some(
            (
                "<blockquote>".to_string(),
                "</blockquote>".to_string()
            )
        )
            
        // (format("{}", "<block"))
        // return Some("<blockquote")
    }
    None
}

// pub fn ordered_list(s: &str) -> Option<(String,String)> {
    
//     if let Some(first_char) = s.get(0..1){
//         if first_char != "<" {
//             if let Ok(x) = first_char.parse::<usize>() {
                
//             }
//         }
//     }

//     match x {
//         1 => {
//             return "<ol>\n<li>".to_string(), "</li>".to_string()
//         },
//         _ => {
//             return "<li>".to_string(), "</li>".to_string()
//         }
//     }

//     None
// }


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
