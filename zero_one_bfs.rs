use cargo_snippet::snippet;

#[snippet]
// https://drken1215.hatenablog.com/entry/2021/07/30/024800
pub fn zero_one_bfs(
    field: &[Vec<char>],
    h: usize,
    w: usize,
    start: (usize, usize),
) -> Vec<Vec<usize>> {
    let inf: usize = 1 << 60;
    let mut dist = vec![vec![inf; w]; h];
    let mut deque = std::collections::VecDeque::new();

    let dx = [1, 0, -1, 0];
    let dy = [0, 1, 0, -1];

    dist[start.0][start.1] = 0;
    deque.push_front(start);

    while let Some((x, y)) = deque.pop_front() {
        for dir in 0..4 {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];

            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if field[nx][ny] != '#' {
                if dist[nx][ny] > dist[x][y] {
                    dist[nx][ny] = dist[x][y];
                    deque.push_front((nx, ny));
                }
            } else {
                if dist[nx][ny] > dist[x][y] + 1 {
                    dist[nx][ny] = dist[x][y] + 1;
                    deque.push_back((nx, ny));
                }
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_one_bfs() {
        let field = vec![
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['#', '#', '#', '.', '.', '.'],
            vec!['#', '#', '#', '.', '.', '.'],
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['.', '.', '.', '#', '#', '#'],
            vec!['.', '.', '#', '#', '#', '#'],
        ];
        let h = field.len();
        let w = field[0].len();
        let start = (0, 5);
        let dist = zero_one_bfs(&field, h, w, start);
        let expected = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0],
            vec![2, 2, 1, 0, 0, 0],
            vec![3, 3, 2, 1, 1, 1],
            vec![2, 2, 2, 2, 2, 2],
            vec![2, 2, 3, 3, 3, 3],
        ];
        assert_eq!(dist, expected);
    }
}
