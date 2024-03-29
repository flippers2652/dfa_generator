use std::fmt;

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;

use crate::regular_expressions::RegularExpression;
use crate::regular_expressions::RegularExpression::*;

mod tests;

pub trait TokenRequirements:
    std::cmp::PartialEq + std::fmt::Display + Copy + std::hash::Hash + std::cmp::Eq + std::fmt::Debug
where
    Self: std::marker::Sized,
{
}
impl<T> TokenRequirements for T where
    T: std::cmp::PartialEq
        + std::fmt::Display
        + Copy
        + std::hash::Hash
        + std::cmp::Eq
        + std::fmt::Debug
{
}

#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
pub(in crate) enum State<Token: TokenRequirements> {
    Start,
    Standard,
    End(Token),
}

impl<Token: TokenRequirements> fmt::Display for State<Token> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Start => f.write_fmt(format_args!("Start")),
            State::Standard => f.write_fmt(format_args!("")),
            State::End(s) => f.write_fmt(format_args!("End({})", s)),
        }
    }
}

#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug)]
pub(in crate) enum BranchLabel {
    Letter(char),
    Empty,
}

impl fmt::Display for BranchLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchLabel::Letter(c) => f.write_fmt(format_args!("{}", c)),
            BranchLabel::Empty => f.write_fmt(format_args!("ε")),
        }
    }
}

pub(in crate) fn converter<Token: TokenRequirements>(
    tokens: &Vec<(Token, RegularExpression<char>)>,
) -> Graph<State<Token>, BranchLabel> {
    let mut graph = Graph::<State<Token>, BranchLabel>::new();
    let start = graph.add_node(State::Start);
    for (token, re) in tokens {
        let end = generate(&mut graph, start, re.clone());
        *(graph.node_weight_mut(end).unwrap()) = State::End(*token);
    }
    return graph;
}

fn generate<Token: TokenRequirements>(
    graph: &mut Graph<State<Token>, BranchLabel>,
    start: NodeIndex<u32>,
    expression: RegularExpression<char>,
) -> NodeIndex<u32> {
    match expression {
        Character(s) => character(graph, start, s),
        Concatenation(vec) => concatenation(graph, start, vec),
        Alternative(vec) => alternative(graph, start, vec),
        KleeneStar(a) => kleene_star(graph, start, *a),
        Empty => empty(graph, start),
    }
}

fn concatenation<Token: TokenRequirements>(
    graph: &mut Graph<State<Token>, BranchLabel>,
    start: NodeIndex<u32>,
    vec: Vec<RegularExpression<char>>,
) -> NodeIndex<u32> {
    let mut mid = start;
    for v in vec {
        mid = generate(graph, mid, v);
    }

    return mid;
}

fn alternative<Token: TokenRequirements>(
    graph: &mut Graph<State<Token>, BranchLabel>,
    start: NodeIndex<u32>,
    vec: Vec<RegularExpression<char>>,
) -> NodeIndex<u32> {
    let end = graph.add_node(State::Standard);

    for v in vec {
        let end_v = generate(graph, start, v);
        let edge = graph.add_edge(end_v, end, BranchLabel::Empty);
    }
    return end;
}

fn kleene_star<Token: TokenRequirements>(
    graph: &mut Graph<State<Token>, BranchLabel>,
    start: NodeIndex<u32>,
    a: RegularExpression<char>,
) -> NodeIndex<u32> {
    let node = graph.add_node(State::Standard);
    let end = generate(graph, node, a);
    let edge = graph.add_edge(end, start, BranchLabel::Empty);
    let edge = graph.add_edge(start, node, BranchLabel::Empty);
    return node;
}

fn character<Token: TokenRequirements>(
    graph: &mut Graph<State<Token>, BranchLabel>,
    start: NodeIndex<u32>,
    s: char,
) -> NodeIndex<u32> {
    let end = graph.add_node(State::Standard);
    let edge = graph.add_edge(start, end, BranchLabel::Letter(s));
    return end;
}

fn empty<Token: TokenRequirements>(
    graph: &mut Graph<State<Token>, BranchLabel>,
    start: NodeIndex<u32>,
) -> NodeIndex<u32> {
    let end = graph.add_node(State::Standard);
    let edge = graph.add_edge(start, end, BranchLabel::Empty);
    return end;
}
