mod tests;

use std::fmt;

#[derive(Clone, Debug,PartialEq)]
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
    pub fn upper_alpha() -> Self {
        let mut out=RegularExpression::Empty;
        for c in b'A'..b'Z'{
            out=out.alternate(&Self::Character(c as char));
        }
        return out;
    }
    pub fn lower_alpha() -> Self {
        let mut out=RegularExpression::Empty;
        for c in b'a'..b'z'{
            out=out.alternate(&Self::Character(c as char))
        }
        return out;
    }
    pub fn alpha() -> Self {
        return Self::upper_alpha().alternate(&Self::lower_alpha());
    }
    pub fn numeric() -> Self {
        let mut out=RegularExpression::Empty;
        for c in b'0'..b'9'{
            out=out.alternate(&Self::Character(c as char))
        }
        return out;
    }
    pub fn alphanumeric() -> Self {
        return Self::alpha().alternate(&Self::numeric());
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
    if *self==RegularExpression::Empty{
        return other.clone();
    }
    if *other==RegularExpression::Empty{
        return self.clone();
    }
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
