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
        print_type_of(&s);
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

       let contents: Vec<&str> = line_contents.split(' ').collect();
       let mut symbol_len = 0;

       let mut output_line = String::new();
        
       // let mut bolds = contents
       //              .iter()
       //              .map(|x| convert_bold(x))
       //              .filter(|x| !x.is_none())
       //              .peekable();

       // if bolds.peek().is_some() {
       //    println!("{:?}", bolds.next().unwrap());
       // }
       // println!("{}", 
       // contents
       //      .iter()
       //      .map(|x| convert_bold(x))
       //      .filter(|x| !x.is_none())
       //      // .filter(|x| x.contains("**"))
       //      // .map(|x|{
       //      //    print_type_of(&x);
       //      //    let l = &x.len();
       //      //     format!("{}{}{}","<strong>",&x[2..l-2],"</strong>")
       //      // })
       //      .collect::<Vec<_>>()
       //      .is_empty()
       //  );

       // println!("{:?}",contents.iter().filter(|x| x.contains("**") ).collect::<Vec<_>>());
    
       if keywords.contains(&contents[0].to_string()){
           symbol_len = contents[0].len();
           let full_contents = &contents[1..].iter()
               .map(|&x| {
                   if let Some(s) = convert_bold(x) {
                       return s
                       // return &s[..]
                   }else{
                       return x.to_string()
                   }
               })
               .collect::<Vec<_>>()
               .join(" ");

            // println!("{}", full_contents1);
           // let full_contents = &contents[1..].join(" ");

           if _ptag {
               _ptag = false;
               output_line.push_str("</p>\n");
           }
           
           if _htag{
               _htag = false;
               output_line.push_str(&format!("</h{}>\n", symbol_len));
               
           }
           _htag = true;
           output_line.push_str(&format!("<h{}>", symbol_len));
           output_line.push_str(full_contents);

       }else{
           if !_ptag {
               _ptag = true;
               output_line.push_str("<p>");
           }
           output_line.push_str(&contents.join(" "));
       }

       
       if _ptag {
           _ptag = false;
           output_line.push_str("</p>\n");
       }
       if _htag {
           _htag = false;
           output_line.push_str(&format!("</h{}>\n", symbol_len));
       }

       if output_line != "<p></p>\n" {
           tokens.push(output_line);
       }

    }


   // for line in &tokens {
   //     println!("{}", line);
   //     // outfile.write_all(line.as_bytes())
   //     //      .expect("[ ERROR ] Could not write to output file!");
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

fn usage(){
    // let the_version = get_version();
    // println!("tinymd, a markdown compiler writeen by Jie");
    // println!("The Version: {}", the_version);
    print_long_banner();
}

fn main() {
    // if let Some(s) = convert_bold("**hello**") {
    //     println!("{}", s);
    // };
    
    let args: Vec<String> = std::env::args().collect();
    match args.len(){
        1 => usage(),
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ERROR] Invalid invocation (you done goofed!)");
        }
    }
}
