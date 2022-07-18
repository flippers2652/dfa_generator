use std::collections::HashMap;

use crate::re_to_nfa::{State, TokenRequirements};
use crate::table::Table;

use crate::table::State as TableState;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

mod tests;

pub(in crate) fn converter<Token: TokenRequirements>(
    dfa: Graph<State<Token>, char>,
) -> Table<Token> {
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
    let mut start_id: Option<usize> = None;
    let mut states = vec![TableState::<Token>::Error; dfa.node_count() + 1];
    for (node, id) in dfa.node_indices().zip(1..dfa.node_count() + 1) {
        nodes.insert(node, id);
        match *dfa.node_weight(node).unwrap() {
            State::<Token>::Start => {
                start_id = Some(id);
                states[id] = TableState::<Token>::Standard;
            }
            State::<Token>::Standard => states[id] = TableState::<Token>::Standard,
            State::<Token>::End(token) => states[id] = TableState::<Token>::Token(token),
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
    let start_id = start_id.expect("No Start State");
    return Table {
        table,
        alphabet,
        start_id,
        current_id: start_id,
        states,
    };
}
