#[cfg(test)]
#[test]
fn dfa_to_table() {
    use crate::dfa_to_table::converter;
    use crate::nfa_to_dfa::converter as dfa;
    use crate::re_to_nfa::converter as nfa;
    use crate::regular_expressions::RegularExpression;
    use crate::regular_expressions::RegularExpression::*;
    use petgraph::dot::Dot;
    let re = Character('a');
    let left = Character('a').alternate(&Character('b')).kleene_star();
    let right = Character('b').alternate(&Character('c')).kleene_star();
    let mid = left.alternate(&right);
    let re = re.concatenate(&mid).concatenate(&Character('c'));
    let start = Character('a')
        .alternate(&Character('b'))
        .alternate(&Character('c'))
        .kleene_star();
    let re = start.concatenate(&re);
    let re = start.concatenate(&re);
    let mut map = Vec::<(&str, RegularExpression<_>)>::new();

    let table = converter(dfa(nfa(&map), &map));
    //Todo
}
