use std::env;
use std::ffi::OsString;
use std::collections::HashMap;
mod parser;

fn main() {
    //Get input
    let path = env::current_dir();
    let mut cwd:OsString = path.unwrap().into_os_string();
    cwd.push(r"\src\input.txt");
    let file_path: &String = &cwd.into_string().unwrap();
    let mut aoc_parser = parser::Parser::new(file_path);
    aoc_parser.advance();
    
    //Format left or right instructions
    let mut left_or_right: Vec<usize> = Vec::new();
    let left_right_string: String = aoc_parser.line_buffer.clone();
    for c in left_right_string.chars() {
        match c  {
            'L' => left_or_right.push(0),
            'R' => left_or_right.push(1),
            _ => panic!("Should not get here!"),
        }
    }

    //Make map of Nodes and where they lead to
    let mut node_map: HashMap<String,[String;2]> = HashMap::new();
    aoc_parser.advance();
    let mut end_of_file: bool = false;
    while !end_of_file {
        aoc_parser.advance();
        
        let node_string: String = aoc_parser.line_buffer.clone();
        let node_info: Vec<&str> = node_string.split('=').collect();

        let node_name: &str = node_info[0].trim();
        let mut node_destinations: String = node_info[1].to_string();
        node_destinations.retain(|c| !c.is_whitespace() && c != '(' && c != ')');
        let node_destinations_formatted: Vec<&str> = node_destinations.split(',').collect();
        
        node_map.insert(node_name.to_string(), [node_destinations_formatted[0].to_string(),node_destinations_formatted[1].to_string()]);

        if !aoc_parser.has_more_lines() { end_of_file = true; }
    }

    //Loop through left or right instructions starting from AAA until ZZZ
    let mut res:i64 = 0;
    let mut left_right_index: usize = 0;
    let mut current_node: String = String::from("AAA");
    while current_node != String::from("ZZZ") {
        let direction: usize = left_or_right[left_right_index];
        current_node = node_map.get(&current_node).unwrap()[direction].clone();
        res += 1;
        left_right_index += 1;
        if left_right_index == left_or_right.len() { left_right_index = 0; }
    }
    println!("{}",res);
    
}