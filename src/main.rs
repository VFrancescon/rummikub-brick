#![allow(dead_code)]

// use core::num;
// use rand::rng;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::collections::HashMap;
use std::{fmt, u8};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
enum Suit {
    Blue,
    Red,
    Orange,
    Black,
    JokerR,
    JokerB,
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
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

fn find_sets(sorted_hand: &Vec<Tile>) -> HashMap<u8, i32> {
    let mut hand_copy = sorted_hand.clone();
    // let mut sorted_hand = sort_by_number(&mut hand_copy);
    let mut val_map: HashMap<u8, i32> = HashMap::new();

    hand_copy.dedup();
    for tile in hand_copy {
        let curr_val = tile.value;

        val_map
            .entry(curr_val)
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }

    return val_map;
}

//  1   2   3   4   5
//  1  1+1 1+2 1+3 1+4
//  1 + 4 - 1 = 4

fn find_runs(sorted_hand: &Vec<Tile>) -> HashMap<Tile, i32> {
    let mut hand_copy = sorted_hand.clone();
    hand_copy = sort_by_suit(&mut hand_copy);
    hand_copy.dedup();
    let mut val_map: HashMap<Tile, i32> = HashMap::new();

    // let s = Suit::Red;

    let all_suits = [Suit::Black, Suit::Blue, Suit::Orange, Suit::Red];
    for s in all_suits {
        let tiles_by_suit: Vec<Tile> = hand_copy
            .clone()
            .into_iter()
            .filter(|tile| tile.suit == s)
            .collect();

        let win_size_max: usize = 7;
        let win_size = 3;

        for win_n in win_size..win_size_max {
            let bt_win = tiles_by_suit.windows(win_n);
            for win in bt_win {
                let run_span: i32 = (win[win_n - 1].value - win[0].value) as i32;
                if run_span == win_n as i32 - 1 {
                    val_map.entry(win[0].clone()).insert_entry(run_span);
                }
            }
        }
    }
    return val_map;
}
fn main() {
    let mut rng = rand::rng();
    let mut starting_stack  = generate_tile_stack(13, 4, &mut rng);
    let mut hand = draw_tiles(&mut starting_stack, 14);

    let hand_by_num = sort_by_number(&mut hand);

    let sets_map = find_sets(&hand_by_num);
    let runs_map = find_runs(&hand_by_num);


    println!("Generated random hand:");
    for tile in &hand{
        print!("{} ", tile);
    }
    print!("\n");

    println!("Sets result: {:?}", sets_map);
    println!("Runs result: {:?}", runs_map);

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
        let mut test_hand = vec![
            Tile::new(13, Suit::Red),
            Tile::new(13, Suit::Blue),
            Tile::new(13, Suit::Black),
            Tile::new(13, Suit::Black),
            Tile::new(1, Suit::Red),
            Tile::new(1, Suit::Black),
            Tile::new(2, Suit::Orange),
        ];
        test_hand = sort_by_number(&mut test_hand);
        let sets = find_sets(&test_hand);
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
        let mut test_hand = vec![
            Tile::new(1, Suit::Red),
            Tile::new(2, Suit::Red),
            Tile::new(3, Suit::Red),
            Tile::new(4, Suit::Red),
            Tile::new(5, Suit::Red),
            Tile::new(3, Suit::Red),
            Tile::new(10, Suit::Red),
            Tile::new(10, Suit::Orange),
            Tile::new(11, Suit::Orange),
            Tile::new(12, Suit::Orange),
            Tile::new(13, Suit::Orange),
        ];
        test_hand = sort_by_number(&mut test_hand);
        let counted_runs = find_runs(&test_hand);
        let ans_map: HashMap<Tile, i32> = HashMap::from([
            (Tile::new(1, Suit::Red), 4),
            (Tile::new(2, Suit::Red), 3),
            (Tile::new(3, Suit::Red), 2),
            (Tile::new(10, Suit::Orange), 3),
            (Tile::new(11, Suit::Orange), 2),
        ]);
        assert_eq!(counted_runs, ans_map);
    }
}

// run starting at 1 with span of 2 = 1 + 1+1 + 1+2
// run startin at 3 with span 5 = 3 + 3+1 + 3+2 + 3+3 + 3+4 + 3+5
// run starting at n with span of k = n + n+1 + n+2 + n+3 .. + n+k-1 + n+k
// which we can group as (k+1)n + Sum^k_0 i

// which when n = 1, k = 2 -> (3)*1 + 0+1+2 = 6 which is correct
// when n = 3, k = 5 -> (6)*3 + 0+1+2+3+4+5 = 33. which is correct

// the sum section is k(k+1)/2.

// so when we write out the whole expression.
//run_value = (k+1)n + (k+1)k/2 -> (k+1)2n/2 + (k+1)k/2 -> (k+1)(2n+k)/2
