use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::io::Write;


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

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);
    
    let mut keywords = vec!["#", "##", "###"];

    for line in reader.lines(){
       let line_contents = line.unwrap();
       let mut first_char: Vec<char> = line_contents.chars().take(1).collect();

       let contents: Vec<&str> = line_contens.split(' ').collect().unwrap();
       let first_content = line_contents.split(' ').nth(0).unwrap();
       
       let f_len = first_content.len();

       let mut output_line = String::new();

       if keywords.contains(&first_content) {
           // println!("{}",format!("<h{}>", first_content.len()));
           if _ptag {
                _ptag = false;
                output_line.push_str("</p>\n");
           }

           if _htag {
               _htag = false;
               output_line.push_str(&format!("</h{}>\n", f_len));
           }
           
           _htag = true;
           output_line.push_str(&format!("<h{}>",f_len ));
           output_line.push_str(&line_contents[2..]);
       }
           
    
       match first_char.pop(){
           Some('#') => {
               if _ptag {
                   _ptag = false;
                   output_line.push_str("</p>\n");
               }
               
               if _htag {
                   _htag = false;
                   output_line.push_str("</h1>\n");
                   
               }
               _htag = true;
               output_line.push_str("<h1>");
               output_line.push_str(&line_contents[2..]);
               // output_line.push_str(&line_contents);
           },
           _ => {
               if !_ptag {
                   _ptag = true;
                   output_line.push_str("<p>");
               }
               output_line.push_str(&line_contents);
           }
       };
       
       if _ptag {
           _ptag = false;
           output_line.push_str("</p>\n");
       }
       if _htag {
           _htag = false;
           output_line.push_str("</h1>\n");
       }

       if output_line != "<p></p>\n" {
           tokens.push(output_line);
       }

    }

   // for line in &tokens {
   //     outfile.write_all(line.as_bytes())
   //          .expect("[ ERROR ] Could not write to output file!");
   // }
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
    env!("CARGO_PKG_HOMEPAGE")
  );
}

fn get_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
    // env!("CARGO_PKG_VERSION")
}
fn usage(){
    // let the_version = get_version();
    // println!("tinymd, a markdown compiler writeen by Jie");
    // println!("The Version: {}", the_version);
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
