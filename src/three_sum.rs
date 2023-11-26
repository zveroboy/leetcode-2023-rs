/**
 * 15. 3Sum
 * https://leetcode.com/problems/3sum/
 */
struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();

        let range = 0..len;
        for i in range.clone() {
            // for first in range.clone() {
            println!("{}", nums[i]);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // #[ignore]
    #[test]
    fn test_case_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let res = Solution::three_sum(nums);
        assert_eq!(res, [[-1, -1, 2], [-1, 0, 1]]);
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let nums = vec![0, 1, 1];
        let res = Solution::three_sum(nums);
        assert_eq!(res, Vec::<Vec<_>>::new());
    }

    #[ignore]
    #[test]
    fn test_case_3() {
        let nums = vec![0, 0, 0];
        let res = Solution::three_sum(nums);
        assert_eq!(res, [[0, 0, 0]]);
    }
}
