/**
 * 125. Valid Palindrome
 */
struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let forward = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        let backward = forward.clone().rev();

        forward.eq(backward)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let s = "A man, a plan, a canal: Panama".to_string();
        let res = Solution::is_palindrome(s);
        assert!(res);
    }

    #[test]
    fn test_case_2() {
        let s = "race a car".to_string();
        let res = Solution::is_palindrome(s);
        assert!(!res);
    }

    #[test]
    fn test_case_3() {
        let s = " ".to_string();
        let res = Solution::is_palindrome(s);
        assert!(res);
    }
}
