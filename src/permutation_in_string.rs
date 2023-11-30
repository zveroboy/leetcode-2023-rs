/**
* 567. Permutation in String
*/
struct Solution {}
impl Solution {
    fn align_index(b: usize) -> usize {
        b % b'a' as usize
    }

    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let alphabet_len = (b'a'..=b'z').count();
        let s1_len = s1.len();
        let s2_len = s2.len();

        if s1_len > s2_len {
            return false;
        }

        let init_lc = vec![0; alphabet_len];
        let s1_lc = s1.as_bytes().iter().fold(init_lc, |mut acc, b| {
            acc[Self::align_index(*b as usize)] += 1;
            acc
        });

        let mut s2_lc = s1_lc.clone();

        let mut j: i32 = -1;
        let s2_bytes = s2.as_bytes();

        while j < s1_len as i32 - 1 {
            j += 1;
            s2_lc[Self::align_index(s2_bytes[j as usize].into())] -= 1;
        }

        while j < s2_len as i32 - 1 {
            if s2_lc.iter().all(|&b| b == 0) {
                return true;
            }

            j += 1;
            s2_lc[Self::align_index(s2_bytes[j as usize].into())] -= 1;
            s2_lc[Self::align_index(s2_bytes[j as usize - s1_len].into())] += 1;
        }

        s2_lc.iter().all(|&b| b == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn test_case_1() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();

        assert!(Solution::check_inclusion(s1, s2));
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();

        assert!(!Solution::check_inclusion(s1, s2));
    }

    #[ignore]
    #[test]
    fn test_case_3() {
        let s1 = "adc".to_string();
        let s2 = "dcda".to_string();

        assert!(Solution::check_inclusion(s1, s2));
    }

    // #[ignore]
    #[test]
    fn test_case_4() {
        let s1 = "ad".to_string();
        let s2 = "a".to_string();

        assert!(!Solution::check_inclusion(s1, s2));
    }
}
