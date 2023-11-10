struct Solution {}
impl Solution {
    const RANGE: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    fn check_iter<'a>(iter: impl Iterator<Item = &'a char>) -> bool {
        let mut range_copy = Solution::RANGE.clone();

        for c in iter {
            let d = match c {
                d @ '1'..='9' => d.to_digit(10).unwrap(),
                '.' => continue,
                _ => panic!("Wrong value"),
            };

            let slot = &mut range_copy[d as usize - 1];
            *slot -= d as i32;

            if *slot < 0 {
                return false;
            }
        }
        return true;
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..=8 {
            let iter = board[i].iter();
            if !Self::check_iter(iter) {
                return false;
            }

            let iter = board.iter().flat_map(|row| row.get(i));
            if !Self::check_iter(iter) {
                return false;
            }

            let from = (i / 3) * 3;
            let iter = board[from..from + 3].iter().flat_map(|row| {
                let from = (i % 3) * 3;
                &row[from..from + 3]
            });

            if !Self::check_iter(iter) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn convert_to_vec(board: [[&str; 9]; 9]) -> Vec<Vec<char>> {
        board
            .iter()
            .map(|row| row.iter().flat_map(|s| s.chars()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_case_1() {
        let board = [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        let board_vec = convert_to_vec(board);

        let res = Solution::is_valid_sudoku(board_vec);

        assert!(res);
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let board = [
            ["8", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ];

        let board_vec = convert_to_vec(board);

        let res = Solution::is_valid_sudoku(board_vec);

        assert!(!res);
    }
}
