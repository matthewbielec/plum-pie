use crate::board::Board;

mod board;
mod piece;

fn main() {
    let board = Board::start();
    println!("{}", board);
}
