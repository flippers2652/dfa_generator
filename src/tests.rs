#[cfg(test)]
#[test]
fn converter() {
    use crate::dfa_minimiser::minimise;
    use crate::dfa_to_table::converter;
    use crate::nfa_to_dfa::converter as dfa;
    use crate::re_to_nfa::converter as nfa;
    use crate::regular_expression_to_table;
    use crate::regular_expressions::RegularExpression;
    use crate::regular_expressions::RegularExpression::*;
    use crate::regular_expressions::Alphanumeric;

    use petgraph::dot::Dot;
    use std::fmt;
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    enum Token {
        IF,
        STRUCT,
        VAR,
        INTEGER,
    }
    impl std::fmt::Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "{:?}", self)
        }
    }

    use Token::*;
    let mut map = Vec::<(Token, RegularExpression<_>)>::new();
    map.push((
        IF,
        RegularExpression::literal("if2")
            .concatenate(&RegularExpression::literal("if").kleene_star()),
    ));
    //map.push((STRUCT,RegularExpression::literal("struct")));
    map.push((
        VAR,
        RegularExpression::alpha().concatenate(&RegularExpression::alphanumeric().kleene_star()),
    ));
    //map.push((INTEGER,RegularExpression::literal("int").concatenate(&RegularExpression::Character('+').alternate(&RegularExpression::Character('-')).alternate(&RegularExpression::Empty).concatenate(&RegularExpression::numeric()).concatenate(&RegularExpression::numeric().kleene_star()))));
    let nfa = nfa(&map);
    //println!("{}", Dot::with_config(&nfa, &[]));
    let dfa = minimise(dfa(nfa, &map));
    //println!("{}", Dot::with_config(&dfa, &[]));
    //println!("{:?}", converter(dfa));
}
