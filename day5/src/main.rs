use std::env;
use std::ffi::OsString;
//use std::collections::HashMap;
mod parser;

fn main() {
    let path = env::current_dir();
    let mut cwd:OsString = path.unwrap().into_os_string();
    cwd.push(r"\src\input.txt");
    let file_path: &String = &cwd.into_string().unwrap();

    let mut aoc_parser = parser::Parser::new(file_path);
    aoc_parser.advance();
    let mut seed_string: String = aoc_parser.line_buffer.clone();
    let pos: usize = seed_string.chars().position(|c: char| c == ':').unwrap();
    seed_string.replace_range(..pos+2, "");        
    
    //Turning seed string into vector of integers in one step, pulled this from 
    //https://stackoverflow.com/questions/26536871/how-can-i-convert-a-string-of-numbers-to-an-array-or-vector-of-integers-in-rust
    let seeds: Vec<i64> = seed_string
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();

    drop(aoc_parser); // No longer needed, no point in keeping around for rest of script

    let mut res: i64 = 999999999999; // big number for initial comparison
    for seed in seeds {
        let mut seed_parser = parser::Parser::new(file_path);
        let mut end_of_file: bool = false;
        let mut map_info: Vec<i64> = [0,0,0].to_vec();
        seed_parser.advance();
        let mut location: i64 = seed;
        while !end_of_file {
            seed_parser.advance();
            if seed_parser.line_buffer.is_empty() {
                location = map_info[0] + (location - map_info[1]);
                map_info  = [0,0,0].to_vec();
                seed_parser.advance(); //move past empty line
            } else {
                let current_map: Vec<i64> = seed_parser.line_buffer.split_whitespace()
                    .filter_map(|w| w.parse().ok())
                    .collect();
                
                if current_map[1] < location && current_map[1] + current_map[2] > location && location - current_map[1] <= location - map_info[1] {
                    map_info = current_map;
                }
    
            }
            if !seed_parser.has_more_lines() { 
                location = map_info[0] + (location - map_info[1]);
                end_of_file = true; 
            }
        }
        if location < res { res = location;}
    }
    println!("{}", res);
}
