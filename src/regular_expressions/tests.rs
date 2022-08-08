#[cfg(test)]
#[test]
fn regular_expressions_general() {
    use crate::regular_expressions::RegularExpression;
    use crate::regular_expressions::RegularExpression::*;
    use crate::regular_expressions::Alphanumeric;

    let re = Character('a');
    let re = re.concatenate(&re.kleene_star());
    let re = re.alternate(&RegularExpression::literal("b"));
    let re = re.alternate(&Empty);
    let re = RegularExpression::<char>::literal("c").concatenate(&re);

    assert_eq!(format!("{}", re), "c(aa*|b|ε)");
}

#[test]
fn literal() {
    use crate::regular_expressions::Alphanumeric;

    use crate::regular_expressions::RegularExpression;
    assert_eq!(
        format!("{}", RegularExpression::literal("Hello World")),
        "Hello World"
    );
}

#[test]
fn empty() {
    use crate::regular_expressions::Alphanumeric;

    use crate::regular_expressions::RegularExpression;
    assert_eq!(format!("{}", RegularExpression::literal("")), "ε");
}
#[test]
fn first() {
    use crate::regular_expressions::Alphanumeric;
     use crate::regular_expressions::RegularExpression;
    use crate::regular_expressions::RegularExpression::*;

    let re = Character('a');
    let re = re.kleene_star();
    let re = re.alternate(&RegularExpression::literal("b"));

    assert_eq!(re.first(),std::collections::HashSet::from([Some('a'),Some('b'),None]))
}
