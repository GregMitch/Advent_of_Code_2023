use std::env;
use std::ffi::OsString;
mod parser;
mod hand;

fn main() {
    let path = env::current_dir();
    let mut cwd:OsString = path.unwrap().into_os_string();
    cwd.push(r"\src\input.txt");
    let file_path: &String = &cwd.into_string().unwrap();

    let mut aoc_parser = parser::Parser::new(file_path);

    let mut hands: Vec<hand::Hand> = Vec::new();
    let mut end_of_file: bool = false;
    while !end_of_file {
        aoc_parser.advance();
        
        let mut input_hand: hand::Hand = hand::Hand::new();
        let input_string: String = aoc_parser.line_buffer.clone();
        let hand_info: Vec<&str> = input_string.split_whitespace().collect();

        input_hand.bid = hand_info[1].parse().unwrap();
        let iter = hand_info[0].chars();
        for (card_index, c) in iter.enumerate() {
            match c {
                '2' => input_hand.cards[card_index] = 2,
                '3' => input_hand.cards[card_index] = 3,
                '4' => input_hand.cards[card_index] = 4,
                '5' => input_hand.cards[card_index] = 5,
                '6' => input_hand.cards[card_index] = 6,
                '7' => input_hand.cards[card_index] = 7,
                '8' => input_hand.cards[card_index] = 8,
                '9' => input_hand.cards[card_index] = 9,
                'T' => input_hand.cards[card_index] = 10,
                'J' => input_hand.cards[card_index] = 11,
                'Q' => input_hand.cards[card_index] = 12,
                'K' => input_hand.cards[card_index] = 13,
                'A' => input_hand.cards[card_index] = 14,
                _ => panic!("WTF IS THIS CARD?!?"),
            }
        }

        input_hand.whatami();
        hands.push(input_hand);

        if !aoc_parser.has_more_lines() { end_of_file = true; }
    }

    hands.sort();
    println!("{:?}", hands);
    let mut res: i64 = 0;
    let mut tmp: i64 = 1;
    for hand in hands {
        res += hand.bid * tmp;
        tmp += 1;
    }
    println!("{}", res);
}