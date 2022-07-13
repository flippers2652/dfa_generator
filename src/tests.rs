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
fn re_to_nfa(){
    use petgraph::dot::{Dot, Config};
    use crate::regular_expression::RegularExpression;
    use crate::regular_expression::RegularExpression::*;
    use crate::re_to_nfa::converter;

    let re = Character('a');
    let re = re.kleene_star();
    let re = re.alternate(&RegularExpression::literal("b"));
    let re = re.alternate(&Empty);
    let re = RegularExpression::literal("c").concatenate(&re);


    println!("{:?}", Dot::with_config(&converter(re), &[]));
}
