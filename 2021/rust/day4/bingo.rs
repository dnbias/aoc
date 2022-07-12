#![allow(dead_code)]

use std::io::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct Board {
    numbers: [[u32;5];5] // 5x5
}

pub(crate) fn play(input: &str) -> Result<u64, Error> {
    let (draw,boards) = parse(input);

    Ok(1)
}

fn parse(input: &str) -> (Vec<u32>,Vec<Board>) {
    let input = File::open(input).unwrap();
    let reader = BufReader::new(input);
    let mut draw: Vec<u32> = vec!();
    let mut boards: Vec<Board> = vec!();
    // first line is draw
    let mut lines = reader.lines().map(|l| l.unwrap());
    let l = lines.next();
    let draw: Vec<u32> = l.unwrap()
                          .split(',')
                          .map(|n| n.parse::<u32>().unwrap()).collect();

    // following lines are boards
    // skip white space
    let mut line = lines.next();
    while line != None {
        let mut board:Board;
        board.numbers = [[0;5];5];
        for i in 0..5 {
            let l = lines.next().unwrap();
            let l: Vec<u32> = l.split(' ').map(|n| n.parse::<u32>().unwrap()).collect();
            for j in 0..5 {
                board.numbers[i][j] = l[j];
            }
            boards.push(board);
        }
        line = lines.next();
    }

    (draw,boards)
}
