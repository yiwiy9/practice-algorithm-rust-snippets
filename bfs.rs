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
}
