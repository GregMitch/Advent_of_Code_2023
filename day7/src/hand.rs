use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct Hand {
    pub cards: [i32; 5],
    pub bid: i64,
    pub hand_type: i32,
}

impl Default for Hand {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool { 
        if self.hand_type == other.hand_type {
            if self.cards[0] == other.cards[0] {
                if self.cards[1] == other.cards[1] {
                    if self.cards[2] == other.cards[2] {
                        if self.cards[3] == other.cards[3] {
                            if self.cards[4] == other.cards[4] {
                                false
                            } else {
                                self.cards[4] < other.cards[4]
                            }
                        } else {
                            self.cards[3] < other.cards[3]
                        }
                    } else {
                        self.cards[2] < other.cards[2]
                    }
                } else {
                    self.cards[1] < other.cards[1]
                }
            } else {
                self.cards[0] < other.cards[0]
            }
        } else {
            self.hand_type < other.hand_type
        }
    }

    fn le(&self, other: &Self) -> bool { 
        self.hand_type <= other.hand_type
    }

    fn gt(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            if self.cards[0] == other.cards[0] {
                if self.cards[1] == other.cards[1] {
                    if self.cards[2] == other.cards[2] {
                        if self.cards[3] == other.cards[3] {
                            if self.cards[4] == other.cards[4] {
                                false
                            } else {
                                self.cards[4] > other.cards[4]
                            }
                        } else {
                            self.cards[3] > other.cards[3]
                        }
                    } else {
                        self.cards[2] > other.cards[2]
                    }
                } else {
                    self.cards[1] > other.cards[1]
                }
            } else {
                self.cards[0] > other.cards[0]
            }
        } else {
            self.hand_type > other.hand_type
        }
    }

    fn ge(&self, other: &Self) -> bool {
        self.hand_type >= other.hand_type
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else if self.gt(other) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: [0;5],
            bid: 0,
            hand_type: 0,
        }
    }

    pub fn whatami(&mut self) {
        let mut card_count: [i32; 5] = [0;5];
        for (i, card) in self.cards.into_iter().enumerate() {
            let mut matches: i32 = 0;
            for card_tmp in self.cards {
                if card_tmp == card { matches += 1; }
            }
            card_count[i] = matches;
        }
        card_count.sort();

        match card_count[4] {
            5 => self.hand_type = 7, //5 of a kind
            4 => self.hand_type = 6, //4 of a kind
            3 => {
                if card_count[1] == 2 { self.hand_type = 5; } //Full House
                else { self.hand_type = 4; } //3 of a kind
            },
            2 => {
                if card_count[1] == 2 { self.hand_type = 3; } //2 pair
                else { self.hand_type = 2; } //1 pair
            },
            1 => self.hand_type = 1, //High Card
            _ => panic!("This shouldn't be possible..."),
        }

    }
}