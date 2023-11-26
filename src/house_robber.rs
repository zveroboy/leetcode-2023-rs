/**
 * 198. House Robber
 * https://leetcode.com/problems/house-robber/
 */
struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut max_alter = 0;
        let mut max_prev = 0;
        for (i, &num) in nums.iter().enumerate() {
            max = match i {
                0 | 1 => num.max(max),
                _ => {
                    let candidate2 = max - nums[i - 1] + num;
                    let candidate3 = max_alter + num;

                    max.max(candidate2).max(candidate3)
                }
            };

            max_alter = max_prev;
            max_prev = max;
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
