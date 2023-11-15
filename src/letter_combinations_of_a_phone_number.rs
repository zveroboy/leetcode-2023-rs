/**
* 17. Letter Combinations of a Phone Number
*/
struct Solution {}
impl Solution {
    const BUTTONS: [Option<&'static str>; 10] = [
        None,
        None,
        Some("abc"),
        Some("def"),
        Some("ghi"),
        Some("jkl"),
        Some("mno"),
        Some("pqrs"),
        Some("tuv"),
        Some("wxyz"),
    ];

    // candidates
    // constraints
    // goal

    // fn is_valid<T, Iter>(mut state: Peekable<Iter>) -> bool where Iter: Iterator<Item = T> {
    //     state.peek().is_some()
    // }

    // fn get_candidates<T, Iter>(state: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    //     state
    // }

    // // ["abc", "def"]

    // fn search<T, Iter>(mut state: Peekable<Iter>, solutions) -> () where Iter: Iterator<Item = T> + Clone, T: Clone {
    //     if Self::is_valid(state.clone()) {
    //         solutions.append(state.clone());
    //         return;
    //     }

    //     for cand in Self::get_candidates(state.clone()) {
    //         state.add(cand);
    //         Self::search(state, solutions);
    //         state.remove(cand);
    //     }
    // }

    // fn solve() {
    //     let solutions = [];
    //     let state= HashMap::new();
    //     Self::search(state, solutions);
    //     return solutions;
    // }

    fn process(combs: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let first = match combs.first() {
            Some(v) => v,
            _ => return vec![],
        };

        if combs.len() == 1 {
            return first.iter().map(|ch| vec![*ch]).collect::<Vec<_>>();
        }

        let mut res = vec![];

        for ch in first {
            let perms = Self::process(combs[1..].to_vec());

            for mut perm in perms {
                perm.insert(0, *ch);
                res.push(perm)
            }
        }

        res
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let combs = digits
            .chars()
            .filter_map(|c| c.to_digit(10))
            .filter_map(|d| Solution::BUTTONS[d as usize])
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self::process(combs)
            .iter()
            .map(|comb| comb.iter().collect::<String>())
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_answer(ans: Vec<String>, exp: &[&str]) {
        assert_eq!(ans.len(), exp.len());
        exp.iter()
            .for_each(|s1| assert!(ans.iter().any(|s2| s2 == s1)));
    }

    #[test]
    fn test_case_1() {
        let digits = "23".to_string();

        let res = Solution::letter_combinations(digits);

        assert_answer(res, &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    }

    #[test]
    fn test_case_2() {
        let digits = "".to_string();

        let res = Solution::letter_combinations(digits);

        assert_answer(res, &[]);
    }

    #[test]
    fn test_case_3() {
        let digits = "2".to_string();

        let res = Solution::letter_combinations(digits);

        assert_answer(res, &["a", "b", "c"]);
    }
}
