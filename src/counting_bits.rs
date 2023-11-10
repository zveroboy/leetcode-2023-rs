use std::vec;

/**
 * 338. Counting Bits
 */
struct Solution;
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut buf = vec![0; n + 1];
        for i in 1..=n {
            let power = (i as f32).log2() as u32;
            let closest = 1 << power;
            if i == closest {
                buf[i] = 1;
                continue;
            }

            buf[i] = buf[closest] + buf[i % closest];
        }
        buf
    }
}

#[test]
fn test_case_1() {
    assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
}

#[test]
fn test_case_2() {
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
