use std::collections::HashMap;

use crate::re_to_nfa::State;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

mod tests;

fn convert(dfa: Graph<State, char>) -> (Vec<Vec<usize>>, HashMap<char, usize>, usize) {
    let mut alphabet = HashMap::<char, usize>::new();
    let mut count = 0;
    for edge in dfa.edge_indices() {
        let letter = dfa.edge_weight(edge).unwrap();
        if alphabet.contains_key(&letter) {
            continue;
        }
        alphabet.insert(*letter, count);
        count += 1;
    }

    let mut nodes = HashMap::<NodeIndex, usize>::new();
    let mut start_state: Option<usize> = None;
    for (node, id) in dfa.node_indices().zip(1..dfa.node_count() + 1) {
        nodes.insert(node, id);
        if *dfa.node_weight(node).unwrap() == State::Start {
            start_state = Some(id)
        }
    }

    let mut table = vec![vec![0; alphabet.len()]; nodes.len() + 1];
    for node in dfa.node_indices() {
        for neighbor in dfa.neighbors(node) {
            let edges = dfa.edges_connecting(node, neighbor);
            for edge in edges.into_iter() {
                let weight = dfa.edge_weight(edge.id()).unwrap();
                table[nodes[&node]][alphabet[weight]] = nodes[&neighbor];
            }
        }
    }
    let start_state = start_state.expect("No Start State");
    return (table, alphabet, start_state);
}
