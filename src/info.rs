
fn get_title() -> String{
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    the_title
}

fn print_short_banner(){
   println!("{}", get_title());
}

fn print_long_banner(){
    print_short_banner();

    println!("Written by: {}\nHomepage: {}\nUsage: tinmy <somefile>.md\n",
             env!("CARGO_PKG_AUTHORS"),
             "home page",
             );
}

pub fn usage(){
    print_long_banner();
}
