/**
 * 238. Product of Array Except Self
 */
struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let mut ascending = vec![1; len];
        for i in 1..len {
            ascending[i] *= ascending[i - 1] * nums[i - 1];
        }

        let mut descending = vec![1; len];
        for i in (0..len - 1).rev() {
            descending[i] *= descending[i + 1] * nums[i + 1];
        }

        ascending
            .iter()
            .zip(descending.iter())
            .map(|(a, b)| a * b)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // fn assert_answer(ans: Vec<Vec<i32>>, exp: &[&[i32]]) {
    //     assert_eq!(ans.len(), exp.len());
    //     exp.iter()
    //         .for_each(|sub| assert!(ans.iter().any(|s2| s2.iter().eq(sub.iter()))));
    // }

    #[test]
    fn test_case_1() {
        let nums = [1, 2, 3, 4];

        let res = Solution::product_except_self(nums.to_vec());

        assert_eq!(res, [24, 12, 8, 6]);
    }

    // #[ignore]
    #[test]
    fn test_case_2() {
        let nums = [-1, 1, 0, -3, 3];

        let res = Solution::product_except_self(nums.to_vec());

        assert_eq!(res, [0, 0, 9, 0, 0]);
    }
}
