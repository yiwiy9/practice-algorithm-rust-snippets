use cargo_snippet::snippet;

#[snippet]
pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, seen, next_v);
    }
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
    fn test_dfs_single_v() {
        let graph = vec![vec![]];
        let mut seen = vec![false];
        dfs(&graph, &mut seen, 0);
        assert_eq!(seen, vec![true]);
    }

    #[test]
    fn test_dfs_simple_graph() {
        let graph = create_graph();
        let mut seen = vec![false; graph.len()];
        dfs(&graph, &mut seen, 0);
        assert_eq!(seen, vec![true, true, true, true, true, true, true]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let graph = vec![vec![1], vec![0], vec![3], vec![2]];
        let mut seen = vec![false; graph.len()];
        dfs(&graph, &mut seen, 0);
        assert_eq!(seen, vec![true, true, false, false]);
    }
}
