#![allow(dead_code)]
mod bingo;

fn main() {
    let score = bingo::play("input").unwrap();
    println!("{}", score);
}
