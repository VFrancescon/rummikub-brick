#![allow(dead_code)]

// use core::num;
use rand::rng;
use rand::{Rng, random, rngs::ThreadRng};
use std::fmt;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Suit {
    Blue,
    Red,
    Orange,
    Black,
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

fn generate_tile_stack(max_tile: usize, num_suits: usize) -> Vec<Tile> {
    let max_n: u8 = max_tile.try_into().unwrap();
    let mut test_vect: Vec<Tile> = Vec::with_capacity(max_tile * num_suits * 2);
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
    return test_vect;
}

fn draw_tiles(current_stack: &mut Vec<Tile>, mut rng: ThreadRng,  to_draw: usize) -> Vec<Tile> {
    // let to_draw: usize = 14;
    //check if 14 tiles exist in stack

    if current_stack.len() < to_draw {
        return Vec::new();
    }
    let mut hand_vec: Vec<Tile> = Vec::with_capacity(to_draw);
    for _ in 0..to_draw {
        let random_tile_idx = rng.random_range(0..current_stack.len());
        // println!("{}",random_tile_idx);
        hand_vec.push(current_stack[random_tile_idx].clone());
        current_stack.remove(random_tile_idx);
    }
    return hand_vec;
}

fn sort_by_number(hand: &mut Vec<Tile>) -> Vec<Tile>{
    hand.sort_by(|a, b| a.value.cmp(&b.value));
    
    return hand.to_vec();
}

fn main() {
    let rng = rand::rng();
    let mut starting_stack = generate_tile_stack(13, 4);
    let hand = draw_tiles(&mut starting_stack, rng, 14);
    for tile in hand {
        println!("{}", tile);
    }
}

#[cfg(test)]
mod tests {
    // use rand::rng;

    use super::*;

    #[test]
    fn test_generation() {
        assert_eq!(generate_tile_stack(13, 4).len(), 104);
    }

    #[test]
    fn test_draw_tiles() {
        let rng = rand::rng();
        //1. generate stack
        let mut stack = generate_tile_stack(13, 4);
        //2. draw a hand
        let hand = draw_tiles(&mut stack, rng, 14);
        //3. assert size of hand
        assert_eq!(hand.len(), 14);
        //4. assert size of stack now that player has drawn
        assert_eq!(stack.len(), 104 - 14);
    }

    #[test]
    fn test_sorting_num(){
        let mut test_hand = vec![
            Tile::new(8, Suit::Blue),
            Tile::new(10, Suit::Blue),
            Tile::new(5, Suit::Red),
            Tile::new(1, Suit::Orange),

        ];
        sort_by_number(&mut test_hand);
        assert_eq!(test_hand, vec![
            Tile::new(1, Suit::Orange),
            Tile::new(5, Suit::Red),
            Tile::new(8, Suit::Blue),
            Tile::new(10, Suit::Blue),
        ]);
    }
}
