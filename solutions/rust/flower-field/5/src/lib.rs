#[derive(Clone, Copy, Debug)]
enum Cell {
    Count(u32),
    Flower,
}

struct Grid {
    counted_grid: Vec<Vec<Cell>>,
}

impl Grid {
    fn index_safe(
        rows_len: usize,
        cols_len: usize,
        rows_i: isize,
        cols_i: isize,
    ) -> bool {
        let mut is_safe = true;
        is_safe &= rows_i >= 0;
        is_safe &= (rows_i) < (rows_len as isize);
        is_safe &= cols_i >= 0;
        is_safe &= (cols_i) < (cols_len as isize);
        is_safe
    }

    fn from_garden(garden: &[&str]) -> Self {
        if garden.is_empty() || garden[0].is_empty() {
            // Should return an error, but I'll implement what the test expect
            return Self {
                // garden: garden.clone(),
                counted_grid: vec![vec![]],
            };
        }

        let cols = garden[0].len();
        let rows = garden.len();

        let mut counted_grid: Vec<Vec<Cell>> = vec![vec![Cell::Count(0); cols]; rows];
        let grid_as_chars: Vec<Vec<char>> = garden
            .iter()
            .map(|x| {
                x.chars()
                    .collect()
            })
            .collect();

        for i in 0..rows {
            for j in 0..cols {
                counted_grid[i][j] = {
                    match counted_grid[i][j] {
                        Cell::Count(mut count) => {
                            // flower is on the left
                            if Self::index_safe(rows, cols, i as isize, j as isize - 1)
                                && grid_as_chars[i][j - 1] == '*'
                            {
                                count += 1;
                            }
                            if Self::index_safe(rows, cols, i as isize, j as isize + 1)
                                && grid_as_chars[i][j + 1] == '*'
                            {
                                count += 1;
                            }
                            if Self::index_safe(rows, cols, i as isize + 1, j as isize)
                                && grid_as_chars[i + 1][j] == '*'
                            {
                                count += 1;
                            }
                            if Self::index_safe(
                                rows,
                                cols,
                                i as isize + 1,
                                j as isize - 1,
                            ) && grid_as_chars[i + 1][j - 1] == '*'
                            {
                                count += 1;
                            }
                            if Self::index_safe(
                                rows,
                                cols,
                                i as isize + 1,
                                j as isize + 1,
                            ) && grid_as_chars[i + 1][j + 1] == '*'
                            {
                                count += 1;
                            }
                            if Self::index_safe(rows, cols, i as isize - 1, j as isize)
                                && grid_as_chars[i - 1][j] == '*'
                            {
                                count += 1;
                            }
                            if Self::index_safe(
                                rows,
                                cols,
                                i as isize - 1,
                                j as isize - 1,
                            ) && grid_as_chars[i - 1][j - 1] == '*'
                            {
                                count += 1;
                            }
                            if Self::index_safe(
                                rows,
                                cols,
                                i as isize - 1,
                                j as isize + 1,
                            ) && grid_as_chars[i - 1][j + 1] == '*'
                            {
                                count += 1;
                            }

                            // It's a flower
                            if grid_as_chars[i][j] == '*' {
                                Cell::Flower
                            } else {
                                Cell::Count(count)
                            }
                        }
                        Cell::Flower => Cell::Flower,
                    }
                }
            }
        }

        Self {
            // garden: garden.clone(),
            counted_grid,
        }
    }

    fn to_vec_string(&self) -> Vec<String> {
        self.counted_grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| match c {
                        Cell::Count(x) => {
                            if *x == 0 {
                                String::from(" ")
                            } else {
                                x.to_string()
                            }
                        }
                        Cell::Flower => String::from("*"),
                    })
                    .collect()
            })
            .collect()
    }
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        vec![]
    } else {
        let grid = Grid::from_garden(garden);
        grid.to_vec_string()
    }
}
