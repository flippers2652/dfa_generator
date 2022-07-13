use crate::regular_expression::RegularExpression;
use crate::regular_expression::RegularExpression::*;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use std::fmt;
#[derive(Debug)]
pub enum State {
    Start,
    Standard,
    End(&'static str),
}
pub enum BranchLabel{
    Letter(char),
    Empty
}

impl fmt::Debug for BranchLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BranchLabel::Letter(c) => f.write_fmt(format_args!("{}",c)),
            BranchLabel::Empty => f.write_fmt(format_args!("ε"))

        }



    }
}


pub fn converter(expression: RegularExpression) -> Graph<State, BranchLabel>{
    let mut graph = Graph::<State, BranchLabel>::new();
    let start = graph.add_node(State::Start);
    let end = generate(&mut graph,start, expression);
    return graph;
}

fn generate(
    graph: &mut Graph<State, BranchLabel>,
    start:NodeIndex<u32>,
    expression: RegularExpression,
) -> NodeIndex<u32> {
    match expression {
        Character(s) => character(graph,start, s),
        Concatenation(a, b) => concatenation(graph,start, *a, *b),
        Alternative(a, b) => alternative(graph,start, *a, *b),
        KleeneStar(a) => kleene_star(graph,start, *a),
        Empty => empty(graph,start),
    }
}

fn concatenation(
    graph: &mut Graph<State, BranchLabel>,
    start:NodeIndex<u32>,
    a: RegularExpression,
    b: RegularExpression,
)  -> NodeIndex<u32> {


    let mid = generate(graph,start,a);

    let end =generate(graph,mid,b);
    return end;
}

fn alternative(
    graph: &mut Graph<State, BranchLabel>,
    start:NodeIndex<u32>,
    a: RegularExpression,
    b: RegularExpression,
)-> NodeIndex<u32>  {
    let end = graph.add_node(State::Standard);
    let end_a = generate(graph,start,a);

    let edge = graph.add_edge(end_a, end, BranchLabel::Empty);
    let end_b = generate(graph,start,b);
    let edge = graph.add_edge(end_b, end, BranchLabel::Empty);
    return end;
}

fn kleene_star(
    graph: &mut Graph<State, BranchLabel>,
    start:NodeIndex<u32>,
    a: RegularExpression,
) -> NodeIndex<u32> {
    let end = generate(graph,start,a);
    let edge = graph.add_edge(end, start, BranchLabel::Empty);
    return end;
}

fn character(graph: &mut Graph<State, BranchLabel>,start:NodeIndex<u32>, s: char)-> NodeIndex<u32>  {
    let end = graph.add_node(State::Standard);
    let edge = graph.add_edge(start, end, BranchLabel::Letter(s));
    return end;
}

fn empty(graph: &mut Graph<State, BranchLabel>,start:NodeIndex<u32>,) -> NodeIndex<u32> {
    let end = graph.add_node(State::Standard);
    let edge = graph.add_edge(start, end, BranchLabel::Empty);
    return end;
}
