/*
In the card game poker, a hand consists of five cards and are ranked, from lowest to highest, in the following way:

    High Card: Highest value card.
    One Pair: Two cards of the same value.
    Two Pairs: Two different pairs.
    Three of a Kind: Three cards of the same value.
    Straight: All cards are consecutive values.
    Flush: All cards of the same suit.
    Full House: Three of a kind and a pair.
    Four of a Kind: Four cards of the same value.
    Straight Flush: All cards are consecutive values of same suit.
    Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.

The cards are valued in the order:
2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.

If two players have the same ranked hands then the rank made up of the highest value wins; for
example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie, for
example, both players have a pair of queens, then highest cards in each hand are compared
(see example 4 below); if the highest cards tie then the next highest cards are compared, and so on.

The file, poker.txt, contains one-thousand random hands dealt to two players. Each line of
the file contains ten cards (separated by a single space): the first five are Player 1's cards and
the last five are Player 2's cards. You can assume that all hands are valid (no invalid characters
 or repeated cards), each player's hand is in no specific order, and in each hand there is a clear
  winner.

How many hands does Player 1 win?
*/
use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::cmp::max;


fn straight(hand: &str) -> bool {
    let mut options: Vec<&str> = vec![];
    // Push on royal family options
    options.push("AJKQT");
    options.push("9JKQT");
    options.push("89JQT");
    options.push("789JT");
    options.push("6789T");
    options.push("2345A");
    let mut cards: Vec<char> = vec![];
    for card in hand.split(" ") {
        cards.push(card.chars().nth(0).unwrap())
    }
    cards.sort();
    let s: String = cards.iter().collect();
    // Check for the options defined above
    if options.contains(&&s[..]) {
        return true
    }
    let mut current_val = 0;
    for val in cards {
        let v = val as i32 - '0' as i32;
        if current_val == 0 {
            current_val = v;
            continue;
        }
        else if v != current_val + 1 {
            return false
        }
        current_val = v;
    }
    true
}


fn flush(hand: &str) -> bool {
    let mut suit = "*";
    for card in hand.split(" ") {
        if suit == "*"{
            suit = &card[1..2]
        }
        else if &card[1..2] != suit {
            return false
        }
    }
    return true
}

fn sets(hand: &str) -> Vec<bool> {
    let mut hm: HashMap<&str, i32> = HashMap::new();

    for card in hand.split(" ") {
        let face: &str = &card[0..1];
        if hm.contains_key(face) {
            let total = hm[face] + 1;
            hm.remove(face);
            hm.insert(face, total);
        }
        else {
            hm.insert(face, 1);
        }
    }
    let mut pair = false;
    let mut two_pair = false;
    let mut full_house = false;
    let mut three = false;
    let mut four = false;
    for (_k, v) in hm {
        if v == 2 {
            if !pair {
                pair = true;
            }
            else {
                two_pair = true;
                pair = false;
            }
            if three {
                full_house = true;
                three = false;
                pair = false;
            }
        }
        if v == 3 {
            if pair {
                full_house = true;
                pair = false;
            }
            else {
                three = true;
            }
        }
        if v == 4 {
            four = true;
        }
    }
    vec![pair, two_pair, full_house, three, four]
}

fn royal_flush(hand: &str) -> bool{
    let mut suit: char = '*';
    let r = vec!['T', 'J', 'Q', 'K', 'A'];
    // Check Suit
    for card in hand.split(" ") {
        if suit == '*' {
            suit = card.chars().nth(1).unwrap();
        }
        else if suit != card.chars().nth(1).unwrap() {
            return false
        };
        if !r.contains(&card.chars().nth(0).unwrap()) {
            return false
        }
    }
    return true
}

fn straight_flush(hand: &str) -> bool {
    return straight(hand) && flush(hand)
}

