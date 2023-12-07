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
    
    let times: Vec<i32> = time_string
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();

    aoc_parser.advance();
    let mut distance_string: String = aoc_parser.line_buffer.clone();
    let pos: usize = distance_string.chars().position(|c: char| c == ':').unwrap();
    distance_string.replace_range(..pos+2, "");        
        
    let records: Vec<i32> = distance_string
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();

    drop(aoc_parser);

    println!("{:?}", times);
    println!("{:?}", records);
    let mut possible_wins: Vec<i32> = Vec::new();
    for i in 0..times.len() {
        let mut count: i32 = 0;
        for j in 1..times[i] {
            let x = (times[i] - j) * j;
            if x > records[i] {count += 1;}
            //println!("x: {}, record: {}", x, records[i]);
        }
        possible_wins.push(count);
    }
    println!("{:?}", possible_wins);

    let mut res: i32 = 1;
    for value in possible_wins {
        res = res * value;
    }
    println!("{}", res);
}
