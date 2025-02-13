pub mod graphics;
pub mod math;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
pub fn gen_u32(num: u32) -> u32 {
    let mut random = num;
    random ^= random << 13;
    random ^= random >> 17;
    random ^= random << 5;
    random
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
