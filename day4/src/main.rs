use std::env;
use std::ffi::OsString;
use std::collections::HashMap;
mod parser;

fn main() {
    let path = env::current_dir();
    let mut cwd:OsString = path.unwrap().into_os_string();
    cwd.push(r"\src\input.txt");
    let file_path: &String = &cwd.into_string().unwrap();

    let mut aoc_parser = parser::Parser::new(file_path);
    let mut end_of_file: bool = false;
    let mut res: i32 = 0;
    let mut num_matches = 0;

    let mut aoc_parser_tmp = parser::Parser::new(file_path);
    let mut card_map: HashMap<i32, i32> = HashMap::new();
    while !end_of_file {
        aoc_parser_tmp.advance();
        card_map.insert(aoc_parser_tmp.line_count, 1);
        if !aoc_parser_tmp.has_more_lines() { end_of_file = true; }
    }
    
    end_of_file = false;
    while !end_of_file {
        aoc_parser.advance();
        let number_of_copies: i32 = *card_map.get(&aoc_parser.line_count).unwrap();

        let mut scratchcard_string: String = aoc_parser.line_buffer.clone();
        let pos: usize = scratchcard_string.chars().position(|c: char| c == ':').unwrap();
        scratchcard_string.replace_range(..pos+1, "");        
        let card_info: Vec<&str> = scratchcard_string.split('|').collect();
    
        let winning_numbers: Vec<&str> = card_info[0].split(' ').collect();
        let mut winning_numbers_matches: HashMap<&str, i32> = HashMap::new();
        for num in winning_numbers {
            if !num.is_empty() { winning_numbers_matches.insert(num, 0); } //Split on ' ' makes entires for empty chars
        }

        let actual_numbers: Vec<&str> = card_info[1].split(' ').collect();
        for num in actual_numbers {
            if !num.is_empty() {  //Split on ' ' makes entires for empty chars
                if winning_numbers_matches.contains_key(num) {
                    let value: &mut i32 = winning_numbers_matches.get_mut(num).unwrap();
                    if *value == 0 { num_matches += 1; }
                    *value += 1;
                }
            }
        }

        while num_matches > 0 {
            let value: &mut i32 = card_map.get_mut(&(aoc_parser.line_count + num_matches)).unwrap();
            *value += number_of_copies;
            num_matches -= 1;
        }

        if !aoc_parser.has_more_lines() { end_of_file = true; }
    }

    for pair in card_map {
        res += pair.1;
    }
    println!("{}", res);
}
