use cargo_snippet::snippet;

#[snippet]
pub fn rotate_grid(field: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = field.len();
    let m = field[0].len();
    let mut result = vec![vec!['.'; n]; m];

    for (i, row) in field.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            result[j][n - 1 - i] = c;
        }
    }

    result
}

#[snippet]
pub fn trim_grid(field: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut top = field.len();
    let mut bottom = 0;
    let mut left = field[0].len();
    let mut right = 0;

    for (i, row) in field.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                top = top.min(i);
                bottom = bottom.max(i);
                left = left.min(j);
                right = right.max(j);
            }
        }
    }

    if top > bottom || left > right {
        return Vec::new();
    }

    field[top..=bottom]
        .iter()
        .map(|row| row[left..=right].to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_grid_rectangular_field() {
        let field = vec![
            vec!['.', '.', '#'],
            vec!['.', '#', '.'],
            vec!['#', '.', '.'],
            vec!['#', '.', '#'],
        ];

        let rotated = rotate_grid(&field);

        let expected = vec![
            vec!['#', '#', '.', '.'],
            vec!['.', '.', '#', '.'],
            vec!['#', '.', '.', '#'],
        ];

        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rotate_grid_square_field() {
        let field = vec![
            vec!['.', '.', '#'],
            vec!['.', '#', '.'],
            vec!['#', '#', '.'],
        ];

        let rotated = rotate_grid(&field);

        let expected = vec![
            vec!['#', '.', '.'],
            vec!['#', '#', '.'],
            vec!['.', '.', '#'],
        ];

        assert_eq!(rotated, expected);
    }

    #[test]
    fn test_rotate_grid_full_circle() {
        let field = vec![
            vec!['.', '.', '#', '.'],
            vec!['.', '#', '.', '.'],
            vec!['#', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
        ];

        let once_rotated = rotate_grid(&field);
        let twice_rotated = rotate_grid(&once_rotated);
        let thrice_rotated = rotate_grid(&twice_rotated);
        let full_circle_rotated = rotate_grid(&thrice_rotated);

        assert_eq!(field, full_circle_rotated);
    }

    #[test]
    fn test_trim_grid() {
        let field = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '#', '#', '#', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];

        let trimmed = trim_grid(&field);

        let expected = vec![
            vec!['.', '#', '.'],
            vec!['.', '#', '.'],
            vec!['#', '#', '#'],
        ];

        assert_eq!(trimmed, expected);
    }

    #[test]
    fn test_trim_grid_no_change_needed() {
        let field = vec![
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '.', '#', '.', '.'],
            vec!['#', '#', '#', '#', '#'],
            vec!['.', '.', '#', '.', '.'],
        ];

        let trimmed = trim_grid(&field);

        assert_eq!(trimmed, field);
    }

    #[test]
    fn test_trim_grid_single_cell() {
        let field = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '#', '.'],
        ];

        let trimmed = trim_grid(&field);

        let expected = vec![vec!['#']];

        assert_eq!(trimmed, expected);
    }

    #[test]
    fn test_trim_grid_empty_field() {
        let field = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];

        let trimmed = trim_grid(&field);

        let expected: Vec<Vec<char>> = Vec::new();

        assert_eq!(trimmed, expected);
    }
}
