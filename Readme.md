# Rummikub Opening Simulator

I love a good game of [Rummikub](https://rummikub.com/). What I don't love is watching everybody else play while I draw tiles and pass until somebody else wins.

But what are the chances that this happens?

## Some actual background

In rummikub, there are tiles numbered 1 through 13. Each tile can be one of 4 suits (Orange, Red, Blue, Black) and is repeated twice. Additionally, there are 2 wild-cards in the tile stack, for a total of 106 tiles.

At the start of the game, each player draws 14 tiles as a starting hand. From these tiles, the player must assemble either a run (consecutive tiles of increasing value, but the same suit), a set (3 or 4 of the same number, but varying suit) or a combination of both.

The sum of all tiles' value in an opening play must equal or surpass 30. If the turn player cannot make an opening play, they must draw a tile from the stack and pass turn, hoping to get an opening play on their next turn.

## Back to the point

Over the last year, I have played a few games with family and too many times I had to sit there watching the rest of the table play while drawing duplicate after duplicate. What are the chances of that happening? I am certain there is a very clever way to frame the problem with some statistics and a hypergeometric distribution or two that gets you a super elegant analytical solution, but I did not want to sit down and figure that out. And I just so happeend to have always wanted some project to finally start writing Rust. So here I am killing both birds by writing a simulator.

## Author

Vittorio Francescon, [email](mailto:vittorio.francescon@gmail.com).
