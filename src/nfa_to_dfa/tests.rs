#[cfg(test)]
#[test]
fn nfa_to_dfa() {
    use crate::nfa_to_dfa::converter as dfa;
    use crate::re_to_nfa::converter;
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
    let mut map = Vec::<(&str, RegularExpression<_>)>::new();
    map.push(("", re));
    let nfa = converter(&map);
    let dfa = dfa(nfa, &map);
    //println!("{}", Dot::with_config(&dfa, &[]))
}
