#![allow(dead_code)]
// gamma * epsilon = powerconsumption
// gamma: bit gamma_i is the most common bit for each bit_i in the inputs
// epsilon: same as gamma but least common bits
// so:
// gamma XOR epsilon = 11111...
// epsilon = gamma XOR 11111..

use std::io::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub(crate) fn calculate_diagnostic(input: &str) -> Result<(u16,u16), Error> {
    let (vec,n_bits) = parse(input);

    let gamma = get_gamma(vec, n_bits as u16);
    let mut ones = 0;
    for _ in 0..n_bits {
        ones = ones << 1;
        ones = ones ^ 1;
    }

    let eps = gamma ^ ones;
    // print_bin(gamma);
    // print_bin(eps);
    Ok((gamma,eps))
}

fn parse(input: &str) -> (Vec<u16>,usize) {
    let input = File::open(input).unwrap();
    let reader = BufReader::new(input);
    let lines = reader.lines();
    let mut vec: Vec<u16> = Vec::new();
    let mut n_bits = 0;
    for line in lines {
        let s = line.unwrap();
        if n_bits == 0 {
            n_bits = s.len();
        }
        // println!("string {}, len {}",s,n_bits);
        let mut input: u16 = 0;
        for i in 0..n_bits {
            match s.chars().nth(i).unwrap() { // nth consumes!
                '1' => input = input ^ (1 << n_bits - i - 1),
                _ => (),
            }
        }
        vec.push(input);
    }

    (vec,n_bits)
}

// see: count_ones and count_zeros
// BitVec
// 0b00000
fn get_gamma(vec: Vec<u16>, bits: u16) -> u16 {
    let size = vec.len();
    let mut one_bits_at_index: Vec<u32> = vec!(0;bits.into());
    // println!("n bits: {}", bits);
    for b in vec {
        for i in 0..bits {
            one_bits_at_index[(bits-1-i) as usize] += (b >> i & 1) as u32;
        }
    }

    // println!("{}",size);
    let ref half_size = (size as u32)/2;
    let mut gamma: u16 = 0;
    for (i,b) in one_bits_at_index.iter().enumerate() {
        // println!("b:{}, hsize:{}",b,half_size);
        if b > half_size { // mostly ones in row i
            gamma = gamma ^ (1 << bits-1-(i as u16));
        }
    }
    // print_bin(gamma);

    gamma
}

pub(crate) fn ratings(input: &str) -> Result<(u64,u64),Error> {
    let (vec,n_bits) = parse(input);
    let oxr = ox_gen_rating(&vec,n_bits).unwrap();
    let cor = co_scr_rating(&vec,n_bits).unwrap();

    Ok((oxr,cor))
}

fn filter_out_at_digit(v: &mut Vec<u16>, value: bool,pos: u8) {
    v.retain(|x| digit(x,pos) != value);
}

fn digit(n: &u16, p: u8) -> bool {
    let d = n >> p & 1;
    match d {
        0 => false,
        _ => true,
    }
}

fn ox_gen_rating(v: &Vec<u16>, n_bits: usize) -> Result<u64,&'static str> {
    let mut filtered: Vec<u16> = v.clone();
    let mut ones = 0;
    let mut zeros = 0;

    for i in 0..n_bits {
        for (j,b) in filtered.iter().enumerate() {
            let r = b >> n_bits - 1 - i & 1;
            match r {
                0 => zeros += 1,
                _ => ones += 1,
            }
            if ones > (filtered.len()/2) as u32 {
                filter_out_at_digit(&mut filtered,false,(n_bits - 1 - i) as u8);
                break;
            } else if zeros > (filtered.len()/2) as u32 {
                filter_out_at_digit(&mut filtered, true, (n_bits - 1 - i) as u8);
                break;
            } else if j == filtered.len()-1 {
                assert_eq!(zeros,ones);
                filter_out_at_digit(&mut filtered, false, (n_bits - 1 - i) as u8);
                break;
            }
        }
        if filtered.len() == 1 { return Ok(filtered[0] as u64) }
        ones = 0;
        zeros = 0;
    }
    Err("rip")
}

fn co_scr_rating(v: &Vec<u16>, n_bits: usize) -> Result<u64,&'static str> {
    let mut filtered: Vec<u16> = v.clone();
    let mut ones = 0;
    let mut zeros = 0;
    for i in 0..n_bits {
        for (j,b) in filtered.iter().enumerate() {
            let r = b >> n_bits - 1 - i & 1;
            match r {
                0 => zeros += 1,
                _ => ones += 1,
            }
            if ones > (filtered.len()/2) as u32 {
                filter_out_at_digit(&mut filtered,true, (n_bits - 1 - i) as u8);
                break;
            } else if zeros > (filtered.len()/2) as u32 {
                filter_out_at_digit(&mut filtered, false, (n_bits - 1 - i) as u8);
                break;
            } else if j == filtered.len()-1 {
                assert_eq!(zeros,ones);
                filter_out_at_digit(&mut filtered, true, (n_bits - 1 - i) as u8);
                break;
            }
        }
        if filtered.len() == 1 { return Ok(filtered[0] as u64) }
        ones = 0;
        zeros = 0;
    }

    Err("rip")
}

fn print_vec(v: &Vec<u32>) {
    for e in v.iter() {
        print!("{},",e);
    }
    println!("");
}

fn print_bin(n: u16) {
    println!("{}", format!("{n:b}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic() {
        let (gam,eps) = calculate_diagnostic("test").unwrap();
        print_bin(gam);
        print_bin(eps);
        assert_eq!(gam*eps,198);
    }

    #[test]
    fn test_ratings() {
        let (ox,co) = ratings("test").unwrap();
        assert_eq!(ox,23);
        assert_eq!(co,10);
        assert_eq!(ox*co,230);
    }
}
