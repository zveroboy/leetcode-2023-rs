/**
 * 190. Reverse Bits
 */
struct Solution {}
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut res = 0;
        for e in (0..32).rev() {
            res += ((x >> e) & 1) << (31 - e);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let res = Solution::reverse_bits(0b00000010100101000001111010011100u32);

        assert_eq!(res, 0b00111001011110000010100101000000);
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let res = Solution::reverse_bits(0b11111111111111111111111111111101u32);

        assert_eq!(res, 0b10111111111111111111111111111111);
    }
}
