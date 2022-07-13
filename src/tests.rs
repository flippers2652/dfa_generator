#[cfg(test)]
#[test]
fn regular_expressions() {
    use crate::regular_expression::RegularExpression;
    use crate::regular_expression::RegularExpression::*;

    let re = Character('a');
    let re = re.kleene_star();
    let re = re.alternate(&RegularExpression::literal("b"));
    let re = re.alternate(&Empty);
    let re = RegularExpression::literal("c").concatenate(&re);

    assert_eq!(format!("{}", re), "c(a*|b|ε)");

    assert_eq!(format!("{}", RegularExpression::literal("Hello")), "Hello");
    assert_eq!(format!("{}", RegularExpression::literal("")), "ε");
}
#[test]
fn re_to_nfa() {
    use crate::re_to_nfa::converter;
    use crate::regular_expression::RegularExpression;
    use crate::regular_expression::RegularExpression::*;
    use petgraph::dot::Dot;

    let re = Character('a');
    let re = re.kleene_star();
    let re = re.alternate(&RegularExpression::literal("b"));
    let re = re.alternate(&Empty);
    let re = RegularExpression::literal("c").concatenate(&re);

    //println!("{:?}", Dot::with_config(&converter(re), &[]));
}

#[test]
fn nfa_to_dfa() {
    use crate::nfa_to_dfa::converter as dfa;
    use crate::re_to_nfa::converter;
    use crate::regular_expression::RegularExpression::*;
    use petgraph::dot::Dot;
    let re = Character('a');
    let left = Character('a').alternate(&Character('b')).kleene_star();
    let right = Character('b').alternate(&Character('c')).kleene_star();
    let mid = left.alternate(&right);
    let re = re.concatenate(&mid).concatenate(&Character('c'));
    println!("{}",re);
    let nfa = converter(re);
    println!("{:?}", Dot::with_config(&dfa(nfa), &[]));
}
