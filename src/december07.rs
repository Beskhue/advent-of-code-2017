use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

extern crate itertools;
use itertools::Itertools;

extern crate regex;
extern crate petgraph;

#[derive(Clone)]
struct Node {
    name: String,
    weight: u32,
    total_weight: u32
}

fn lines_from_file<P>(filename: P) -> Result<Vec<String>, io::Error>
where
    P: AsRef<Path>,
{
    let file = try!(File::open(filename));
    let buf = std::io::BufReader::new(file);
    buf.lines().collect()
}

fn median(mut numbers: Vec<u32>) -> u32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn main() {
    // Regular expression to parse only alphanumeric words
    let re = regex::Regex::new("[0-9A-Za-z_]+").unwrap();

    // Create a vec of node specifications (Name, Weight, AdjacentNodes)
    let nodes: Vec<_> = lines_from_file("input/december07.txt")
        .unwrap()
        .iter()
        .cloned()
        .map(|l| {
            let matches: Vec<String> = re.find_iter(&l)
                .map(|regex_match| regex_match.as_str().to_string())
                .collect();
            (
                Node {name: matches[0].to_string(), weight: matches[1].parse::<u32>().unwrap(), total_weight: 0},
                matches[2..].iter().cloned().collect::<Vec<String>>() // Adjacencies
            )
        })
        .collect();

    let mut node_ids = HashMap::new();
    let mut graph = petgraph::Graph::<Node, ()>::new();

    // Insert all nodes
    for &(ref node, _) in &nodes {
        let node_id = graph.add_node((*node).clone());
        node_ids.insert((*node.name).to_string(), node_id);
    }

    // Insert all edges
    for &(ref node, ref adjacent_nodes) in &nodes {
        for adjacent_node in adjacent_nodes {
            graph.add_edge(node_ids[&node.name], node_ids[adjacent_node], ());
        }
    }

    // Sort the graph topologically
    let sorted_graph_node_ids = petgraph::algo::toposort(&graph, None).unwrap();
    println!("Part 1: {:#?}", graph[sorted_graph_node_ids[0]].name);

    // Walk through the graph in reverse order, finding the first node that
    // has imbalanced children (where one child has a different weight than the
    // others -- note that precisely one node has a wrong weight)
    for &node_id in sorted_graph_node_ids.iter().rev() {
        if graph.neighbors(node_id).map(|neighbor| &graph[neighbor].total_weight).all_equal() {
            // This node is fine
            graph[node_id].total_weight = graph[node_id].weight + graph.neighbors(node_id).map(|neighbor| &graph[neighbor].total_weight).sum::<u32>();
        } else {
            // The imbalance is below this node
            let weights = graph.neighbors(node_id).map(|neighbor| graph[neighbor].total_weight).collect::<Vec<u32>>();
            let normal_weight = median(weights.clone());

            let mut idx_odd_one_out = 0;
            for (idx, weight) in weights.iter().enumerate() {
                if weight != &normal_weight {
                    idx_odd_one_out = idx;
                    break;
                }
            }

            let neighbor_node_id = graph.neighbors(node_id).collect::<Vec<_>>()[idx_odd_one_out];
            let correct_weight = normal_weight - (&graph[neighbor_node_id].total_weight - &graph[neighbor_node_id].weight);

            println!("Part 2: Program with wrong weight: {:#?} (should be: {:#?})", &graph[neighbor_node_id].name, correct_weight);

            break;
        }
    }
}
