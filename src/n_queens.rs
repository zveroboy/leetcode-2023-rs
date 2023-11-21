use std::ops::Add;
/**
 * 51. N-Queens
 */

#[derive(Clone, Copy, Debug)]
struct Coords(i32, i32);
struct Index {
    y: i32,
    x: i32,
    relative_row: usize,
    relative_cell: usize,
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone, Copy)]
enum Dir {
    LeftToRight,
    TopToBottom,
    TopLeftToBottomRight,
    TopRightToBottomLeft,
}

impl Dir {
    pub const VALUES: &'static [Self] = &[
        Self::LeftToRight,
        Self::TopToBottom,
        Self::TopLeftToBottomRight,
        Self::TopRightToBottomLeft,
    ];

    fn step(&self) -> Coords {
        match self {
            Dir::LeftToRight => Coords(0, 1),
            Dir::TopToBottom => Coords(1, 0),
            Dir::TopLeftToBottomRight => Coords(1, 1),
            Dir::TopRightToBottomLeft => Coords(1, -1),
        }
    }
}

#[derive(Clone, Copy)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn shortest_side(&self) -> u32 {
        self.width.min(self.height)
    }
}

#[derive(Clone, Copy)]
struct IndexIterator {
    dir: Dir,
    size: Rect,
    current: u32,
}

impl IndexIterator {
    fn new(size: Rect, dir: Dir) -> Self {
        Self {
            dir,
            size,
            current: 0,
        }
    }

    fn total(&self) -> u32 {
        self.size.width * self.size.height
    }

    fn rows(&self) -> u32 {
        match self.dir {
            Dir::LeftToRight => self.size.height,
            Dir::TopToBottom => self.size.width,
            Dir::TopLeftToBottomRight | Dir::TopRightToBottomLeft => {
                self.size.width + self.size.height - 1
            }
        }
    }

    fn start_coord(&self) -> Coords {
        match self.dir {
            Dir::LeftToRight => Coords(0, 0),
            Dir::TopToBottom => Coords(0, 0),
            Dir::TopLeftToBottomRight => {
                let max_ver_index = (self.size.height - 1) as i32;
                Coords(max_ver_index, max_ver_index * -1)
            }
            Dir::TopRightToBottomLeft => {
                let max_hor_index = (self.size.width - 1) as i32;
                Coords(max_hor_index * -1, 0)
            }
        }
    }

    fn row_delta(&self) -> Coords {
        match self.dir {
            Dir::LeftToRight => Coords(0, 1),
            Dir::TopToBottom => Coords(1, 0),
            Dir::TopLeftToBottomRight => Coords(-1, 1),
            Dir::TopRightToBottomLeft => Coords(1, 1),
        }
    }

    fn cell_delta(&self) -> Coords {
        match self.dir {
            Dir::LeftToRight => Coords(0, 1),
            Dir::TopToBottom => Coords(1, 0),
            Dir::TopLeftToBottomRight => Coords(1, 1),
            Dir::TopRightToBottomLeft => Coords(1, -1),
        }
    }

    fn get_triangle_number(i: u32) -> u32 {
        (((8.0 * i as f32 + 1.0).sqrt() - 1.0) / 2.0) as u32
    }

    fn get_triangle_sum(n: u32) -> u32 {
        n * (n + 1) / 2
    }

    fn current_index(&self) -> Index {
        match self.dir {
            Dir::LeftToRight => Index {
                y: (self.current / self.size.width) as i32,
                x: (self.current % self.size.width) as i32,
                relative_row: (self.current / self.size.width) as usize,
                relative_cell: (self.current % self.size.width) as usize,
            },
            Dir::TopToBottom => Index {
                y: (self.current % self.size.height) as i32,
                x: (self.current / self.size.height) as i32,
                relative_row: (self.current / self.size.height) as usize,
                relative_cell: (self.current % self.size.height) as usize,
            },
            Dir::TopLeftToBottomRight | Dir::TopRightToBottomLeft => {
                let min_side = self.size.width.min(self.size.height);
                let triangle_cnt = Self::get_triangle_sum(min_side) - min_side;

                let (row, cell) = if self.current < triangle_cnt {
                    let row = Self::get_triangle_number(self.current);
                    let sum = Self::get_triangle_sum(row);

                    let cell = self.current - sum;
                    (row, cell)
                } else if self.current >= self.total() - triangle_cnt {
                    let row = self.rows()
                        - 1
                        - Self::get_triangle_number(
                            ((self.total() as i32 - 1) - self.current as i32).abs() as u32,
                        );

                    let sum = Self::get_triangle_sum(self.rows() - row);
                    let cell = (self.total() as i32 - (self.current + sum) as i32).abs() as u32;
                    (row, cell)
                } else {
                    let row = min_side - 1 + (self.current - triangle_cnt) / min_side;
                    let cell = (self.current - triangle_cnt) % min_side;
                    (row, cell)
                };

                Index {
                    y: (self.start_coord().0 + self.row_delta().0 * row as i32)
                        .clamp(0, self.size.height as i32 - 1)
                        + self.cell_delta().0 * cell as i32,
                    x: (self.start_coord().1 + self.row_delta().1 * row as i32)
                        .clamp(0, self.size.width as i32 - 1)
                        + self.cell_delta().1 * cell as i32,
                    relative_row: row as usize,
                    relative_cell: cell as usize,
                }
            }
        }
    }
}

