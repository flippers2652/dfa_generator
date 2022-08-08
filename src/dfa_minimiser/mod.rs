use crate::re_to_nfa::{State, TokenRequirements};
use std::collections::HashMap;
use std::collections::HashSet;

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

mod tests;

pub(in crate) fn minimise<Token: TokenRequirements>(
    dfa: Graph<State<Token>, char>,
) -> Graph<State<Token>, char> {
    let mut alphabet = HashMap::<char, usize>::new();
    let mut count = 0;
    for letter in dfa.edge_weights().into_iter().collect::<HashSet<&char>>() {
        if alphabet.contains_key(letter) {
            continue;
        }
        alphabet.insert(*letter, count);
        count += 1;
    }
    let mut sets: Vec<HashSet<NodeIndex>> = vec![];
    for state in dfa.node_weights().into_iter().collect::<HashSet<_>>() {
        sets.push(
            dfa.node_indices()
                .filter(|node| *dfa.node_weight(*node).unwrap() == *state)
                .collect(),
        );
    }
    loop {
        let mut new_sets = Vec::<HashSet<NodeIndex>>::new();
        for set in &sets {
            let mut map_arrows = Vec::<HashMap<char, &HashSet<NodeIndex>>>::new();
            let mut map_locations = Vec::<HashSet<NodeIndex>>::new();
            //println!("{:?}",set);
            for &node in set {
                //println!("\t{:?}",node);
                let mut arrows = HashMap::<char, &HashSet<NodeIndex>>::new();
                for neighbor in dfa.neighbors(node) {
                    for edge in dfa.edges_connecting(node, neighbor) {
                        for set in &sets {
                            if set.contains(&neighbor) {
                                arrows.insert(*dfa.edge_weight(edge.id()).unwrap(), set);
                                break;
                            };
                        }
                    }
                }
                let index = map_arrows.iter().position(|x| *x == arrows);
                match index {
                    None => {
                        map_arrows.push(arrows);
                        map_locations.push(vec![node].into_iter().collect());
                    }
                    Some(x) => {
                        map_locations[x].insert(node);
                    }
                };
                //println!("\t\t{:?}",map_locations);
            }
            for loc in map_locations {
                new_sets.push(loc);
            }
        }
        if sets == new_sets {
            break;
        } else {
            sets = new_sets;
            //println!("{:?}",sets);
        }
    }
    //let sets: Vec<HashSet<NodeIndex>>=sets.into_iter().filter(|set| !set.is_empty()).collect();
    let mut nodes = vec![];
    let mut minimised_dfa = Graph::<State<Token>, char>::new();
    for set in &sets {
        if let Some(node) = set.iter().next() {
            nodes.push(minimised_dfa.add_node(*dfa.node_weight(*node).unwrap()));
        }
    }
    assert_eq!(sets.len(),nodes.len());
    for set in &sets {
        if let Some(node) = set.iter().next() {
            for neighbor in dfa.neighbors(*node) {
                for edge in dfa.edges_connecting(*node, neighbor) {
                    for neighbor_set in sets.iter().filter(|set| set.contains(&neighbor)) {
                        minimised_dfa.add_edge(
                            nodes[sets.iter().position(|s| *s == *set).unwrap()],
                            nodes[sets.iter().position(|s| *s == *neighbor_set).unwrap()],
                            *dfa.edge_weight(edge.id()).expect("edge"),
                        );
                    }
                }
            }
        }
    }
    return minimised_dfa;
}
