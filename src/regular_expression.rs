use std::fmt;
#[derive(Clone, Debug)]
pub enum RegularExpression {
    Character(char),
    Alternative(Box<RegularExpression>, Box<RegularExpression>),
    Concatenation(Box<RegularExpression>, Box<RegularExpression>),
    KleeneStar(Box<RegularExpression>),
    Empty,
}

impl RegularExpression {
    pub fn literal(string: &str) -> Self {
        let mut literal = match string.chars().next() {
            Some(s) => RegularExpression::Character(s),
            None => return RegularExpression::Empty,
        };
        for char in string[1..].chars() {
            literal = RegularExpression::Concatenation(
                Box::new(literal),
                Box::new(RegularExpression::Character(char)),
            )
        }
        return literal;
    }
    fn precedence(&self) -> i32 {
        match self {
            RegularExpression::Character(_) => 5,
            RegularExpression::Empty => 5,
            RegularExpression::Alternative(_, _) => 1,
            RegularExpression::Concatenation(_, _) => 2,
            RegularExpression::KleeneStar(_) => 3,
        }
    }
    pub fn kleene_star(&self) -> Self {
        return RegularExpression::KleeneStar(Box::new(self.clone()));
    }
    pub fn concatenate(&self, other: &Self) -> Self {
        return RegularExpression::Concatenation(Box::new(self.clone()), Box::new(other.clone()));
    }
    pub fn alternate(&self, other: &Self) -> Self {
        return RegularExpression::Alternative(Box::new(self.clone()), Box::new(other.clone()));
    }
}

impl fmt::Display for RegularExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegularExpression::Character(c) => f.write_fmt(format_args!("{}", c)),
            RegularExpression::Alternative(a, b) => {
                if a.precedence() >= 1 {
                    f.write_fmt(format_args!("{}", a))?;
                } else {
                    f.write_fmt(format_args!("({})", a))?;
                }
                f.write_fmt(format_args!("|"))?;
                if b.precedence() >= 1 {
                    f.write_fmt(format_args!("{}", b))?;
                } else {
                    f.write_fmt(format_args!("({})", b))?;
                }
                return Ok(());
            }
            RegularExpression::Concatenation(a, b) => {
                if a.precedence() >= 2 {
                    f.write_fmt(format_args!("{}", a))?;
                } else {
                    f.write_fmt(format_args!("({})", a))?;
                }
                if b.precedence() >= 2 {
                    f.write_fmt(format_args!("{}", b))?;
                } else {
                    f.write_fmt(format_args!("({})", b))?;
                }
                return Ok(());
            }
            RegularExpression::KleeneStar(a) => {
                if a.precedence() >= 3 {
                    f.write_fmt(format_args!("{}", a))?;
                } else {
                    f.write_fmt(format_args!("({})", a))?;
                }
                f.write_fmt(format_args!("*"))?;
                return Ok(());
            }
            RegularExpression::Empty => f.write_fmt(format_args!("ε")),
        }
    }
}
