#[cfg(test)]
#[test]
fn re_to_nfa() {
    use crate::re_to_nfa::converter;
    use crate::regular_expressions::RegularExpression;
    use crate::regular_expressions::RegularExpression::*;
    use crate::regular_expressions::Alphanumeric;
    use petgraph::dot::Dot;
    let re = Character('a');
    let re = re.kleene_star();
    let re = re.alternate(&RegularExpression::literal("b"));
    let re = re.alternate(&Empty);
    let re = RegularExpression::literal("c").concatenate(&re);
    let mut map = Vec::<(&str, RegularExpression<_>)>::new();
    map.push(("Hello", re));
    assert_eq!(
        format!("{}", Dot::with_config(&converter(&map), &[])),
        "digraph {
    0 [ label = \"Start\" ]
    1 [ label = \"\" ]
    2 [ label = \"End(Hello)\" ]
    3 [ label = \"\" ]
    4 [ label = \"\" ]
    5 [ label = \"\" ]
    6 [ label = \"\" ]
    0 -> 1 [ label = \"c\" ]
    3 -> 4 [ label = \"a\" ]
    4 -> 1 [ label = \"ε\" ]
    1 -> 3 [ label = \"ε\" ]
    3 -> 2 [ label = \"ε\" ]
    1 -> 5 [ label = \"b\" ]
    5 -> 2 [ label = \"ε\" ]
    1 -> 6 [ label = \"ε\" ]
    6 -> 2 [ label = \"ε\" ]
}
"
    );
}
