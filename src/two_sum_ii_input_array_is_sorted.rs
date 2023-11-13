/**
 * 167. Two Sum II - Input Array Is Sorted
 */
struct Solution {}
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();
        let mut l = 0usize;
        let mut r = len - 1;

        while l < r {
            let lnum = numbers[l];
            let rnum = numbers[r];
            // println!("[{lnum}, {rnum}] ({l}, {r})");

            let sum = lnum + rnum;

            if sum > target {
                r -= 1;
            } else if sum < target {
                l += 1;
            } else {
                break;
            }
        }

        vec![l as i32 + 1, r as i32 + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // #[ignore]
    #[test]
    fn test_case_1() {
        let s = vec![2, 7, 11, 15];
        let res = Solution::two_sum(s, 9);
        assert_eq!(res, [1, 2]);
    }

    // #[ignore]
    #[test]
    fn test_case_2() {
        let s = vec![2, 3, 4];
        let res = Solution::two_sum(s, 6);
        assert_eq!(res, [1, 3]);
    }

    // #[ignore]
    #[test]
    fn test_case_3() {
        let s = vec![-1, 0];
        let res = Solution::two_sum(s, -1);
        assert_eq!(res, [1, 2]);
    }

    // #[ignore]
    #[test]
    fn test_case_4() {
        let s = vec![5, 25, 75];
        let res = Solution::two_sum(s, 100);
        assert_eq!(res, [2, 3]);
    }

    // #[ignore]
    #[test]
    fn test_case_5() {
        let s = vec![3, 24, 50, 79, 88, 150, 345];
        let res = Solution::two_sum(s, 200);
        assert_eq!(res, [3, 6]);
    }

    // #[ignore]
    #[test]
    fn test_case_6() {
        let s = vec![
            12, 13, 23, 28, 43, 44, 59, 60, 61, 68, 70, 86, 88, 92, 124, 125, 136, 168, 173, 173,
            180, 199, 212, 221, 227, 230, 277, 282, 306, 314, 316, 321, 325, 328, 336, 337, 363,
            365, 368, 370, 370, 371, 375, 384, 387, 394, 400, 404, 414, 422, 422, 427, 430, 435,
            457, 493, 506, 527, 531, 538, 541, 546, 568, 583, 585, 587, 650, 652, 677, 691, 730,
            737, 740, 751, 755, 764, 778, 783, 785, 789, 794, 803, 809, 815, 847, 858, 863, 863,
            874, 887, 896, 916, 920, 926, 927, 930, 933, 957, 981, 997,
        ];
        let res = Solution::two_sum(s, 542);
        assert_eq!(res, [24, 32]);
    }
}
