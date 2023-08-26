use cargo_snippet::snippet;

#[snippet]
pub fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let inf = (1 << 30) as usize;

    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut que = std::collections::VecDeque::new();

    dist[s] = 0;
    que.push_back(s);

    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if dist[v] != inf {
                continue;
            }
            dist[v] = dist[u] + 1;
            que.push_back(v);
        }
    }
    dist
}

#[snippet]
pub fn grid_bfs(field: &Vec<Vec<char>>, s: (usize, usize)) -> Vec<Vec<usize>> {
    let inf: usize = 1 << 30;
    let dx: [i32; 4] = [1, 0, -1, 0];
    let dy: [i32; 4] = [0, 1, 0, -1];

    if field.is_empty() {
        return Vec::new();
    }

    let h = field.len();
    let w = field[0].len();
    let mut dist = vec![vec![inf; w]; h];
    let mut que = std::collections::VecDeque::new();

    dist[s.0][s.1] = 0;
    que.push_back(s);

    while let Some((x, y)) = que.pop_front() {
        for dir in 0..4 {
            let nx = x as i32 + dx[dir];
            let ny = y as i32 + dy[dir];

            if nx < 0 || h as i32 <= nx || ny < 0 || w as i32 <= ny {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if field[nx][ny] == '#' {
                continue;
            }
            if dist[nx][ny] != inf {
                continue;
            }

            dist[nx][ny] = dist[x][y] + 1;
            que.push_back((nx, ny))
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_graph() -> Vec<Vec<usize>> {
        vec![
            vec![1, 2],
            vec![0, 3, 4],
            vec![0, 5, 6],
            vec![1],
            vec![1],
            vec![2],
            vec![2],
        ]
    }

    #[test]
    fn test_bfs_single_node() {
        let graph = vec![vec![]];
        let result = bfs(&graph, 0);
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_bfs_simple_graph() {
        let graph = create_graph();
        let result = bfs(&graph, 0);
        assert_eq!(result, vec![0, 1, 1, 2, 2, 2, 2]);
    }

    #[test]
    fn test_bfs_simple_graph_alternate_start() {
        let graph = create_graph();
        let result = bfs(&graph, 3);
        assert_eq!(result, vec![2, 1, 3, 0, 2, 4, 4]);
    }

    #[test]
    fn test_bfs_disconnected_graph() {
        let graph = vec![vec![1], vec![0], vec![3], vec![2]];
        let result = bfs(&graph, 0);
        let inf = (1 << 30) as usize;
        assert_eq!(result, vec![0, 1, inf, inf]);
    }

    #[test]
    fn test_grid_bfs_single_cell() {
        let field = vec![vec!['.']];
        let result = grid_bfs(&field, (0, 0));
        assert_eq!(result, vec![vec![0]]);
    }

    #[test]
    fn test_grid_bfs_obstacles() {
        let field = vec![
            vec!['.', '.', '.'],
            vec!['.', '#', '.'],
            vec!['.', '.', '.'],
        ];
        let result = grid_bfs(&field, (0, 0));
        let inf = (1 << 30) as usize;
        assert_eq!(result, vec![vec![0, 1, 2], vec![1, inf, 3], vec![2, 3, 4]]);
    }

    #[test]
    fn test_grid_bfs_disconnected() {
        let field = vec![
            vec!['.', '#', '.'],
            vec!['#', '#', '#'],
            vec!['.', '#', '.'],
        ];
        let result = grid_bfs(&field, (0, 0));
        let inf = (1 << 30) as usize;
        assert_eq!(
            result,
            vec![vec![0, inf, inf], vec![inf, inf, inf], vec![inf, inf, inf]]
        );
    }

    #[test]
    fn test_grid_bfs_empty_field() {
        let field: Vec<Vec<char>> = Vec::new();
        let result = grid_bfs(&field, (0, 0));
        assert!(result.is_empty());
    }
}
