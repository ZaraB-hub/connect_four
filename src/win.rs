pub fn check_win(grid: &Vec<Vec<char>>, rows: usize, cols: usize) -> bool {
    // Check for wins on rows
    for i in 0..rows {
        for j in 0..cols-3 {
            if grid[i][j] == grid[i][j + 1] && grid[i][j] == grid[i][j + 2] && grid[i][j] == grid[i][j + 3] && grid[i][j] != '\u{2B1B}' {
                return true;
            }
        }
    }

    // Check for wins on columns
    for j in 0..cols {
        for i in 0..rows-3 {
            if grid[i][j] == grid[i + 1][j] && grid[i][j] == grid[i + 2][j] && grid[i][j] == grid[i + 3][j] && grid[i][j] != '\u{2B1B}'{
                return true;
            }
        }
    }

    // Check for wins on diagonals
    for i in 0..rows-3 {
        for j in 0..cols-3 {
            if grid[i][j] == grid[i + 1][j + 1] && grid[i][j] == grid[i + 2][j + 2] && grid[i][j] == grid[i + 3][j + 3] && grid[i][j] != '\u{2B1B}' {
                return true;
            }
            if grid[i][j + 3] == grid[i + 1][j + 2] && grid[i][j + 3] == grid[i + 2][j + 1] && grid[i][j + 3] == grid[i + 3][j] && grid[i][j + 3] != '\u{2B1B}' {
                return true;
            }
        }
    }

    // No wins found
    false
}