/**
* 3. Longest Substring Without Repeating Characters
*/
struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len = s.len();
        if len < 2 {
            return len as i32;
        }
        let mut res = 1;

        let chars = s.as_bytes();

        let mut l = 0;
        let mut r = 0;

        let mut mask = 0u128;
        while r < len {
            let rchi = chars[r] as u128;
            if (mask >> rchi) & 1 == 1 {
                res = res.max(r - l);
                l += 1;
                r = l;
                mask = 0;
                continue;
            }

            mask = mask | (1 << rchi);
            r += 1;
        }

        res = res.max(r - l);

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // #[ignore]
    #[test]
    fn test_case_1() {
        let s = "abcabcbb".to_string();

        let res = Solution::length_of_longest_substring(s);

        assert_eq!(res, 3);
    }

    // #[ignore]
    #[test]
    fn test_case_2() {
        let s = "bbbbb".to_string();

        let res = Solution::length_of_longest_substring(s);

        assert_eq!(res, 1);
    }

    // #[ignore]
    #[test]
    fn test_case_3() {
        let s = "pwwkew".to_string();

        let res = Solution::length_of_longest_substring(s);

        assert_eq!(res, 3);
    }

    // #[ignore]
    #[test]
    fn test_case_4() {
        let s = "au".to_string();

        let res = Solution::length_of_longest_substring(s);

        assert_eq!(res, 2);
    }

    // #[ignore]
    #[test]
    fn test_case_5() {
        let s = "dvdf".to_string();

        let res = Solution::length_of_longest_substring(s);

        assert_eq!(res, 3);
    }
}
