#[derive(Clone, Copy, Debug)]
enum Cell {
    Count(u32),
    Flower,
}

struct Grid {
    counted_grid: Vec<Vec<Cell>>,
}

impl Grid {
    fn from_garden(garden: &[&str]) -> Self {
        if !Grid::is_valid_garden(garden) {
            // Should return an error, but I'll implement what the test expect
            if garden.is_empty() {
                return Self {
                    // garden: garden.clone(),
                    counted_grid: vec![],
                };
            }
            dbg!("pooo");

            return Self {
                // garden: garden.clone(),
                counted_grid: vec![vec![Cell::Count(0)]],
            };
        }

        let rows = garden[0].len();
        let cols = garden.len();

        let mut counted_grid: Vec<Vec<Cell>> = vec![vec![Cell::Count(0); cols]; rows];
        let grid_as_chars: Vec<Vec<char>> = garden
            .iter()
            .map(|x| {
                x.chars()
                    .collect()
            })
            .collect();

        for i in 0..cols {
            for j in 0..rows {
                counted_grid[i][j] = {
                    match counted_grid[i][j] {
                        Cell::Count(mut count) => {
                            // flower is on the left
                            if j > 0 && grid_as_chars[i][j - 1] == '*' {
                                count += 1;
                            }
                            // flower is on the right
                            if j < (cols - 1) && grid_as_chars[i][j + 1] == '*' {
                                count += 1;
                            }

                            // Above is a flower
                            if i > 0 && grid_as_chars[i - 1][j] == '*' {
                                count += 1;
                            }
                            // Below is a flower
                            if i < (rows - 1) && grid_as_chars[i + 1][j] == '*' {
                                count += 1;
                            }

                            // There a flower diagonally north-west
                            if i > 0 && j > 0 && grid_as_chars[i - 1][j - 1] == '*' {
                                count += 1;
                            }
                            // There a flower diagonally south-west
                            if i < (rows - 1) && j > 0 && grid_as_chars[i + 1][j - 1] == '*' {
                                count += 1;
                            }
                            // There a flower diagonally north-east
                            if i > 0 && j < (cols - 1) && grid_as_chars[i - 1][j + 1] == '*' {
                                count += 1;
                            }
                            // There a flower diagonally south-east
                            if i < (rows - 1)
                                && j < (cols - 1)
                                && grid_as_chars[i + 1][j + 1] == '*'
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

    fn is_valid_garden(garden: &[&str]) -> bool {
        (garden.is_empty()) && (garden[0].len() >= 2)
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
    let grid = Grid::from_garden(garden);
    grid.to_vec_string()
}
