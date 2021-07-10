use cargo_snippet::snippet;

#[snippet("dijkstra")]
#[derive(Debug, Clone, PartialEq, Eq, Ord)]
struct Node {
    cost: i64,
    v: usize,
}

#[snippet("dijkstra")]
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}

#[snippet("dijkstra")]
pub fn dijkstra(n: usize, start: usize, Graph: &Vec<Vec<(usize, i64)>>) -> (Vec<i64>, Vec<i64>) {
    let mut binary_heap = std::collections::BinaryHeap::new();
    let mut d = vec![std::i64::MAX; n];
    d[start] = 0;
    let mut shortest_path = vec![-1; n];
    binary_heap.push(Node { cost: 0, v: start });
    while let Some(Node { cost, v }) = binary_heap.pop() {
        if d[v] < cost {
            continue;
        }
        for &(nv, ncost) in &Graph[v] {
            if d[nv] <= d[v] + ncost {
                continue;
            }
            d[nv] = d[v] + ncost;
            shortest_path[nv] = v as i64;
            binary_heap.push(Node { cost: d[nv], v: nv });
        }
    }
    (d, shortest_path)
}

#[snippet(include = "dijkstra")]
fn get_shortest_path(path: Vec<i64>, end: usize) -> Vec<(i64, i64)> {
    let mut node = end as i64;
    let mut pre_shortest_path = vec![];
    while node >= 0 {
        pre_shortest_path.push(node);
        node = path[node as usize];
    }
    pre_shortest_path.reverse();
    let mut shotest_path = vec![];
    for i in 0..pre_shortest_path.len() - 1 {
        shotest_path.push((pre_shortest_path[i], pre_shortest_path[i + 1]));
    }
    shotest_path
}

#[test]
fn test_dijkstra() {
    todo!();
}
