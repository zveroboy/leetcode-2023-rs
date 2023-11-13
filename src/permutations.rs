/**
 * 46. Permutations
 */
struct Solution {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];

        for n in nums {
            let mut s = res.len();

            while s > 0 {
                let mut temp = vec![];

                let i = res.len() - s;
                let sub = &res[i];

                let mut k = 0;
                while k <= sub.len() {
                    let mut clone = sub.clone();
                    clone.insert(k, n);
                    temp.push(clone);
                    k += 1;
                }

                res.splice(i..i + 1, temp);

                s -= 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_answer(ans: Vec<Vec<i32>>, exp: &[&[i32]]) {
        assert_eq!(ans.len(), exp.len());
        exp.iter()
            .for_each(|sub| assert!(ans.iter().any(|s2| s2.iter().eq(sub.iter()))));
    }

    #[test]
    fn test_case_1() {
        let nums = [1, 2, 3];

        let res = Solution::permute(nums.to_vec());

        assert_answer(
            res,
            &[
                &[1, 2, 3],
                &[1, 3, 2],
                &[2, 1, 3],
                &[2, 3, 1],
                &[3, 1, 2],
                &[3, 2, 1],
            ],
        );
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let nums = [0, 1];

        let res = Solution::permute(nums.to_vec());

        assert_answer(res, &[&[0, 1], &[1, 0]]);
    }
}
