use std::env;
use std::ffi::OsString;
mod parser;

fn main() {
 
    let path = env::current_dir();
    let mut cwd:OsString = path.unwrap().into_os_string();
    cwd.push(r"\src\input.txt");
    let file_path: &String = &cwd.into_string().unwrap();

    let mut aoc_parser = parser::Parser::new(file_path);
    let mut end_of_file: bool = false;
    let mut res: i64 = 0;
    
    let mut two_d_char_vec: Vec<Vec<char>> = Vec::new();
    while !end_of_file {
        aoc_parser.advance();

        let line_buffer_vector: Vec<char> = aoc_parser.line_buffer.chars().collect::<Vec<_>>();
        two_d_char_vec.push(line_buffer_vector);

        if !aoc_parser.has_more_lines() { end_of_file = true; }
    }

    let total_rows: usize = two_d_char_vec.len();
    let total_cols: usize = two_d_char_vec[0].len();
    let mut current_row: usize = 0;
    let mut current_col: usize = 0;
    
    let mut gear_nums: Vec<String> = Vec::new();

    while current_row < total_rows {
        while current_col < total_cols {
            let current_char = two_d_char_vec[current_row][current_col];
            if current_char == '*' {
                let mut tl: bool = false;
                let mut tm: bool = false;
                let mut bl: bool = false;
                let mut bm: bool = false;

                let mut value_string: String = String::new();
                if current_col == 0 && current_row == 0 { //Top Left
                    if two_d_char_vec[current_row][current_col+1].is_ascii_digit() { 
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col].is_ascii_digit() {
                        bm = true;
                        let mut tmp = current_col;
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col+1].is_ascii_digit() && !bm {
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }


                } else if current_col > 0 && current_col < total_cols-1 && current_row == 0 { // Top
                    if two_d_char_vec[current_row][current_col+1].is_ascii_digit() { 
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row][current_col-1].is_ascii_digit() { 
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col-1].is_ascii_digit() {
                        bl = true;
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col].is_ascii_digit() {
                        bm = true;
                        if !bl {
                            let mut tmp = current_col;
                            while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                                if tmp == 0 {break;}
                                tmp -=1;
                            }
                            if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                            while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                                value_string.push(two_d_char_vec[current_row+1][tmp]);
                                tmp += 1;
                            }
                            gear_nums.push(value_string.clone());
                        }
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col+1].is_ascii_digit() && !bm {
                        let mut tmp = current_col+1;
                        while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }


                } else if current_col == total_cols-1 && current_row == 0 { // Top Right
                    if two_d_char_vec[current_row][current_col-1].is_ascii_digit() { 
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col-1].is_ascii_digit() {
                        bl = true;
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col].is_ascii_digit() && !bl {
                        let mut tmp = current_col;
                        while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }


                } else if current_col == 0 && current_row > 0 && current_row < total_rows-1 { //Middle left
                    if two_d_char_vec[current_row][current_col+1].is_ascii_digit() { 
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col].is_ascii_digit() {
                        bm = true;
                        let mut tmp = current_col;
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col+1].is_ascii_digit() && !bm {
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col].is_ascii_digit() {
                        tm = true;
                        let mut tmp = current_col;
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col+1].is_ascii_digit() && !tm {
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }


                } else if current_col > 0 && current_col < total_cols-1 && current_row > 0 && current_row < total_rows-1 { //Middle
                    if two_d_char_vec[current_row][current_col+1].is_ascii_digit() { 
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row][current_col-1].is_ascii_digit() { 
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col-1].is_ascii_digit() {
                        bl = true;
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col].is_ascii_digit() {
                        bm = true;
                        if !bl {
                            let mut tmp = current_col;
                            while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                                if tmp == 0 {break;}
                                tmp -=1;
                            }
                            if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                            while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                                value_string.push(two_d_char_vec[current_row+1][tmp]);
                                tmp += 1;
                            }
                            gear_nums.push(value_string.clone());
                        }
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col+1].is_ascii_digit() && !bm {
                        let mut tmp = current_col+1;
                        while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col-1].is_ascii_digit() {
                        tl = true;
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        } 
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col].is_ascii_digit() {
                        tm = true;
                        if !tl {
                            let mut tmp = current_col;
                            while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                                if tmp == 0 {break;}
                                tmp -=1;
                            }
                            if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                            while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                                value_string.push(two_d_char_vec[current_row-1][tmp]);
                                tmp += 1;
                            }
                            gear_nums.push(value_string.clone());
                        }
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col+1].is_ascii_digit() && !tm {
                        let mut tmp = current_col+1;
                        while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                        

                } else if current_col == total_cols-1 && current_row > 0 && current_row < total_rows-1 { // Middle right
                    if two_d_char_vec[current_row][current_col-1].is_ascii_digit() { 
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col-1].is_ascii_digit() {
                        bl = true;
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row+1][current_col].is_ascii_digit() && !bl {
                        let mut tmp = current_col;
                        while two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row+1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row+1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row+1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col-1].is_ascii_digit() {
                        tl = true;
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col].is_ascii_digit() && !tl {
                        let mut tmp = current_col;
                        while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }

                } else if current_col == 0 && current_row == total_rows-1 { //Bottom left 
                    if two_d_char_vec[current_row][current_col+1].is_ascii_digit() { 
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col].is_ascii_digit() {
                        tm = true;
                        let mut tmp = current_col;
                        while  tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col+1].is_ascii_digit() && !tm {
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }

                } else if current_col > 0 && current_col < total_cols-1 && current_row == total_rows-1 { //Bottom
                    if two_d_char_vec[current_row][current_col+1].is_ascii_digit() { 
                        let mut tmp = current_col+1;
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row][current_col-1].is_ascii_digit() { 
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col-1].is_ascii_digit() {
                        tl = true;
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col].is_ascii_digit() {
                        tm = true;
                        if !tl {
                            let mut tmp = current_col;
                            while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                                if tmp == 0 {break;}
                                tmp -=1;
                            }
                            if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                            while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                                value_string.push(two_d_char_vec[current_row-1][tmp]);
                                tmp += 1;
                            }
                            gear_nums.push(value_string.clone());
                        }
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col+1].is_ascii_digit() && !tm {
                        let mut tmp = current_col+1;
                        while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }


                } else { // Bottom right...
                    if two_d_char_vec[current_row][current_col-1].is_ascii_digit() { 
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col-1].is_ascii_digit() {
                        tl = true;
                        let mut tmp = current_col-1;
                        while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                    value_string.clear();
                    if two_d_char_vec[current_row-1][current_col].is_ascii_digit() && !tl {
                        let mut tmp = current_col;
                        while two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            if tmp == 0 {break;}
                            tmp -=1;
                        }
                        if !two_d_char_vec[current_row-1][tmp].is_ascii_digit() { tmp += 1; }
                        while tmp != total_cols && two_d_char_vec[current_row-1][tmp].is_ascii_digit() {
                            value_string.push(two_d_char_vec[current_row-1][tmp]);
                            tmp += 1;
                        }
                        gear_nums.push(value_string.clone());
                    }
                }

                if gear_nums.len() == 2 {
                    println!("{:?}", gear_nums);
                    let num1: i64 = gear_nums[0].parse().unwrap();
                    let num2: i64 = gear_nums[1].parse().unwrap();
                    res += num1 * num2;
                } 
            }
            gear_nums.clear();
            current_col += 1;
        }
        current_col = 0;
        current_row += 1;
    }

    println!("{}", res);

}