fn highest_set(hand: &str) -> &str {
    let mut cards: Vec<&str> = vec![];
    for card in hand.split(" ") {
        let f: &str = &card[0..1];
        if cards.contains(&f) {
            return f
        }
        cards.push(f)
    }
    &hand[13..14]
}

fn highest_card(hand: &str) -> &str {
    let mut cards: Vec<&str> = vec![];
    for card in hand.split(" ") {
        cards.push(&card[0..1])
    }
    if cards.contains(&"A") {
        return "A"
    } else if cards.contains(&"K") {
        return "K"
    } else if cards.contains(&"Q") {
        return "Q"
    } else if cards.contains(&"J") {
        return "J"
    } else if cards.contains(&"T") {
        return "T"
    }
    let mut current_card = "2";
    for card in cards {
        current_card = max(card, current_card)
    }
    current_card
}

fn _tests() {
    for _ in 0..1 {
        assert_eq!(highest_card("2D 3D 4D 5D 6D"), "6");
        assert_eq!(highest_card("2D 2D 2D 2D 2D"), "2");
        assert_eq!(highest_card("2D 3D 9D AD 2D"), "A");
        assert_eq!(highest_card("2D KD 9D AD 2D"), "A");
        assert_eq!(highest_card("2D QD 9D TD 2D"), "Q");
        assert_eq!(highest_card("2D 6D 9D TD 2D"), "T");
        assert_eq!(highest_set("5D KC 3H 5S AC"), "5");
        assert_eq!(highest_set("4S 7H QS 4C 2H"), "4");
        assert_eq!(highest_set("4S 7H QS 2C 2H"), "2");
        assert!(straight("AS 2S 3C 4C 5S"));
        assert!(straight("6S 2S 3C 4C 5S"));
        assert!(straight("TS JS KC QC AS"));
        assert!(straight("9S JS KC QC TS"));
        assert!(straight("TS JS 9C QC 8S"));
        assert!(straight("TS JS 9C 8C 7S"));
        assert!(straight("TS 7S 6C 8C 9S"));
        assert_eq!(straight("AS TC 4H 6S 8S"), false);
        assert!(royal_flush("TS JS QS KS AS"));
        assert_eq!(royal_flush("TC JS QS KS AS"), false);
        assert_eq!(royal_flush("9S JS QS KS AS"), false);
        assert!(straight_flush("3S 4S 5S 6S 7S"));
        assert!(straight_flush("AS 2S 3S 4S 5S"));
        assert!(straight_flush("9S TS JS QS KS"));
        // Pair
        assert_eq!(sets("2S 2C 3C 4C 5C"), vec![true, false, false, false, false]);
        // Two Pair
        assert_eq!(sets("2S 2C 3C 3S 5C"), vec![false, true, false, false, false]);
        // Three of a kind
        assert_eq!(sets("2S 2C 2D 3S 5C"), vec![false, false, false, true, false]);
    // Full House
        assert_eq!(sets("2S 2C 3C 3S 3D"), vec![false, false, true, false, false]);
        assert_eq!(sets("2S 2C 2C 3S 3D"), vec![false, false, true, false, false]);
    // Four of a kind
        assert_eq!(sets("2S 2C 2D 2H 5C"), vec![false, false, false, false, true]);
        assert!(flush("2S 3S 4S 5S TS"));
        assert_eq!(flush("2S 3S 4S 5S TC"), false);
    }
}

