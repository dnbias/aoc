#![allow(dead_code)]

use std::io::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
extern crate nalgebra as na;
use na::Matrix5;

struct Board {
    matrix: Matrix5<u32>,
    index_extracted: Vec<(u8,u8)>,
    last_extracted: u32,
}

impl Board {
    fn new(m: Matrix5<u32>) -> Self {
        Board {
            matrix: m,
            index_extracted: vec!(),
            last_extracted: 0,
        }
    }

    fn extract(&mut self, n: u32) {
        self.matrix = self
            .matrix
            .map(|i|if i == n {0} else {i});

        // need the index
        for i in 0..5 {
            for j in 0..5 {
                let &mut el = self.matrix.get((i,j)).as_mut().unwrap();
                if *el == n {
                    self.index_extracted.push((i.try_into().unwrap(),j.try_into().unwrap()));
                    *el = 0;
                }
            }
        }
    }

    fn score(&self) -> u64 {
        let sum: u64 = self.matrix.sum().into();
        let last: u64 = self.last_extracted.into();
        sum * last
    }
}

pub(crate) fn play(input: &str) -> Result<u64, Error> {
    let (draw,mut boards) = parse(input).unwrap();

    for e in draw {
        for b in boards.iter_mut() {
            b.extract(e);
        }
    }


    Ok(1)
}

fn parse(input: &str) -> Result<(Vec<u32>,Vec<Board>), Error> {
    let input = File::open(input).unwrap();
    let reader = BufReader::new(input);

    // first line is draw
    let mut lines = reader.lines().map(|l| l.unwrap());
    let l = lines.next();
    let draw: Vec<u32> = l.unwrap()
                          .split(',')
                          .map(|n| n.parse::<u32>().unwrap()).collect();

    // following lines are boards
    // skip whitespace
    let mut boards: Vec<Board> = vec!();
    let mut line = lines.next();
    while line != None {
        let mut vec_board:Vec<u32> = vec!();
        for _ in 0..5 {
            let l = lines.next().unwrap();
            let row =
                l.trim()
                 .split(' ')
                 .map(|n|
                      n.trim().parse::<u32>());
            let mut vec: Vec<u32> = vec!();
            for e in row {
                match e {
                    Ok(n) => vec.push(n),
                    _ => (),
                }
            }

            vec_board.append(&mut vec);
        }
        // the matrix is filled column by column
        let board = Matrix5::from_iterator(vec_board.into_iter());
        // println!("{board}");
        let board: Board = Board::new(board);
        boards.push(board);
        line = lines.next(); // whitespace
    }

    Ok((draw,boards))
}
