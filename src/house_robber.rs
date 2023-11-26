/**
 * 198. House Robber
 * https://leetcode.com/problems/house-robber/
 */
struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut acc_alt = 0;
        let mut acc = 0;
        for (i, &num) in nums.iter().enumerate() {
            max = match i {
                0 | 1 => num.max(acc),
                _ => {
                    let candidate1 = acc;
                    let candidate2 = acc - nums[i - 1] + num;
                    let candidate3 = acc_alt + num;

                    max.max(candidate1).max(candidate2).max(candidate3)
                }
            };

            acc_alt = acc;
            acc = max;
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // #[ignore]
    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3, 1];
        let res = Solution::rob(nums);
        assert_eq!(res, 4);
    }

    // #[ignore]
    #[test]
    fn test_case_2() {
        let nums = vec![2, 7, 9, 3, 1];
        let res = Solution::rob(nums);
        assert_eq!(res, 12);
    }

    // #[ignore]
    #[test]
    fn test_case_3() {
        let nums = vec![10, 1, 2, 6, 3];
        let res = Solution::rob(nums);
        assert_eq!(res, 16);
    }

    // #[ignore]
    #[test]
    fn test_case_4() {
        let nums = vec![4, 1, 2, 7, 5, 3, 1];
        let res = Solution::rob(nums);
        assert_eq!(res, 14);
    }
}
