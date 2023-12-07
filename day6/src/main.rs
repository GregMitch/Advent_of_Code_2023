use std::env;
use std::ffi::OsString;
mod parser;

fn main() {
    let path = env::current_dir();
    let mut cwd:OsString = path.unwrap().into_os_string();
    cwd.push(r"\src\input.txt");
    let file_path: &String = &cwd.into_string().unwrap();

    let mut aoc_parser = parser::Parser::new(file_path);
    aoc_parser.advance();
    let mut time_string: String = aoc_parser.line_buffer.clone();
    let pos: usize = time_string.chars().position(|c: char| c == ':').unwrap();
    time_string.replace_range(..pos+2, "");
    time_string.retain(|c| !c.is_whitespace());        
    
    let time: f64 = time_string.parse().unwrap();

    aoc_parser.advance();
    let mut distance_string: String = aoc_parser.line_buffer.clone();
    let pos: usize = distance_string.chars().position(|c: char| c == ':').unwrap();
    distance_string.replace_range(..pos+2, "");
    distance_string.retain(|c| !c.is_whitespace()); 
        
    let record: f64 = distance_string.parse().unwrap();

    drop(aoc_parser);
    println!("{}", time);
    println!("{}", record);

    //Quadratic formula time: (-b +/- sqrt(b^2 - 4ac)) / 2a
    //a = -1
    //b = time
    //c = -record
   
    let num_to_root: f64 = (time*time) - 4.0*(-1.0)*(record*-1.0);
    let root1: f64 = ((time*(-1.0)) - num_to_root.sqrt())/ (2.0*(-1.0));
    let root2: f64 = ((time*(-1.0)) + num_to_root.sqrt())/ (2.0*(-1.0));

    let root1_int: i64 = root1.round() as i64;
    let root2_int: i64 = root2.round() as i64;

    println!("{}", root1_int - root2_int + 1);
}
