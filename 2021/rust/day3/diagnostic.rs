// gamma * epsilon = powerconsumption
// gamma: bit gamma_i is the most common bit for each bit_i in the inputs
// epsilon: same as gamma but least common bits
// so:
// gamma XOR epsilon = 11111...
// epsilon = gamma XOR 11111..

pub(crate) fn calculate_diagnostic(input: &str) -> Result<(i32,i32), Error> {
    let input = File::open(input).unwrap();
    let reader = BufReader::new(input);
    let lines = reader.lines();
    let mut vec: Vec<u8> = Vec::new();
    let n_bits;
    for line in lines {
        let s = line.unwrap();
        n_bits = line.len();
        vec.push(s.parse::<u8>());
    }
    let gamma = get_gamma(vec, n_bits);
    let ones = 0;
    for i in n_bits {
        ones = ones ^ 1;
        ones.rotate_left(1);
    }

    let eps = gamma ^ ones;
    Ok((gamma,eps))
}

// see: count_ones and count_zeros
// BitVec
// 0b00000
fn get_gamma(vec: Vec<u8>, bits: u8) -> u8 {
    let size = vec.capacity();
    let one_bits_at_index[u32, bits];
    for b in vec {
        for i in 0..bits {
            one_bits_at_index[i] += b >> i & 1;
        }
    }

    let gamma = 0;
    for (i,b) in one_bits_at_index {
        if b > size/2 { // mostly ones in row i
            gamma = gamma ^ (1 << i);
        }
    }

    gamma
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lesgo() {
        let (gam,eps) = calculate_diagnostic("test");
        asserteq!(gamma*eps,198);
    }
}
