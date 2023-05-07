use cargo_snippet::snippet;

#[snippet(name = "__dijkstra_struct")]
#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    vertex: usize,
    cost: usize,
}

#[snippet(name = "__dijkstra_cmp")]
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[snippet(name = "__dijkstra_partial_cmp")]
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[snippet(include = "__dijkstra_struct")]
#[snippet(include = "__dijkstra_cmp")]
#[snippet(include = "__dijkstra_partial_cmp")]
pub fn dijkstra(graph: &[Vec<(usize, usize)>], start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![std::usize::MAX; n];
    let mut pq = std::collections::BinaryHeap::new();

    dist[start] = 0;
    pq.push(Node {
        vertex: start,
        cost: 0,
    });

    while let Some(Node { vertex, cost }) = pq.pop() {
        if dist[vertex] < cost {
            continue;
        }

        for &(next_vertex, edge_cost) in &graph[vertex] {
            let new_cost = cost + edge_cost;
            if new_cost < dist[next_vertex] {
                dist[next_vertex] = new_cost;
                pq.push(Node {
                    vertex: next_vertex,
                    cost: new_cost,
                });
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = vec![
            vec![(1, 2), (2, 5)],
            vec![(0, 2), (3, 3)],
            vec![(0, 5), (3, 1)],
            vec![(1, 3), (2, 1)],
        ];

        let start_vertex = 0;
        let dist = dijkstra(&graph, start_vertex);
        let expected = vec![0, 2, 5, 5];
        assert_eq!(dist, expected);

        let start_vertex = 1;
        let dist = dijkstra(&graph, start_vertex);
        let expected = vec![2, 0, 4, 3];
        assert_eq!(dist, expected);

        let start_vertex = 2;
        let dist = dijkstra(&graph, start_vertex);
        let expected = vec![5, 4, 0, 1];
        assert_eq!(dist, expected);

        let start_vertex = 3;
        let dist = dijkstra(&graph, start_vertex);
        let expected = vec![5, 3, 1, 0];
        assert_eq!(dist, expected);
    }
}