impl Iterator for IndexIterator {
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.total() {
            return None;
        }

        let res = Some(self.current_index());
        self.current += 1;

        res
    }
}

struct Solution {}
impl Solution {
    fn is_valid(state: &[u16], n: u8, dir: Dir) -> bool {
        let mask = (1 << n) - 1;

        let iter = IndexIterator::new(
            Rect {
                width: n.into(),
                height: n.into(),
            },
            dir,
        );

        let index_sets = iter.fold(
            vec![0; iter.rows() as usize],
            |mut acc,
             Index {
                 y,
                 x,
                 relative_row,
                 relative_cell,
             }| {
                let orig_row_num = state[y as usize];
                let shift = n as usize - 1 - x as usize;
                let byte = ((mask & orig_row_num) >> shift) & 1;

                let set = &mut acc[relative_row];
                *set |= byte << relative_cell;

                acc
            },
        );

        for set in &index_sets {
            if set.count_ones() > 1 {
                return false;
            }
        }

        true
    }

    fn is_valid_all_directions(state: &[u16], n: u8) -> bool {
        Dir::VALUES.iter().all(|dir| Self::is_valid(state, n, *dir))
    }

    fn solve(state: Vec<u16>, solutions: &mut Vec<Vec<u16>>) -> Vec<u16> {
        let len = state.len();

        if !Self::is_valid_all_directions(&state, len as u8) {
            return vec![];
        }

        let current = match state.iter().position(|row| *row == 0) {
            Some(current) => current,
            _ => {
                solutions.push(state); // all row are filled state is valid
                return vec![];
            }
        };

        for perm in Self::get_candidates(state, current) {
            Self::solve(perm, solutions);
        }

        vec![]
    }

    fn get_candidates(state: Vec<u16>, current: usize) -> Vec<Vec<u16>> {
        (0..state.len())
            .map(|i| {
                let mut clone = state.clone();
                clone[current] = 1 << i;
                clone
            })
            .collect::<Vec<_>>()
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let state = vec![0; n];
        let mut solutions: Vec<Vec<u16>> = vec![];

        Solution::solve(state, &mut solutions);

        solutions
            .iter()
            .map(|solution| {
                let res = solution
                    .iter()
                    .map(|row| {
                        (0..n)
                            .map(|i| {
                                let res = if (row & (1 << i)).is_power_of_two() {
                                    "Q"
                                } else {
                                    "."
                                };
                                res.to_string()
                            })
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>();

                res
            })
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // #[ignore]
    #[test]
    fn test_case_1() {
        let inp = 4;
        let res = Solution::solve_n_queens(inp);
        assert_eq!(
            res,
            [
                [".Q..", "...Q", "Q...", "..Q."],
                ["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
    }

    #[ignore]
    #[test]
    fn test_case_2() {
        let inp = 5;
        let res = Solution::solve_n_queens(inp);
        assert_eq!(
            res,
            [
                ["Q....", "..Q..", "....Q", ".Q...", "...Q."],
                ["Q....", "...Q.", ".Q...", "....Q", "..Q.."],
                [".Q...", "...Q.", "Q....", "..Q..", "....Q"],
                [".Q...", "....Q", "..Q..", "Q....", "...Q."],
                ["..Q..", "Q....", "...Q.", ".Q...", "....Q"],
                ["..Q..", "....Q", ".Q...", "...Q.", "Q...."],
                ["...Q.", "Q....", "..Q..", "....Q", ".Q..."],
                ["...Q.", ".Q...", "....Q", "..Q..", "Q...."],
                ["....Q", ".Q...", "...Q.", "Q....", "..Q.."],
                ["....Q", "..Q..", "Q....", "...Q.", ".Q..."]
            ]
        );
    }

    // #[ignore]
    #[test]
    fn test_case_3() {
        let inp = 1;
        let res = Solution::solve_n_queens(inp);
        assert_eq!(res, [["Q"]]);
    }
}
