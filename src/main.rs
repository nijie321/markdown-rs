#![feature(str_split_once)]
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::io::Write;
use std::any::type_name;

mod convert;
mod info;
mod utility;

#[derive(Debug)]
struct Parser {
    symbol_stack: Vec<String>,
    content_stack: Vec<String>,
}

fn parse_markdown_file(_filename: &str){

    println!("[ INFO ] Trying to parse {}...", _filename);

    let input_filename = Path::new(_filename);
    
    let mut output_filename = String::from(_filename.split('.').nth(0).unwrap());
    output_filename.push_str(".html");

    let mut outfile = File::create(output_filename)
                        .expect("[ ERROR ] Could not create output file1");

    let file = File::open(&input_filename)
                .expect("[ ERROR ] Failed to open file!");

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);
    
    let keywords: Vec<&str> = vec!["#","##","###"];
    
    let mut see_order_list = false;
    let mut next_number: usize = 1;
    // let order_list_ended = false;

    let mut encounter = false;

    for line in reader.lines().map(|l| l.unwrap()){

        if line != ""{
            let mut p = Parser{
                symbol_stack:  Vec::new(),
                content_stack: Vec::new(),
            };

            for word in line.split_whitespace() {
                p.symbol_stack.push(word.to_string());
            }
            
            while let Some(top) = p.symbol_stack.pop() {
                
                let top_len = top.len();
                if see_order_list && top.parse::<usize>().is_err(){
                    p.content_stack.insert(0, "</ol>".to_string());
                    see_order_list = false;
                }

                if let Some(converted_bold) = convert::bold(&top) {
                    p.content_stack.push(converted_bold);
                }
                else if let Some((start, end)) = convert::block_quote(&top){
                    p.content_stack.push(start);
                    p.content_stack.insert(0,end);
                }
                else if keywords.contains(&top.as_str()){
                    p.content_stack.insert(0, format!("</h{}>", top_len));
                   p.content_stack.push(format!("<h{}>", top_len ));
                } else{
                    p.content_stack.push(top.to_string())
                }

            }
            
            p.content_stack.reverse();
            let first_item = p.content_stack[0].as_str();
            
            if let Some(first_char) = first_item.get(0..1) {
                if first_char != "<" {
                    if let Ok(x) = first_char.parse::<usize>() {
                        encounter = true;
                        match x {
                            1 => {
                                p.content_stack[0] = "<ol>\n<li>".to_string();
                                p.content_stack.push("</li>".to_string());
                            },
                            _ => {
                                p.content_stack[0] = "<li>".to_string();
                                
                                p.content_stack.push("</li>".to_string());
                            }
                        }
                        println!("numeric!!! {}", x);
                    }else{

                        // encounter = false;
                        // if first_char.is_numeric()
                        // println!("first character = {}", first_char);
                        p.content_stack.push(format!("</p>"));
                        p.content_stack.insert(0, "<p>".to_string());

                    }

                }
            }

            if encounter && p.content_stack[0] != "<li>".to_string() && p.content_stack[0] != "<ol>\n<li>".to_string() {
                tokens.push("</ol>\n".to_string());
                encounter = false;
            }

            tokens.push(format!("{}\n",p.content_stack.join(" ")) );
            
            for line in &tokens{
                println!("{}", line);
            }
        
        }
    }
   println!("[ INFO ] Parsing complete!");
}


fn main() {

    let args: Vec<String> = std::env::args().collect();
    
    match args.len(){
        1 => info::usage(),
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ERROR] Invalid invocation (you done goofed!)");
        }
    }
}
