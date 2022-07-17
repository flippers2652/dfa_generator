use std::collections::HashSet;
use std::collections::VecDeque;

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;

use crate::re_to_nfa::{BranchLabel, State};

mod tests;

pub(in crate) fn converter(nfa: Graph<State, BranchLabel>) -> Graph<State, char> {
    let mut start = None;
    for node in nfa.node_indices() {
        if *(nfa.node_weight(node).unwrap()) == State::Start {
            start = Some(node);
        }
    }
    let start_set = closure(
        &nfa,
        vec![start.unwrap()].into_iter().collect(),
        BranchLabel::Empty,
    );
    let mut queue = VecDeque::new();
    queue.push_back(start_set.clone());

    let mut dfa = Graph::<State, char>::new();
    let start_node = dfa.add_node(State::Start);
    let mut sets = vec![start_set];
    let mut nodes = vec![start_node];
    while !queue.is_empty() {
        let indices = queue.pop_front().unwrap();
        let mut alphabet = HashSet::new();
        let node_index = sets.iter().position(|x| *x == indices).unwrap();
        let current_node = nodes[node_index];
        for index in &indices {
            for edge in nfa.edges(index.clone()) {
                if let BranchLabel::Letter(c) = *(edge.weight()) {
                    alphabet.insert(c);
                }
            }
        }

        for character in alphabet {
            let mut set = HashSet::new();
            for &index in &indices {
                for neighbor in nfa.neighbors(index.clone()) {
                    for edge in nfa.edges_connecting(index, neighbor) {
                        if *edge.weight() == BranchLabel::Letter(character) {
                            set.insert(neighbor);
                        }
                    }
                }
            }
            let set = closure(&nfa, set, BranchLabel::Empty);
            if sets.contains(&set) {
                let index = sets.iter().position(|x| *x == set).unwrap();
                let node = nodes[index];
                let edge = dfa.add_edge(current_node, node, character);
            } else {
                let mut state = State::Standard;
                for &node in &set {
                    if let State::End(s) = *nfa.node_weight(node).unwrap() {
                        state = State::End(s);
                    }
                }
                sets.push(set.clone());
                queue.push_back(set);

                let node = dfa.add_node(state);
                nodes.push(node);
                let edge = dfa.add_edge(current_node, node, character);
            }
        }
    }
    return dfa;
}
pub(in crate) fn closure(
    graph: &Graph<State, BranchLabel>,
    indices: HashSet<NodeIndex>,
    label: BranchLabel,
) -> HashSet<NodeIndex> {
    let mut set = HashSet::new();
    if label == BranchLabel::Empty {
        for index in indices.clone() {
            set.insert(index);
        }
    }
    let mut indices = indices;
    while !indices.is_empty() {
        let mut next_indices = HashSet::new();
        for &index in &indices {
            for node in graph.neighbors(index) {
                for edge in graph.edges_connecting(index, node) {
                    if *edge.weight() == label {
                        set.insert(node);
                        next_indices.insert(node);
                        break;
                    }
                }
            }
        }
        indices = next_indices;
    }
    return set;
}
