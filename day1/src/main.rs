use multipeek::multipeek;
use std::env;
use std::ffi::OsString;
mod parser;

fn main() {
    let path = env::current_dir();
    let mut cwd:OsString = path.unwrap().into_os_string();
    cwd.push(r"\src\input.txt");
    let file_path: &String = &cwd.into_string().unwrap();

    let mut aoc_parser = parser::Parser::new(file_path);
    let mut end_of_file = false;
    let mut res: i32 = 0;
    
    while !end_of_file {
        aoc_parser.advance();
        println!("{}", aoc_parser.line_buffer);
        let mut first_digit: char = 'a';
        let mut second_digit: char = 'a';
        let line_buffer_vector: Vec<char> = aoc_parser.line_buffer.chars().collect::<Vec<_>>();
        
        let mut iter = multipeek(line_buffer_vector.clone().into_iter());
        let mut iter_rev = multipeek(line_buffer_vector.into_iter().rev());
        while let Some(c) = iter.next() {
            match c {
                'o' => { //one
                    if iter.peek() == Some(&'n') && iter.peek_nth(1) == Some(&'e') {
                        first_digit = '1';
                        break;
                    }
                },
                't' => { //two or three
                    if iter.peek() == Some(&'w') && iter.peek_nth(1) == Some(&'o') {
                        first_digit = '2';
                        break;
                    } else if iter.peek() == Some(&'h') && iter.peek_nth(1) == Some(&'r') && iter.peek_nth(2) == Some(&'e') && iter.peek_nth(3) == Some(&'e') {
                        first_digit = '3';
                        break;
                    }
                },
                'f' => { //four or five
                    if iter.peek() == Some(&'o') && iter.peek_nth(1) == Some(&'u') && iter.peek_nth(2) == Some(&'r') {
                        first_digit = '4';
                        break;
                    } else if iter.peek() == Some(&'i') && iter.peek_nth(1) == Some(&'v') && iter.peek_nth(2) == Some(&'e') {
                        first_digit = '5';
                        break;
                    }
                },
                's' => { //six or seven
                    if iter.peek() == Some(&'i') && iter.peek_nth(1) == Some(&'x') {
                        first_digit = '6';
                        break;
                    } else if iter.peek() == Some(&'e') && iter.peek_nth(1) == Some(&'v') && iter.peek_nth(2) == Some(&'e') && iter.peek_nth(3) == Some(&'n') {
                        first_digit = '7';
                        break;
                    }
                },
                'e' => { //eight
                    if iter.peek() == Some(&'i') && iter.peek_nth(1) == Some(&'g') && iter.peek_nth(2) == Some(&'h') && iter.peek_nth(3) == Some(&'t') {
                        first_digit = '8';
                        break;
                    }
                },
                'n' => { //nine
                    if iter.peek() == Some(&'i') && iter.peek_nth(1) == Some(&'n') && iter.peek_nth(2) == Some(&'e') {
                        first_digit = '9';
                        break;
                    }
                },
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    first_digit = c;
                    break; 
                },
                _ => continue,
            }
        }

        while let Some(c) = iter_rev.next() {
            match c {
                'e' => { //one, three, five, nine
                    if iter_rev.peek() == Some(&'n') && iter_rev.peek_nth(1) == Some(&'o') {
                        second_digit = '1';
                        break;
                    } else if iter_rev.peek() == Some(&'e') && iter_rev.peek_nth(1) == Some(&'r') && iter_rev.peek_nth(2) == Some(&'h') && iter_rev.peek_nth(3) == Some(&'t') {
                        second_digit = '3';
                        break;
                    } else if iter_rev.peek() == Some(&'v') && iter_rev.peek_nth(1) == Some(&'i') && iter_rev.peek_nth(2) == Some(&'f') {
                        second_digit = '5';
                        break;
                    } else if iter_rev.peek() == Some(&'n') && iter_rev.peek_nth(1) == Some(&'i') && iter_rev.peek_nth(2) == Some(&'n') {
                        second_digit = '9';
                        break;
                    }
                },
                'o' => { //two
                    if iter_rev.peek() == Some(&'w') && iter_rev.peek_nth(1) == Some(&'t') {
                        second_digit = '2';
                        break;
                    }
                },
                'r' => { //four
                    if iter_rev.peek() == Some(&'u') && iter_rev.peek_nth(1) == Some(&'o') && iter_rev.peek_nth(2) == Some(&'f') {
                        second_digit = '4';
                        break;
                    }
                },
                'x' => { //six
                    if iter_rev.peek() == Some(&'i') && iter_rev.peek_nth(1) == Some(&'s') {
                        second_digit = '6';
                        break;
                    }
                },
                'n' => { //seven
                    if iter_rev.peek() == Some(&'e') && iter_rev.peek_nth(1) == Some(&'v') && iter_rev.peek_nth(2) == Some(&'e') && iter_rev.peek_nth(3) == Some(&'s') {
                        second_digit = '7';
                        break;
                    }
                },
                't' => { //eight
                    if iter_rev.peek() == Some(&'h') && iter_rev.peek_nth(1) == Some(&'g') && iter_rev.peek_nth(2) == Some(&'i') && iter_rev.peek_nth(3) == Some(&'e') {
                        second_digit = '8';
                        break;
                    }
                },
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    second_digit = c;
                    break; 
                },
                _ => continue,
            }
        }

        let mut value_string: String = String::new();
        value_string.push(first_digit);
        value_string.push(second_digit);
 
        let value_int: i32 = value_string.parse().unwrap();
        println!("String {} and int {}", value_string, value_int);
        res += value_int;
        if !aoc_parser.has_more_lines() { end_of_file = true; }
    }
    println!("{}",res);

}
