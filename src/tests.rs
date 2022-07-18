#[cfg(test)]
#[test]
fn converter() {
    use crate::dfa_to_table::converter;
    use std::fmt;
    use crate::nfa_to_dfa::converter as dfa;
    use crate::re_to_nfa::converter as nfa;
    use crate::regular_expression_to_table;
    use crate::dfa_minimiser::minimise;
    use crate::regular_expressions::RegularExpression;
    use crate::regular_expressions::RegularExpression::*;
    use petgraph::dot::Dot;
    #[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
    enum Token {
        IF,
        STRUCT,
        I32,
        Var
    }
    impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{:?}", self)
    }
}

    use Token::*;
    let mut map = Vec::<(Token, RegularExpression)>::new();
    map.push((IF,RegularExpression::literal("if")));
    map.push((I32,RegularExpression::literal("i32")));
    map.push((STRUCT,RegularExpression::literal("i64")));
    map.push((Var,RegularExpression::alpha().concatenate(&RegularExpression::alphanumeric().kleene_star())));
    let nfa =nfa(&map);
    println!("{:?}", Dot::with_config(&nfa, &[]));
    let dfa = minimise(dfa(nfa, &map));
    //println!("{:?}", Dot::with_config(&dfa, &[]));
}
