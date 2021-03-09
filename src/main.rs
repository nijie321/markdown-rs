#![feature(str_split_once)]
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::io::Write;
use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn convert_bold(s: &str) -> Option<String>{
    if (s.starts_with("**") && s.ends_with("**")) || 
        (s.starts_with("__") && s.ends_with("__"))
    {
        let s_len = s.len();
        // print_type_of(&s);
        return Some(format!("{}{}{}", "<strong>" , &s[2..s_len-2], "</strong>"))
    }
    None
}

fn parse_markdown_file(_filename: &str){
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", _filename);
    let input_filename = Path::new(_filename);
    
    // let output_filename = _filename.split('.').collect::<Vec<&str>>();
    let mut output_filename = String::from(_filename.split('.').nth(0).unwrap());
    output_filename.push_str(".html");

    let mut outfile = File::create(output_filename)
                        .expect("[ ERROR ] Could not create output file1");

    let file = File::open(&input_filename)
                .expect("[ ERROR ] Failed to open file!");

    let mut _ptag: bool = false;
    let mut _htag: bool = false;
    let mut _bquotetag: bool = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);
    
    let mut keywords: Vec<String> = (1..=6).map(|n| "#".repeat(n) ).collect();
        
    
    for line in reader.lines(){
       let line_contents = line.unwrap();
    
        let contents1: Option<(&str, &str)> = line_contents.split_once(' ');

        let mut blockquote = false;

        if let Some((symbol, contents)) = contents1{
            
            // println!("{} {}",symbol, contents);
            
            if keywords.contains(&symbol.to_string()) {
                let s_len = symbol.len();
                
                let mut temp_s = format!("<h{}>", s_len);
                
                // blockquote
                if &contents[0..1] == ">"{
                    blockquote = true;
                    temp_s.push_str("<blockquote>");
                }
                
                let mut tmp_contents = &contents[..];

                if blockquote{
                    tmp_contents = &tmp_contents[2..];
                }

                // println!("blockquote!!!!!!!{}",
                temp_s.push_str(
                &tmp_contents
                    .split(' ')
                    .map(|x| {
                        if let Some(s) = convert_bold(x) {
                            return s
                        }
                        x.to_string()
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
                );

                if blockquote{
                    temp_s.push_str("</blockquote>");
                    blockquote = false;
                }
                temp_s.push_str(&format!("</h{}>", s_len));
                
                tokens.push(temp_s);
                // println!("{}", temp_s);
            }else{
                let full_contents = format!("<p>{} {}</p>", symbol, contents.trim());
                
                if full_contents != "<p> </p>"{
                    tokens.push(full_contents);
                }
            }
        }
        
    }

   for line in &tokens {
       println!("{}", line);
       // outfile.write_all(line.as_bytes())
       //      .expect("[ ERROR ] Could not write to output file!");
   }
   println!("[ INFO ] Parsing complete!");
}

fn print_short_banner(){
    println!("{}", get_title());
}

fn get_title() -> String{
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    the_title
}


fn print_long_banner(){
    print_short_banner();
  println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n",
    env!("CARGO_PKG_AUTHORS"),
    "home page",
  );
}

fn usage(){
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len(){
        1 => usage(),
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ERROR] Invalid invocation (you done goofed!)");
        }
    }
}
