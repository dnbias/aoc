use std::fs::File;
use std::io::{BufRead, BufReader};

// count the number of times the measurement increases
// from the previous one
pub(crate) fn increases(file: &str) -> i32 {
    let input = File::open(file).unwrap();
    let reader = BufReader::new(input);
    let lines = reader.lines();
    let mut vec: Vec<i32> = Vec::new();

    for line in lines.map(|x| x.unwrap().parse::<i32>()) {
        vec.push(line.unwrap());
    }

    get_increases(vec)
}

pub(crate) fn increases_window(file: &str) -> i32 {
    let input = File::open(file).unwrap();
    let reader = BufReader::new(input);
    let lines = reader.lines();
    let mut vec: Vec<i32> = Vec::new();

    for line in lines.map(|x| x.unwrap().parse::<i32>()) {
        vec.push(line.unwrap());
    }

    get_increases_window(vec)
}

fn get_increases(vec: Vec<i32>) -> i32 {
    let mut counter = 0;
    for couple in vec.windows(2) {
        if couple.get(0) < couple.get(1) {
            counter += 1;
        }
    }
    counter
}

fn get_increases_window(vec: Vec<i32>) -> i32 {
    let mut counter = 0;
    let mut last_sum = std::i32::MAX;
    for triplet in vec.windows(3) {
        let this_sum:i32 = triplet.into_iter().sum();
        if this_sum > last_sum {
            counter += 1;
        }
        last_sum = this_sum;
    }
    counter
}

// find input in 'test', expected 7
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = increases("test");
        assert_eq!(result, 7);
    }
}
