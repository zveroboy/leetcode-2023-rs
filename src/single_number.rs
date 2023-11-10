/**
 * 136. Single Number
 *
 * Constraints:
 * * 1 <= nums.length <= 3 * 10 ** 4
 * * -3 * 10 ** 4 <= nums[i] <= 3 * 10 ** 4
 * * Each element in the array appears twice except for one element which appears only once.
 */
struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        for n in nums {
            total ^= n;
        }
        total
    }
}

#[test]
fn test_case_1() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
}

#[test]
fn test_case_2() {
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
}

#[test]
fn test_case_3() {
    assert_eq!(Solution::single_number(vec![1]), 1);
}
