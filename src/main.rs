#![allow(dead_code)]

// use core::num;
use std::fmt;
// use rand::Rng;

enum Suit {
    Blue,
    Red,
    Orange,
    Black,
}
struct Tile {
    value: u8,
    suit: Suit,
}

impl Tile{
    fn new(value: u8, suit: Suit) -> Tile{
        Tile { value: value, suit: suit }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Black => write!(f, "Black"),
            Suit::Orange => write!(f, "Orange"),
            Suit::Red => write!(f, "Red"),
            Suit::Blue => write!(f, "Blue"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value: {}, Suit: {}", self.value, self.suit)
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


fn main() {
    println!("Drawing a random tile");
    const MAX_TILE: usize = 13;
    const NUM_SUITS: usize = 4;
    const MAX_N: u8 = 13;
    let mut test_vect: Vec<Tile> = Vec::with_capacity(MAX_TILE * NUM_SUITS * 2);
    for i in 0.. MAX_TILE * NUM_SUITS * 2{
        let n: u8 = i.try_into().unwrap();
        // print!("{}\n", (n / 13 ) % 4);
        match (n / 13 ) % 4 {
            1 => test_vect.push(Tile { value: n % MAX_N + 1, suit: Suit::Blue }), 
            2 => test_vect.push(Tile { value: n % MAX_N + 1, suit: Suit::Orange }), 
            3 => test_vect.push(Tile { value: n % MAX_N + 1, suit: Suit::Red }), 
            _ => test_vect.push(Tile { value: n % MAX_N + 1, suit: Suit::Black }), 
        }
        
    }
    println!("we have a container of size: {}. Item number 52 is: {}", test_vect.len(), test_vect.get(52).unwrap())
}
