use std::collections::HashMap;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    // Count degree of each vertex
    let mut degrees: HashMap<u8, i32> = HashMap::new();

    for &(a, b) in input {
        if a == b {
            // Self-loop contributes 2 to the degree
            *degrees.entry(a).or_default() += 2;
        } else {
            // Regular edge contributes 1 to each endpoint
            *degrees.entry(a).or_default() += 1;
            *degrees.entry(b).or_default() += 1;
        }
    }

    // Check if Eulerian cycle exists (all vertices must have even degree)
    for &degree in degrees.values() {
        if degree % 2 != 0 {
            return None;
        }
    }

    // Build adjacency list for DFS
    let mut graph: HashMap<u8, Vec<(u8, usize)>> = HashMap::new();

    for (i, &(a, b)) in input.iter().enumerate() {
        graph.entry(a).or_default().push((b, i));
        if a != b {
            graph.entry(b).or_default().push((a, i));
        }
    }

    // Find Eulerian cycle using DFS with backtracking
    let mut used = vec![false; input.len()];
    let mut result = Vec::new();

    // Start from any vertex that appears in the input
    let start = input[0].0;

    if dfs(start, start, &graph, &mut used, &mut result, input) {
        Some(result)
    } else {
        None
    }
}

fn dfs(
    current: u8,
    start: u8,
    graph: &HashMap<u8, Vec<(u8, usize)>>,
    used: &mut Vec<bool>,
    result: &mut Vec<(u8, u8)>,
    input: &[(u8, u8)],
) -> bool {
    // If we've used all dominoes, check if we're back at start
    if result.len() == input.len() {
        return current == start;
    }

    // Try each unused edge from current vertex
    if let Some(neighbors) = graph.get(&current) {
        for &(next, edge_idx) in neighbors {
            if !used[edge_idx] {
                used[edge_idx] = true;
                let domino = input[edge_idx];

                // Orient the domino correctly based on current vertex
                let oriented_domino = if domino.0 == current {
                    domino
                } else {
                    (domino.1, domino.0)
                };

                result.push(oriented_domino);

                // Recursively try to complete the chain
                if dfs(next, start, graph, used, result, input) {
                    return true;
                }

                // Backtrack
                result.pop();
                used[edge_idx] = false;
            }
        }
    }

    false
}
