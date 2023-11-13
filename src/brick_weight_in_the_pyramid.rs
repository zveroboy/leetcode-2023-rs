/**
 * https://leetcode.com/discuss/interview-question/3247361/Yandex-or-OnSite-or-Brick-weight-in-the-pyramid
 */
struct Solution {}
impl Solution {
    pub fn get_weight_by_coordinates(top_idx: usize, left_idx: usize) -> f32 {
        let mut store: Vec<Vec<f32>> = vec![];
        for t in 0..=top_idx {
            let nums_in_row = t + 1;
            for l in 0..nums_in_row {
                let lpw = if l == 0 {
                    0.0
                } else {
                    store[t - 1][l - 1] + 1.0
                };

                let rpw = if l == nums_in_row - 1 {
                    0.0
                } else {
                    store[t - 1][l] + 1.0
                };

                let row = match store.get_mut(t) {
                    Some(row) => row,
                    None => {
                        store.insert(t, vec![0.0; nums_in_row]);
                        &mut store[t]
                    }
                };

                row[l] = lpw / 2.0 + rpw / 2.0
            }
        }

        store[top_idx][left_idx] // fix
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::get_weight_by_coordinates(0, 0), 0_f32);
        assert_eq!(Solution::get_weight_by_coordinates(1, 0), 0.5);
        assert_eq!(Solution::get_weight_by_coordinates(1, 1), 0.5);
        assert_eq!(Solution::get_weight_by_coordinates(2, 0), 0.75);
        assert_eq!(Solution::get_weight_by_coordinates(2, 1), 1.5);
        assert_eq!(Solution::get_weight_by_coordinates(2, 2), 0.75);
        assert_eq!(Solution::get_weight_by_coordinates(3, 0), 0.875);
        assert_eq!(Solution::get_weight_by_coordinates(3, 1), 2.125);
        assert_eq!(Solution::get_weight_by_coordinates(3, 2), 2.125);
        assert_eq!(Solution::get_weight_by_coordinates(3, 3), 0.875);
        // assert_eq!(
        //     Solution::get_weight_by_coordinates(322, 156),
        //     306.48749781747574
        // );
    }
}
