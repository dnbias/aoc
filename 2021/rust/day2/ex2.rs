use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction{
    Forward,
    Up, // here it's aim
    Down, // aim too now
}

impl Direction {
    fn new(s: &str) -> Result<Direction,Error> {
        use Direction::*;
        match s {
            "forward" => Ok(Forward),
            "up" => Ok(Up),
            "down" => Ok(Down),
            _ => Err(Error),
        }
    }

}

pub(crate) fn get_final_position(course: &str)  -> Result<(i32, i32), Error> {
    let input = File::open(course).unwrap();
    let reader = BufReader::new(input);
    let lines = reader.lines();
    let mut vec: Vec<(Direction,i32)> = Vec::new();

    for line in lines {
        let s = line.unwrap();
        let mut split = s.split(' ');
        let dir = Direction::new(split.next().unwrap())?;
        let amount = split.next().unwrap().parse::<i32>();

        vec.push((dir,amount.unwrap()));
    }

    Ok(calculate_movement_aim(vec))
}

fn calculate_movement_aim(vec: Vec<(Direction,i32)>) -> (i32,i32) {
    let mut aim = 0;
    let mut hor = 0; // horizontal position
    let mut depth = 0;

    use Direction::*;
    for (dir, n) in vec {
        match dir {
            Forward => {
                hor += n;
                depth += aim * n;
            },
            Up => aim -= n,
            Down => aim += n,
        }
    }

    (hor,depth)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let (pos,depth) = get_final_position("test").unwrap();
        assert_eq!(pos*depth,900);
    }
}