pub fn main() {
    let now = Instant::now();
    let data = fs::read_to_string("/Users/daniellee/Development/projecteuler/projecteuler/src/poker.txt").expect("Unable to read file");
    let hands: Vec<&str> = data.split("\n").collect();
    _tests();
    let mut wins = 0;
    for row in hands {
        let hand1 = &row[0..14];
        let hand2 = &row[15..29];
        if royal_flush(hand1) && !royal_flush(hand2){
            wins += 1;
            println!("{}", hand1);
        }
        else if straight_flush(hand1) && !royal_flush(hand2) && !straight_flush(hand2) {
            wins += 1;
            println!("{}", hand1);
        }
        let h1_sets = sets(hand1);
        let h2_sets = sets(hand2);
        // 4 of a kind
        if h1_sets[4] && !h2_sets[4] && !straight_flush(hand2) && !royal_flush(hand2) {
            wins += 1;
            println!("{}", hand1);
        }
        //Full house
        else if h1_sets[2] && !h2_sets[2] && !h2_sets[4] && !straight_flush(hand2) && !royal_flush(hand2) {
            wins += 1;
            println!("{}", hand1);
        }
        // Flush
        else if flush(hand1) && !flush(hand2) && !h2_sets[2] && !h2_sets[4] && !straight_flush(hand2) && !royal_flush(hand2) {
            wins += 1;
            println!("{}", hand1);
        }
        // Straight
        else if straight(hand1) && !straight(hand2) && !flush(hand2) && !h2_sets[2] && !h2_sets[4] && !straight_flush(hand2) && !royal_flush(hand2) {
            wins += 1;
            println!("{}", hand1);
        }
        // 3 of a kind
        else if h1_sets[3] {
            if !h2_sets[3]  && !straight(hand2) && !flush(hand2) && !h2_sets[2] && !h2_sets[4] && !straight_flush(hand2) && !royal_flush(hand2) {
                wins += 1;
//                println!("{}", hand1);
            } else if h2_sets[3] {
                println!("{}, {}", hand2, hand1);
            }
        }
        // 2 pairs
        else if h1_sets[1] {
            if !h2_sets[1] && !straight(hand2) && !flush(hand2) && !h2_sets[3] && !h2_sets[2] && !h2_sets[4] && !straight_flush(hand2) && !royal_flush(hand2) {
               wins += 1;
            } else if h2_sets[1] {
            println!("{}, {}", hand1, hand2)
            }
        }
        else if h1_sets[0] {
            if !h2_sets[0] && !straight(hand2) && !flush(hand2) && !h2_sets[1] && !h2_sets[3] && !h2_sets[2] && !h2_sets[4] && !straight_flush(hand2) && !royal_flush(hand2) {
               wins += 1;
            } else if h2_sets[0] {
                let hs1 = highest_set(hand1);
                let hs2 = highest_set(hand2);
                if hs1 == "A" {
                    wins += 1
                }
                else if hs1 == "K" && hs2 != "A" {
                    wins += 1
                }
                else if hs1 == "Q" && !vec!["A", "K"].contains(&hs2){
                    wins += 1
                }
                else if hs1 == "J" && !vec!["A", "K", "Q"].contains(&hs2){
                    wins +=1
                }
                else if hs1 == "T" && !vec!["A", "K", "Q", "T"].contains(&hs2) {
                    wins += 1
                }
                else if hs1 > hs2 {
                    wins += 1
                }
            }
        }
        // Highest card
        else if !h2_sets[0] && !straight(hand2) && !flush(hand2) && !h2_sets[1] && !h2_sets[3] && !h2_sets[2] && !h2_sets[4] && !straight_flush(hand2) && !royal_flush(hand2) {
            let hc1 = highest_card(hand1);
            let hc2 = highest_card(hand2);
            if hc1 == "A" {
                    wins += 1
                }
                else if hc1 == "K" && hc2 != "A" {
                    wins += 1
                }
                else if hc1 == "Q" && !vec!["A", "K"].contains(&hc2){
                    wins += 1
                }
                else if hc1 == "J" && !vec!["A", "K", "Q"].contains(&hc2){
                    wins +=1
                }
                else if hc1 == "T" && !vec!["A", "K", "Q", "T"].contains(&hc2) {
                    wins += 1
                }
                else if hc1 > hc2 {
                    wins += 1
                }
        }
    }
    println!("Total wins for Player 1: {}", wins);
    println!("Script took {} milliseconds to run", now.elapsed().as_millis());
}
