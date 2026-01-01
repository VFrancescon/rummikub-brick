#![allow(dead_code)]

// use core::num;
// use rand::rng;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Suit {
    Blue,
    Red,
    Orange,
    Black,
    Joker,
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
            Suit::Joker => write!(f, "J"),
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
        suit: Suit::Joker,
    });
    test_vect.push(Tile {
        value: u8::MAX,
        suit: Suit::Joker,
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

// todo: do not add if the suit has already been counted
fn find_sets(mut hand: Vec<Tile>) {
    let sorted_hand = sort_by_number(&mut hand);

    let mut val_map: HashMap<u8, i32> = HashMap::new();

    for tile in sorted_hand{
        let curr_val = tile.value;
        // if !val_map.contains_key(&curr_val){
            // val_map.insert(curr_val, 0);
        // } else{
        val_map.entry(curr_val).and_modify(|val| *val += 1).or_insert(1);
        // }

    }
    print!("{:?}", val_map);

    return;
}

fn main() {
    let mut rng = rand::rng();
    let mut starting_stack = generate_tile_stack(13, 4, &mut rng);
    let mut hand = draw_tiles(&mut starting_stack, 14);

    let hand_sorted  = sort_by_number(&mut hand);
    // println!("Sorted hand by suit");
    // for tile in hand_sorted {
    //     println!("{}", tile);
    // }
    for tile in &hand_sorted{
        println!("{}", tile)
    }
    println!{"Set summary"};
    find_sets(hand_sorted);


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
    fn sort_by_both(){
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

        assert_eq!(test_hand,
        vec![
            Tile::new(5, Suit::Blue),
            Tile::new(5, Suit::Blue),
            Tile::new(12, Suit::Blue),
            Tile::new(1, Suit::Red),
            Tile::new(10, Suit::Red),
            Tile::new(5, Suit::Orange),
            Tile::new(11, Suit::Orange),
            Tile::new(1, Suit::Black),
        ])
    }
}
