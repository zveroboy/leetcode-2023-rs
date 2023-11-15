/**
 * 11. Container With Most Water
 */
struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut sq = 0;

        let mut l = 0;
        let mut r = height.len() - 1;

        while l < r {
            let w = r - l;
            let hl = height[l];
            let hr = height[r];

            let h = hr.min(hl) as usize;

            sq = sq.max(w * h);

            println!("{w} {hl} {hr} {sq}");
            if hr < hl {
                r -= 1;
            } else {
                l += 1;
            }
        }

        sq as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];

        let res = Solution::max_area(height);

        assert_eq!(res, 49);
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let height = vec![1, 1];

        let res = Solution::max_area(height);

        assert_eq!(res, 1);
    }
}