/* //Part 1
    let mut value_string: String = String::new();
    let mut value_string_start: usize = 0;
    let mut valid_num: bool = false;

    while current_row < total_rows {
        while current_col < total_cols {
            let current_char = two_d_char_vec[current_row][current_col];
            
            if current_char.is_ascii_digit() {
                value_string.push(current_char);
                if value_string.len() == 1 { value_string_start = current_col; }

                if current_col == total_cols-1 || !two_d_char_vec[current_row][current_col+1].is_ascii_digit() {
                    //Scan and see if you add number 
                    if value_string_start == 0 && current_row == 0 { //Top Left
                        for i in value_string_start..current_col+2 {
                            if !two_d_char_vec[current_row+1][i].is_ascii_digit() && two_d_char_vec[current_row+1][i] != '.' { valid_num = true; }
                        }
                        if !two_d_char_vec[current_row][current_col+1].is_ascii_digit() && two_d_char_vec[current_row][current_col+1] != '.' { valid_num = true; }


                    } else if value_string_start > 0 && current_col < total_cols-1 && current_row == 0 { // Top
                        for i in value_string_start-1..current_col+2 {
                            if !two_d_char_vec[current_row+1][i].is_ascii_digit() && two_d_char_vec[current_row+1][i] != '.' { valid_num = true; }
                        }
                        if !two_d_char_vec[current_row][value_string_start-1].is_ascii_digit() && two_d_char_vec[current_row][value_string_start-1] != '.' { valid_num = true; }
                        if !two_d_char_vec[current_row][current_col+1].is_ascii_digit() && two_d_char_vec[current_row][current_col+1] != '.' { valid_num = true; }


                    } else if current_col == total_cols-1 && current_row == 0 { // Top Right
                        for i in value_string_start-1..current_col+1 {
                            if !two_d_char_vec[current_row+1][i].is_ascii_digit() && two_d_char_vec[current_row+1][i] != '.' { valid_num = true; }
                        }
                        if !two_d_char_vec[current_row][value_string_start-1].is_ascii_digit() && two_d_char_vec[current_row][value_string_start-1] != '.' { valid_num = true; }


                    } else if value_string_start == 0 && current_row > 0 && current_row < total_rows-1 { //Middle left
                        for i in value_string_start..current_col+2 {
                            if !two_d_char_vec[current_row-1][i].is_ascii_digit() && two_d_char_vec[current_row-1][i] != '.' { valid_num = true; }
                            if !two_d_char_vec[current_row+1][i].is_ascii_digit() && two_d_char_vec[current_row+1][i] != '.' { valid_num = true; }
                        }
                        if !two_d_char_vec[current_row][current_col+1].is_ascii_digit() && two_d_char_vec[current_row][current_col+1] != '.' { valid_num = true; }


                    } else if value_string_start > 0 && current_col < total_cols-1 && current_row > 0 && current_row < total_rows-1 { //Middle
                        for i in value_string_start-1..current_col+2 {
                            if !two_d_char_vec[current_row-1][i].is_ascii_digit() && two_d_char_vec[current_row-1][i] != '.' { valid_num = true; }
                            if !two_d_char_vec[current_row+1][i].is_ascii_digit() && two_d_char_vec[current_row+1][i] != '.' { valid_num = true; }
                        }
                        if !two_d_char_vec[current_row][value_string_start-1].is_ascii_digit() && two_d_char_vec[current_row][value_string_start-1] != '.' { valid_num = true; }
                        if !two_d_char_vec[current_row][current_col+1].is_ascii_digit() && two_d_char_vec[current_row][current_col+1] != '.' { valid_num = true; }
                        

                    } else if current_col == total_cols-1 && current_row > 0 && current_row < total_rows-1 { // Middle right
                        for i in value_string_start-1..current_col+1 {
                            if !two_d_char_vec[current_row-1][i].is_ascii_digit() && two_d_char_vec[current_row-1][i] != '.' { valid_num = true; }
                            if !two_d_char_vec[current_row+1][i].is_ascii_digit() && two_d_char_vec[current_row+1][i] != '.' { valid_num = true; }
                        }
                        if !two_d_char_vec[current_row][value_string_start-1].is_ascii_digit() && two_d_char_vec[current_row][value_string_start-1] != '.' { valid_num = true; }


                    } else if value_string_start == 0 && current_row == total_rows-1 { //Bottom left 
                        for i in value_string_start..current_col+2 {
                            if !two_d_char_vec[current_row-1][i].is_ascii_digit() && two_d_char_vec[current_row-1][i] != '.' { valid_num = true; }
                        }
                        if !two_d_char_vec[current_row][current_col+1].is_ascii_digit() && two_d_char_vec[current_row][current_col+1] != '.' { valid_num = true; }


                    } else if value_string_start > 0 && current_col < total_cols-1 && current_row == total_rows-1 { //Bottom
                        for i in value_string_start-1..current_col+2 {
                            if !two_d_char_vec[current_row-1][i].is_ascii_digit() && two_d_char_vec[current_row-1][i] != '.' { valid_num = true; }
                        }
                        if !two_d_char_vec[current_row][value_string_start-1].is_ascii_digit() && two_d_char_vec[current_row][value_string_start-1] != '.' { valid_num = true; }
                        if !two_d_char_vec[current_row][current_col+1].is_ascii_digit() && two_d_char_vec[current_row][current_col+1] != '.' { valid_num = true; }


                    } else { // Bottom right...
                        for i in value_string_start-1..current_col+1 {
                            if !two_d_char_vec[current_row-1][i].is_ascii_digit() && two_d_char_vec[current_row-1][i] != '.' { valid_num = true; }
                        }
                        if !two_d_char_vec[current_row][value_string_start-1].is_ascii_digit() && two_d_char_vec[current_row][value_string_start-1] != '.' { valid_num = true; }

                    }

                    if valid_num {
                        let value_int: i32 = value_string.parse().unwrap();
                        res += value_int;
                    }
                }

            } else {
                value_string = String::new();
                valid_num = false;
            }

            current_col += 1;
        }
        value_string = String::new();
        valid_num = false;
        current_col = 0;
        current_row += 1;
    }
*/