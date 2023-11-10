/**
 * 191. Number of 1 Bits
 */
struct Solution;
impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut ones = 0;
        let shifts = n.ilog2();
        for i in 0..=shifts {
            ones += 1 & (n >> i) as i32
        }
        ones
    }
}

#[test]
fn test_case_1() {
    assert_eq!(
        Solution::hamming_weight(0b00000000000000000000000000001011),
        3
    );
}

#[test]
fn test_case_2() {
    assert_eq!(
        Solution::hamming_weight(0b000000000000000000000000010000000),
        1
    );
}

#[test]
fn test_case_3() {
    assert_eq!(
        Solution::hamming_weight(0b11111111111111111111111111111101),
        31
    );
}
