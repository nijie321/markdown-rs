

use std::any::type_name;

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}



// pub fn see_number(s: &str) 
