/**
* 567. Permutation in String
*/
struct Solution {}
impl Solution {
    #[rustfmt::skip]
    const NEIGHBOUR_DELTAS: [(isize, isize); 4] = [
                 (-1, 0),
    ( 0, -1), /* ( y, x) */ ( 0, 1),
                 ( 1, 0),
    ];

    fn get_possible_neighbours((y, x): (usize, usize)) -> [(isize, isize); 4] {
        let mut neighbours = [(0, 0); Self::NEIGHBOUR_DELTAS.len()];

        for (i, (dy, dx)) in Self::NEIGHBOUR_DELTAS.iter().enumerate() {
            neighbours[i] = ((y as isize + dy), (x as isize + dx));
        }

        neighbours
    }

    fn mark_island(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, start: (usize, usize)) {
        let y_range = 0..grid.len() as isize;
        let x_range = 0..grid[0].len() as isize;
        let mut stack = vec![start];
        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            visited[current.0][current.1] = true;

            let possible_neighbours = &Self::get_possible_neighbours(current);
            let neighbours = possible_neighbours
                .iter()
                .filter(|(y, x)| y_range.contains(y) && x_range.contains(x))
                .map(|(y, x)| (*y as usize, *x as usize))
                .filter(|(y, x)| !visited[*y][*x] && grid[*y][*x] != '0');

            stack.extend(neighbours);
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == '1' && !visited[y][x] {
                    Self::mark_island(&grid, &mut visited, (y, x));
                    res += 1;
                } else {
                    visited[y][x] = true
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[ignore]
    #[test]
    fn test_case_1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        assert_eq!(Solution::num_islands(grid), 1);
    }

    // #[ignore]
    #[test]
    fn test_case_2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(Solution::num_islands(grid), 3);
    }
}
