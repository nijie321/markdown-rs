#![feature(str_split_once)]
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::io::Write;
use std::any::type_name;

mod convert;
mod info;
mod utility;


fn parse_markdown_file(_filename: &str){
    // print_short_banner();
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
    
    let mut keywords: Vec<String> = (1..=6).map(|n| "#".repeat(n) ).collect();
        
    
    for line in reader.lines().map(|l| l.unwrap()){
    // for line in reader.lines()?{
       // let line_contents = line.unwrap();
    
        let contents1: Option<(&str, &str)> = line.split_once(' ');

        let mut blockquote = false;

        if let Some((symbol, contents)) = contents1{
            
            if keywords.contains(&symbol.to_string()) {
                let s_len = symbol.len();
                
                let mut temp_s = format!("<h{}>", s_len);
                
                // blockquote
                if &contents[0..1] == ">"{
                    blockquote = true;
                    temp_s.push_str("<blockquote>");
                }
                
                let mut tmp_contents = &contents[..];
                
                utility::print_type_of(&tmp_contents);

                if blockquote{
                    tmp_contents = &tmp_contents[2..];
                }

                // println!("blockquote!!!!!!!{}",
                temp_s.push_str(
                    &convert::converter(&tmp_contents)
                );

                if blockquote{
                    temp_s.push_str("</blockquote>");
                    blockquote = false;
                }
                temp_s.push_str(&format!("</h{}>\n", s_len));
                
                
                tokens.push(temp_s);
                // println!("{}", temp_s);
            }else{
                
                let combined_contents = format!("{} {}", symbol, contents.trim());

                
                let contents2 = convert::converter(&combined_contents);
                println!("{}",contents2);
                    
                tokens.push(format!("<p>{}</p>", &convert::converter(&combined_contents)) )
                
                // let full_contents = format!("<p>{} {}</p>\n", symbol, contents.trim());
                
                // if full_contents != "<p> </p>"{
                //     tokens.push(full_contents);
                // }
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

#[derive(Debug)]
struct Parser {
    symbol_stack: Vec<String>,
    // symbol_stack: Vec<&'static str>,
    // content_stack: Vec<&'static str>,
    content_stack: Vec<String>,
}


fn parse_test(_filename: &str){


    let input_filename = Path::new(_filename);
    
    let file = File::open(&input_filename)
                .expect("[ ERROR ] Failed to open file!");

    let reader = BufReader::new(file);
    
    let keywords: Vec<&str> = vec!["#","##","###"];

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
                if let Some(converted_bold) = convert::bold(&top) {
                    p.content_stack.push(converted_bold);
                }else{
                    if keywords.contains(&top.as_str()){
                        p.content_stack.insert(0, format!("</h{}>", top_len));
                       p.content_stack.push(format!("<h{}>", top_len ));
                    } else{
                        p.content_stack.push(top.to_string())
                    }
                }
            }
            
            p.content_stack.reverse();
            let first_item = p.content_stack[0].as_str();
            utility::print_type_of(&first_item);
            
            if let Some(first_char) = first_item.get(0..1) {
                if first_char != "<" {
                    p.content_stack.push(format!("</p>"));
                    p.content_stack.insert(0, "<p>".to_string());
                }
            }
                
            println!("{}",p.content_stack.join(" "));
        
        }

    }
}


fn main() {

    let args: Vec<String> = std::env::args().collect();

    match args.len(){
        1 => info::usage(),
        2 => parse_test(&args[1]),
        _ => {
            println!("[ERROR] Invalid invocation (you done something wrong!)");
        }
    }
    // parse_test()

    //
    //
    //
    // println!("{:?}",p.content_stack.iter().rev().next());

    // let ss: Option<(&str, &str)> = s.split_once(' ');
    
    // if let Some((symbol, contents)) = ss {
    //     println!("{}, {}", symbol, contents);
    // }
    
    // let args: Vec<String> = std::env::args().collect();
    // match args.len(){
    //     1 => info::usage(),
    //     2 => parse_markdown_file(&args[1]),
    //     _ => {
    //         println!("[ERROR] Invalid invocation (you done goofed!)");
    //     }
    // }
}
