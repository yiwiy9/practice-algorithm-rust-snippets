use cargo_snippet::snippet;

#[snippet]
pub fn doubling(next_nodes: &Vec<usize>, node: usize, k: usize) -> usize {
    let n = next_nodes.len();
    let max_log = (k as f64).log2().ceil() as usize;
    let mut doubling_table = vec![vec![0; n]; max_log + 1];

    // 初期化します。iから始まる1ステップ先はnext_nodes[i]です。
    doubling_table[0][..n].copy_from_slice(&next_nodes[..n]);

    // 2^i(=2^(i-1)+2^(i-1))ステップ先のノードを求めます。
    for i in 1..=max_log {
        for j in 0..n {
            doubling_table[i][j] = doubling_table[i - 1][doubling_table[i - 1][j]];
        }
    }

    let mut current_node = node;

    // 2進数表現の各ビットを見ていきます。
    for (i, i_steps_nodes) in doubling_table.iter().enumerate().take(max_log + 1) {
        if (k >> i) & 1 == 1 {
            current_node = i_steps_nodes[current_node];
        }
    }

    current_node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubling_single_node() {
        let next_nodes = vec![0]; // 0 -> 0
        assert_eq!(doubling(&next_nodes, 0, 1), 0);
        assert_eq!(doubling(&next_nodes, 0, 10), 0);
    }

    #[test]
    fn test_doubling_two_nodes() {
        let next_nodes = vec![1, 0]; // 0 -> 1 -> 0
        assert_eq!(doubling(&next_nodes, 0, 1), 1);
        assert_eq!(doubling(&next_nodes, 0, 2), 0);
        assert_eq!(doubling(&next_nodes, 0, 3), 1);
    }

    #[test]
    fn test_doubling_chain() {
        let next_nodes = vec![1, 2, 3, 4, 0]; // 0 -> 1 -> 2 -> 3 -> 4 -> 0
        assert_eq!(doubling(&next_nodes, 0, 1), 1);
        assert_eq!(doubling(&next_nodes, 0, 2), 2);
        assert_eq!(doubling(&next_nodes, 0, 3), 3);
        assert_eq!(doubling(&next_nodes, 0, 4), 4);
        assert_eq!(doubling(&next_nodes, 0, 5), 0);
        assert_eq!(doubling(&next_nodes, 0, 6), 1);
    }
}
