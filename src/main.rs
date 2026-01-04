#![allow(dead_code)]

// use core::num;
// use rand::rng;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::collections::HashMap;
use std::{fmt, u8};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Suit {
    Blue,
    Red,
    Orange,
    Black,
    JokerR,
    JokerB,
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Tile {
    value: u8,
    suit: Suit,
}

impl Tile {
    fn new(value: u8, suit: Suit) -> Tile {
        Tile {
            value: value,
            suit: suit,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Black => write!(f, "A"),
            Suit::Orange => write!(f, "O"),
            Suit::Red => write!(f, "R"),
            Suit::Blue => write!(f, "U"),
            Suit::JokerR => write!(f, "JR"),
            Suit::JokerB => write!(f, "JB"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            value: 1,
            suit: Suit::Black,
        }
    }
}

fn generate_tile_stack(max_tile: usize, num_suits: usize, rng: &mut ThreadRng) -> Vec<Tile> {
    let max_n: u8 = max_tile.try_into().unwrap();
    let mut test_vect: Vec<Tile> = Vec::with_capacity(max_tile * num_suits * 2 + 2);
    for i in 0..max_tile * num_suits * 2 {
        let n: u8 = i.try_into().unwrap();
        // print!("{}\n", (n / 13 ) % 4);
        match (n / 13) % 4 {
            1 => test_vect.push(Tile {
                value: n % max_n + 1,
                suit: Suit::Blue,
            }),
            2 => test_vect.push(Tile {
                value: n % max_n + 1,
                suit: Suit::Orange,
            }),
            3 => test_vect.push(Tile {
                value: n % max_n + 1,
                suit: Suit::Red,
            }),
            _ => test_vect.push(Tile {
                value: n % max_n + 1,
                suit: Suit::Black,
            }),
        }
    }
    test_vect.push(Tile {
        value: u8::MAX,
        suit: Suit::JokerR,
    });
    test_vect.push(Tile {
        value: u8::MAX,
        suit: Suit::JokerB,
    });

    test_vect.shuffle(rng);
    return test_vect;
}

fn draw_tiles(current_stack: &mut Vec<Tile>, to_draw: usize) -> Vec<Tile> {
    // let to_draw: usize = 14;
    //check if 14 tiles exist in stack

    if current_stack.len() < to_draw {
        return Vec::new();
    }
    let mut hand_vec: Vec<Tile> = Vec::with_capacity(to_draw);
    for _ in 0..to_draw {
        // println!("{}",random_tile_idx);
        let curr_last = current_stack.last().unwrap();
        hand_vec.push(curr_last.clone());
        current_stack.pop();
    }
    return hand_vec;
}

fn sort_by_number(hand: &mut Vec<Tile>) -> Vec<Tile> {
    hand.sort_by(|a, b| a.value.cmp(&b.value));

    return hand.to_vec();
}

fn sort_by_suit(hand: &mut Vec<Tile>) -> Vec<Tile> {
    hand.sort_by(|a, b| a.suit.cmp(&b.suit));

    return hand.to_vec();
}

fn find_sets(mut hand: Vec<Tile>) -> HashMap<u8, i32> {
    let mut sorted_hand = sort_by_number(&mut hand);
    let mut val_map: HashMap<u8, i32> = HashMap::new();

    sorted_hand.dedup();
    for tile in sorted_hand {
        let curr_val = tile.value;

        val_map
            .entry(curr_val)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    return val_map;
}

fn find_runs(mut hand: Vec<Tile>) -> HashMap<u8, i32> {
    let mut sorted_hand = sort_by_number(&mut hand);
    sorted_hand = sort_by_suit(&mut sorted_hand);
    sorted_hand.dedup();
    let mut val_map: HashMap<u8, i32> = HashMap::new();

    let s = Suit::Black;

    let mut black_tiles: Vec<Tile> = sorted_hand
        .into_iter()
        .filter(|tile| tile.suit == s)
        .collect();
    let win_size = 3;
    let bt_win = black_tiles.windows(win_size);
    for win in bt_win {
        if win[win_size - 1].value - win[0].value == 2 {
            println!("Found a run of: ");
            println! {"{:?}", win};
        }
    }

    return val_map;
}
fn main() {
    let mut rng = rand::rng();
    let mut starting_stack = generate_tile_stack(13, 4, &mut rng);
    // let mut hand = draw_tiles(&mut starting_stack, 14);

    // let hand_sorted = sort_by_number(&mut hand);
    // println!("Hand drawn and sorted");
    // for tile in &hand_sorted {
    //     println!("{}", tile)
    // }
    // find_runs(hand_sorted);
    // print!("Counted sets");
    // println!("{:?}", sets);

    let test_hand = vec![
        Tile::new(1, Suit::Black),
        Tile::new(2, Suit::Black),
        Tile::new(3, Suit::Black),
        Tile::new(10, Suit::Black),
        Tile::new(10, Suit::Orange),
        Tile::new(11, Suit::Orange),
        Tile::new(13, Suit::Orange),
    ];
    let _ = find_runs(test_hand);
}

#[cfg(test)]
mod tests {
    // use rand::rng;

    use super::*;

    #[test]
    fn test_generation() {
        let mut rng = rand::rng();
        assert_eq!(generate_tile_stack(13, 4, &mut rng).len(), 106);
    }

    #[test]
    fn test_draw_tiles() {
        let mut rng = rand::rng();
        //1. generate stack
        let mut stack = generate_tile_stack(13, 4, &mut rng);
        //2. draw a hand
        let hand = draw_tiles(&mut stack, 14);
        //3. assert size of hand
        assert_eq!(hand.len(), 14);
        //4. assert size of stack now that player has drawn
        assert_eq!(stack.len(), 106 - 14);
    }

    #[test]
    fn test_sorting_num() {
        let mut test_hand = vec![
            Tile::new(8, Suit::Blue),
            Tile::new(10, Suit::Blue),
            Tile::new(5, Suit::Red),
            Tile::new(1, Suit::Orange),
        ];
        sort_by_number(&mut test_hand);
        assert_eq!(
            test_hand,
            vec![
                Tile::new(1, Suit::Orange),
                Tile::new(5, Suit::Red),
                Tile::new(8, Suit::Blue),
                Tile::new(10, Suit::Blue),
            ]
        );
    }

    #[test]
    fn test_sorting_suit() {
        let mut test_hand = vec![
            Tile::new(1, Suit::Orange),
            Tile::new(1, Suit::Blue),
            Tile::new(1, Suit::Red),
            Tile::new(1, Suit::Black),
        ];
        sort_by_suit(&mut test_hand);
        assert_eq!(
            test_hand,
            vec![
                Tile::new(1, Suit::Blue),
                Tile::new(1, Suit::Red),
                Tile::new(1, Suit::Orange),
                Tile::new(1, Suit::Black),
            ]
        );
    }

    #[test]
    fn sort_by_both() {
        // order by the sorter is Blue, Red, Orange, Black
        let mut test_hand = vec![
            Tile::new(1, Suit::Red),
            Tile::new(5, Suit::Blue),
            Tile::new(10, Suit::Red),
            Tile::new(5, Suit::Blue),
            Tile::new(12, Suit::Blue),
            Tile::new(1, Suit::Black),
            Tile::new(5, Suit::Orange),
            Tile::new(11, Suit::Orange),
        ];
        sort_by_number(&mut test_hand);
        sort_by_suit(&mut test_hand);

        assert_eq!(
            test_hand,
            vec![
                Tile::new(5, Suit::Blue),
                Tile::new(5, Suit::Blue),
                Tile::new(12, Suit::Blue),
                Tile::new(1, Suit::Red),
                Tile::new(10, Suit::Red),
                Tile::new(5, Suit::Orange),
                Tile::new(11, Suit::Orange),
                Tile::new(1, Suit::Black),
            ]
        )
    }

    #[test]
    fn count_sets() {
        let test_hand = vec![
            Tile::new(13, Suit::Red),
            Tile::new(13, Suit::Blue),
            Tile::new(13, Suit::Black),
            Tile::new(13, Suit::Black),
            Tile::new(1, Suit::Red),
            Tile::new(1, Suit::Black),
            Tile::new(2, Suit::Orange),
        ];
        let sets = find_sets(test_hand);
        let ans_map: HashMap<u8, i32> = HashMap::from([(13, 3), (1, 2), (2, 1)]);
        assert_eq!(sets, ans_map);
    }

    // fn count_sets_w_joker() {
    //     let test_hand = vec![
    //         Tile::new(13, Suit::Red),
    //         Tile::new(13, Suit::Blue),
    //         Tile::new(13, Suit::Black),
    //         Tile::new(13, Suit::Black),
    //         Tile::new(u8::MAX, Suit::JokerB),
    //         Tile::new(1, Suit::Red),
    //         Tile::new(1, Suit::Black),
    //         Tile::new(2, Suit::Orange),
    //     ];
    //     let sets = find_sets(test_hand);
    //     let ans_map: HashMap<u8, i32> = HashMap::from([(13, 4), (1, 2), (2, 1)]);
    //     assert_eq!(sets, ans_map);
    // }

    #[test]
    fn count_runs() {
        let test_hand = vec![
            Tile::new(1, Suit::Red),
            Tile::new(2, Suit::Red),
            Tile::new(3, Suit::Red),
            Tile::new(10, Suit::Red),
            Tile::new(10, Suit::Orange),
            Tile::new(11, Suit::Orange),
            Tile::new(13, Suit::Orange),
        ];
        let counted_runs = find_runs(test_hand);
        let ans_map: HashMap<u8, i32> = HashMap::from([(1, 3), (10, 2)]);
        assert_eq!(counted_runs, ans_map);
    }
}
