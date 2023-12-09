use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

#[derive(Debug)]
pub struct Parser {
    buf_reader: BufReader<File>,
    pub line_buffer: String,
    pub line_count: i32,
}

impl Parser {
    pub fn new(file_name: &String) -> Self {
        let file =  File::open(file_name).expect("Error: new()->open file fail!");
        Parser {
            buf_reader: BufReader::new(file),
            line_buffer: String::new(),
            line_count: 0,
        }
    }

    pub fn has_more_lines(&mut self) -> bool {
        let start_pos = self.buf_reader.stream_position().expect("Error: has_more_lines()->seeking current position fail!");
        let num_bytes = self.buf_reader.read_line(&mut self.line_buffer).expect("Error: has_more_lines()->reading line fail!");
        
        
        let res = num_bytes != 0;

        self.buf_reader.seek(SeekFrom::Start(start_pos)).expect("Error: has_more_lines()->resetting current position fail!");
        self.line_buffer.clear();
        res
    }

    pub fn advance(&mut self) {
        //Clear buffer before adding anything
        self.line_buffer.clear();
        if self.has_more_lines() {
            self.buf_reader.read_line(&mut self.line_buffer).expect("Error: advance()->reading line fail!");
            self.line_buffer = self.line_buffer.trim().to_string();
            self.line_count += 1;
        }
    }

}