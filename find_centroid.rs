use cargo_snippet::snippet;

#[snippet("find_centroid")]
fn subtree_dfs(
    graph: &Vec<Vec<usize>>,
    weight: &Vec<usize>,
    subtree_weight: &mut Vec<usize>,
    v: usize,
    par: usize,
) -> usize {
    subtree_weight[v] = weight[v];
    for &next_v in &graph[v] {
        if next_v == par {
            continue;
        }
        subtree_weight[v] += subtree_dfs(graph, weight, subtree_weight, next_v, v);
    }
    subtree_weight[v]
}

#[snippet("find_centroid")]
fn centroid_dfs(
    n: usize,
    graph: &Vec<Vec<usize>>,
    weight_sum: usize,
    subtree_weight: &Vec<usize>,
    v: usize,
    par: usize,
) -> usize {
    let mut is_centroid = true;
    for &next_v in &graph[v] {
        if next_v == par {
            continue;
        }
        let res = centroid_dfs(n, graph, weight_sum, subtree_weight, next_v, v);
        if res != n {
            return res;
        }
        if subtree_weight[next_v] > weight_sum / 2 {
            is_centroid = false;
        }
    }
    if weight_sum - subtree_weight[v] > weight_sum / 2 {
        is_centroid = false;
    }
    if is_centroid {
        v
    } else {
        n
    }
}

#[snippet("find_centroid")]
pub fn find_centroid(graph: &Vec<Vec<usize>>, weight: &Vec<usize>) -> usize {
    let n = graph.len();
    let weight_sum = weight.iter().sum::<usize>();

    let mut subtree_weight = vec![0; n];
    subtree_dfs(graph, weight, &mut subtree_weight, 0, n);
    centroid_dfs(n, graph, weight_sum, &subtree_weight, 0, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_centroid() {
        // Test case 1: Single node tree
        {
            let graph = vec![vec![]];
            let weight = vec![1];
            let centroid = find_centroid(&graph, &weight);
            assert_eq!(centroid, 0);
        }

        // Test case 2: Two nodes
        {
            let graph = vec![vec![1], vec![0]];
            let weight = vec![1, 1];
            let centroid = find_centroid(&graph, &weight);
            assert_eq!(centroid, 1);
        }

        // Test case 3: Linear tree (unbalanced)
        {
            // Tree structure:
            // 0 - 1 - 2 - 3 - 4
            let n = 5;
            let mut graph = vec![vec![]; n];
            for i in 0..n - 1 {
                graph[i].push(i + 1);
                graph[i + 1].push(i);
            }
            let weight = vec![1; n];
            let centroid = find_centroid(&graph, &weight);
            // The centroid should be the middle node
            assert_eq!(centroid, 2);
        }

        // Test case 4: Balanced binary tree
        {
            // Tree structure:
            //        0
            //      /   \
            //     1     2
            //    / \   / \
            //   3  4  5  6
            let graph = vec![
                vec![1, 2],    // 0
                vec![0, 3, 4], // 1
                vec![0, 5, 6], // 2
                vec![1],       // 3
                vec![1],       // 4
                vec![2],       // 5
                vec![2],       // 6
            ];
            let weight = vec![1; 7];
            let centroid = find_centroid(&graph, &weight);
            // The centroid should be the root node
            assert_eq!(centroid, 0);
        }

        // Test case 5: Tree with varying weights
        {
            // Tree structure:
            //        0
            //      /   \
            //     1     2
            //    / \
            //   3   4
            let graph = vec![
                vec![1, 2],    // 0
                vec![0, 3, 4], // 1
                vec![0],       // 2
                vec![1],       // 3
                vec![1],       // 4
            ];
            let weight = vec![1, 1, 10, 1, 1]; // Node 2 has a heavy weight
            let centroid = find_centroid(&graph, &weight);
            // The centroid should be node 2 because it has the largest weight
            assert_eq!(centroid, 2);
        }

        // Test case 6: Complex tree
        {
            // Tree structure:
            //        0
            //      / | \
            //     1  2  3
            //         |
            //         4
            //        / \
            //       5   6
            let graph = vec![
                vec![1, 2, 3], // 0
                vec![0],       // 1
                vec![0, 4],    // 2
                vec![0],       // 3
                vec![2, 5, 6], // 4
                vec![4],       // 5
                vec![4],       // 6
            ];
            let weight = vec![1; 7];
            let centroid = find_centroid(&graph, &weight);
            // The centroid should be node 2
            assert_eq!(centroid, 2);
        }
    }
}
