#[test]
fn re_to_nfa() {
    use crate::re_to_nfa::converter;
    use crate::regular_expressions::RegularExpression;
    use crate::regular_expressions::RegularExpression::*;
    use petgraph::dot::Dot;
    let re = Character('a');
    let re = re.kleene_star();
    let re = re.alternate(&RegularExpression::literal("b"));
    let re = re.alternate(&Empty);
    let re = RegularExpression::literal("c").concatenate(&re);
    let mut map = Vec::<(&str, RegularExpression)>::new();
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
    7 [ label = \"\" ]
    0 -> 1 [ label = \"c\" ]
    4 -> 5 [ label = \"a\" ]
    5 -> 4 [ label = \"ε\" ]
    1 -> 4 [ label = \"ε\" ]
    5 -> 3 [ label = \"ε\" ]
    1 -> 6 [ label = \"b\" ]
    6 -> 3 [ label = \"ε\" ]
    3 -> 2 [ label = \"ε\" ]
    1 -> 7 [ label = \"ε\" ]
    7 -> 2 [ label = \"ε\" ]
}
"
    );
}
