use advent_of_code_2025::{input_to_string, BitArray};
use std::collections::HashMap;

struct Graph {
    label_to_index: HashMap<String, usize>,
    index_to_label: Vec<String>,
    edges: Vec<Vec<usize>>
}

impl Graph {
    fn new() -> Self {
        Self { label_to_index: HashMap::new(), index_to_label: Vec::new(), edges: Vec::new() }
    }
    
    fn parse(input: &str) -> Self {
        let mut graph = Self::new();
        
        input.lines().for_each(|line| {
            let (parent, combined_children) = line.split_at(line.find(':').unwrap() + 1);
            let parent = parent.trim_matches(':');
            let children: Vec<&str> = combined_children.split(' ').collect();

            let index =  graph.add_or_get_node_index(parent);
            let child_indices: Vec<usize> = children.iter().filter(|child| !child.trim().is_empty()).map(|child| graph.add_or_get_node_index(child.trim())).collect();
            graph.edges[index].extend(child_indices);
        });

        graph
    }

    fn add_or_get_node_index(&mut self, label: &str) -> usize {
        let index = self.label_to_index.get(label).copied();
        match index {
            Some(index) => {
                index
            }
            None => {
                let index = self.index_to_label.len();
                self.index_to_label.push(label.to_string());
                self.label_to_index.insert(label.to_string(), index);
                self.edges.push(Vec::new());
                index
            }
        }
    }
}

fn count_paths(graph: &Graph, from: usize, to: usize, paths_count: &mut usize) {
    if from == to {
        *paths_count += 1;
        return
    }

    graph.edges[from].iter().for_each(|child| count_paths(graph, *child, to, paths_count));
}

fn part1() {
    let input = input_to_string("day11.txt");
    let graph = Graph::parse(&input);

    let you_index = graph.label_to_index["you"];
    let out_index = graph.label_to_index["out"];

    let mut paths_count = 0;
    count_paths(&graph, you_index, out_index, &mut paths_count);

    println!("Paths: {}", paths_count);
}

fn count_paths_with_cache(graph: &Graph, from: usize, to: usize, mut visited: BitArray, cache: &mut Vec<i64>) -> i64 {
    if cache[from] != -1 {
        return cache[from];
    }
    
    // Unnecessary cycle check
    if visited.get(from) {
        cache[from] = 0;
        return 0;
    }
    
    if from == to {
        cache[from] = 1;
        return 1;
    }

    visited.set(from, true);

    let current_paths = graph.edges[from].iter().map(|child| count_paths_with_cache(graph, *child, to, visited.clone(), cache)).sum();
    cache[from] = current_paths;

    return current_paths;
}

fn part2() {
    let input = input_to_string("day11.txt");
    let graph = Graph::parse(&input);

    let svr_to_fft = count_paths_with_cache(&graph, graph.label_to_index["svr"], graph.label_to_index["fft"], BitArray::new(graph.edges.len()), &mut vec![-1; graph.edges.len()]);
    let svr_to_dac = count_paths_with_cache(&graph, graph.label_to_index["svr"], graph.label_to_index["dac"], BitArray::new(graph.edges.len()), &mut vec![-1; graph.edges.len()]);
    let fft_to_dac = count_paths_with_cache(&graph, graph.label_to_index["fft"], graph.label_to_index["dac"], BitArray::new(graph.edges.len()), &mut vec![-1; graph.edges.len()]);
    let dac_to_fft = count_paths_with_cache(&graph, graph.label_to_index["dac"], graph.label_to_index["fft"], BitArray::new(graph.edges.len()), &mut vec![-1; graph.edges.len()]);
    let fft_to_out = count_paths_with_cache(&graph, graph.label_to_index["fft"], graph.label_to_index["out"], BitArray::new(graph.edges.len()), &mut vec![-1; graph.edges.len()]);
    let dac_to_out = count_paths_with_cache(&graph, graph.label_to_index["dac"], graph.label_to_index["out"], BitArray::new(graph.edges.len()), &mut vec![-1; graph.edges.len()]);

    println!("Paths: {}", svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out);
}

fn main() {
    part1();
    part2();
}
