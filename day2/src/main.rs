use std::env;
use std::ffi::OsString;
mod parser;

/* //Part 1
const BLUE_CUBE: i32 = 14;
const GREEN_CUBE: i32 = 13;
const RED_CUBE: i32 = 12; 
*/
fn main() {
    let path = env::current_dir();
    let mut cwd:OsString = path.unwrap().into_os_string();
    cwd.push(r"\src\input.txt");
    let file_path: &String = &cwd.into_string().unwrap();

    let mut aoc_parser = parser::Parser::new(file_path);
    let mut end_of_file: bool = false;
    let mut res: i32 = 0;
    
    while !end_of_file {
        //let mut possible_game: bool = true; //Part 1
        aoc_parser.advance();
        println!("{}", aoc_parser.line_buffer);

        //Part 2
        let mut min_possible_red: i32 = 1; //Multiply by 0 would be bad... so initalize to 1 :)
        let mut min_possible_green: i32 = 1;
        let mut min_possible_blue: i32 = 1;
        
        let mut game_string: String = aoc_parser.line_buffer.clone();
        let pos: usize = game_string.chars().position(|c: char| c == ':').unwrap();
        game_string.replace_range(..pos+1, "");        
        let cube_counts: Vec<&str> = game_string.split(';').collect();

        for cube_draw in cube_counts {
            let draw_string: String = cube_draw.to_string();
            let single_colour_counts: Vec<&str> = draw_string.split(',').collect();
            
            for colour in single_colour_counts {
                if colour.contains('d') {
                    let red_pos: usize = colour.chars().position(|c: char| c == 'r').unwrap();
                    let mut red_str = colour.to_string();
                    red_str.replace_range(red_pos..,"");
                    let red_value: i32 = red_str.trim().parse().unwrap();
                    
                    /* //Part 1
                    if red_value > RED_CUBE {
                        possible_game = false;
                        break;
                    }
                    */
                    if red_value > min_possible_red { min_possible_red = red_value; }

                } else if colour.contains('g') {
                    let green_pos: usize = colour.chars().position(|c: char| c == 'g').unwrap();
                    let mut green_str = colour.to_string();
                    green_str.replace_range(green_pos..,"");
                    let green_value: i32 = green_str.trim().parse().unwrap();
                    
                    /* //Part 1
                    if green_value > GREEN_CUBE {
                        possible_game = false;
                        break;
                    }
                    */
                    if green_value > min_possible_green { min_possible_green = green_value; }
                } else if colour.contains('b') {
                    let blue_pos: usize = colour.chars().position(|c: char| c == 'b').unwrap();
                    let mut blue_str = colour.to_string();
                    blue_str.replace_range(blue_pos..,"");
                    let blue_value: i32 = blue_str.trim().parse().unwrap();
                    
                    /* //Part 1
                    if blue_value > BLUE_CUBE {
                        possible_game = false;
                        break;
                    }
                    */
                    if blue_value > min_possible_blue { min_possible_blue = blue_value; }
                } else {
                    panic!("Error!");
                }
            }
            //if !possible_game { break; } Part 1

        }
        //println!("{}", possible_game); //Part 1
        //if possible_game { res += aoc_parser.line_count; } //Part 1
        res += min_possible_red * min_possible_green * min_possible_blue;
        if !aoc_parser.has_more_lines() { end_of_file = true; }
    }
    println!("{}",res);

}
