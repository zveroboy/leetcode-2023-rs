use std::collections::{BTreeMap, HashMap};

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut res = strs
            .iter()
            .fold(HashMap::<String, Vec<String>>::new(), |mut acc, s| {
                let key = Self::hash(s);
                let entry = acc.entry(key);
                let vec = entry.or_default();
                vec.push(s.clone());

                acc
            })
            .values_mut()
            .map(|sv| {
                sv.sort_unstable();
                &*sv
            })
            .cloned()
            .collect::<Vec<_>>();

        res.sort_by(|a, b| a.len().cmp(&b.len()));
        res
    }

    fn hash(input: &str) -> String {
        let alphabet = 'a'..='z';
        // let letters: &[(_, _)] = alphabet.map(|ch| (ch, 0)).collect::<Vec<_>>().as_slice();
        let mut map: BTreeMap<_, _> = alphabet.map(|ch| (ch, 0)).collect();
        // map.
        for ch in input.chars() {
            map.entry(ch).and_modify(|counter| *counter += 1);
        }

        map.iter()
            .filter(|(_, &counter)| counter > 0)
            .map(|(ch, counter)| format!("{}:{}", ch, counter))
            .collect::<Vec<_>>()
            .join(";")
    }
}

#[test]
fn test_case_1() {
    assert_eq!(
        vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]],
        Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        )
    );

    // let a1 = Solution::group_anagrams(
    //             vec!["eat", "tea", "tan", "ate", "nat", "bat"]
    //                 .iter()
    //                 .map(|s| s.to_string())
    //                 .collect()
    //         );

    // println!("{:?}", a1);

    // assert_eq!(Solution::hash("tea"), "a:1;e:1;t:1");
}
