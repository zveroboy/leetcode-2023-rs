use std::collections::HashSet;
struct Solution {}
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<&i32> = HashSet::with_capacity(nums.len());
        for num in nums.iter() {
            if set.contains(num) {
                return true;
            }

            set.insert(num);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works_true() {
        let result = Solution::contains_duplicate(vec![2, 2, 3]);
        assert!(result);
    }

    #[test]
    fn it_works_false() {
        let result = Solution::contains_duplicate(vec![2, 1, 3]);
        assert!(!result);
    }
}
