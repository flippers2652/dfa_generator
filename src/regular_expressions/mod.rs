mod tests;
use std::collections::HashSet;

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum RegularExpression<Alphabet: Clone+std::hash::Hash+std::cmp::Eq+std::fmt::Debug> {
    Character(Alphabet),
    Alternative(Vec<RegularExpression<Alphabet>>),
    Concatenation(Vec<RegularExpression<Alphabet>>),
    KleeneStar(Box<RegularExpression<Alphabet>>),
    Empty,
}

impl<Alphabet: std::clone::Clone+std::hash::Hash+std::cmp::Eq+std::fmt::Debug> RegularExpression<Alphabet> {
    fn precedence(&self) -> i32 {
        match self {
            RegularExpression::Character(_) => 5,
            RegularExpression::Empty => 5,
            RegularExpression::Alternative(_) => 1,
            RegularExpression::Concatenation(_) => 2,
            RegularExpression::KleeneStar(_) => 3,
        }
    }
    pub fn kleene_star(&self) -> Self {
        return RegularExpression::KleeneStar(Box::new(self.clone()));
    }
    pub fn concatenate(&self, other: &Self) -> Self {
        match (self, other) {
            (RegularExpression::Empty, _) => other.clone(),
            (_, RegularExpression::Empty) => self.clone(),
            (
                RegularExpression::Concatenation(inner_self),
                RegularExpression::Concatenation(inner_other),
            ) => {
                let mut temp_vec = inner_self.clone();
                temp_vec.append(&mut inner_other.clone());
                return RegularExpression::Concatenation(temp_vec);
            }
            (RegularExpression::Concatenation(inner_self), other) => {
                let mut temp_vec = inner_self.clone();
                temp_vec.push(other.clone());
                return RegularExpression::Concatenation(temp_vec);
            }
            (s, RegularExpression::Concatenation(inner_other)) => {
                let mut temp_vec = inner_other.clone();
                temp_vec.push(s.clone());
                temp_vec.append(&mut inner_other.clone());
                return RegularExpression::Concatenation(temp_vec);
            }
            _ => RegularExpression::Concatenation(vec![
                self.clone(),
                other.clone(),
            ]),
        }
    }
    pub fn alternate(&self, other: &Self) -> Self {
        match (self, other) {
            (
                RegularExpression::Alternative(inner_self),
                RegularExpression::Alternative(inner_other),
            ) => {
                let mut temp_vec = inner_self.clone();
                temp_vec.append(&mut inner_other.clone());
                return RegularExpression::Alternative(temp_vec);
            }
            (RegularExpression::Alternative(inner_self), other) => {
                let mut temp_vec = inner_self.clone();
                temp_vec.push(other.clone());
                return RegularExpression::Alternative(temp_vec);
            }
            (s, RegularExpression::Alternative(inner_other)) => {
                let mut temp_vec = inner_other.clone();
                temp_vec.push(s.clone());
                temp_vec.append(&mut inner_other.clone());
                return RegularExpression::Alternative(temp_vec);
            }
            _ => RegularExpression::Alternative(vec![
                self.clone(),
                other.clone(),
            ]),
        }
    }
    pub fn first(&self) -> HashSet<Option<Alphabet>> {
        return self.first_with_nullable(&[].into_iter().collect());
    }
    pub fn first_with_nullable(&self,nullables:&HashSet<Alphabet>) -> HashSet<Option<Alphabet>> {
        match self {
            RegularExpression::Alternative(vec) => {
                let mut set=HashSet::<Option<Alphabet>>::new();
                for re in vec{
                    let inner_set=re.first_with_nullable(&nullables);
                    set=set.union(&inner_set).map(|s| s.clone()).collect();
                }
                return set;
            },
            RegularExpression::Concatenation(vec) => {
                let mut set=HashSet::<Option<Alphabet>>::new();
                let mut broke=false;
                for re in vec{
                    let inner_set=re.first_with_nullable(&nullables);
                    set=set.union(&inner_set).map(|s| s.clone()).collect();
                    let mut is_nullable=inner_set.contains(&None);
                    //println!("Nullables:");
                    for nullable in nullables{
                        //println!("\t{:?}",*nullable);
                        is_nullable|=inner_set.contains(&Some(nullable.clone()));
                    }
                    if !is_nullable{
                        broke=true;
                        break;
                    } else {
                        set.remove(&None);
                    }
                }
                if !broke{
                    set.insert(None);
                }
                return set;
            },
            RegularExpression::Empty => {
                return vec![None].into_iter().collect();
            },
            RegularExpression::KleeneStar(inner_self) => {
                let mut set=inner_self.first_with_nullable(&nullables);
                set.insert(None);
                return set;
            }
            RegularExpression::Character(c) => {
                return vec![Some(c.clone())].into_iter().collect();
            }
        }
    }
}
impl Alphanumeric for RegularExpression<char> {
    fn literal(string: &str) -> Self {
        let mut literal = match string.chars().next() {
            Some(s) => Self::Character(s),
            None => return Self::Empty,
        };
        for c in string[1..].chars() {
            literal = literal.concatenate(&Self::Character(c));
        }
        return literal;
    }
    fn upper_alpha() -> Self {
        let mut out = Self::Character('A');
        for c in b'B'..=b'Z' {
            out = out.alternate(&Self::Character(c as char));
        }
        return out;
    }
    fn lower_alpha() -> Self {
        let mut out = Self::Character('a');
        for c in b'a'..=b'z' {
            out = out.alternate(&Self::Character(c as char))
        }
        return out;
    }
    fn alpha() -> Self {
        return Self::upper_alpha().alternate(&Self::lower_alpha());
    }
    fn numeric() -> Self {
        let mut out = Self::Character('0');
        for c in b'1'..=b'9' {
            out = out.alternate(&Self::Character(c as char))
        }
        return out;
    }
    fn alphanumeric() -> Self {
        return Self::alpha().alternate(&Self::numeric());
    }
}
pub trait Alphanumeric {
    fn literal(string: &str) -> Self;
    fn upper_alpha() -> Self;
    fn lower_alpha() -> Self;
    fn alpha() -> Self;
    fn numeric() -> Self;
    fn alphanumeric() -> Self;
}

impl<Alphabet: fmt::Display + std::clone::Clone+std::hash::Hash+std::cmp::Eq+Copy+std::fmt::Debug> fmt::Display for RegularExpression<Alphabet> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegularExpression::Character(c) => f.write_fmt(format_args!("{}", c)),
            RegularExpression::Alternative(vec) => {
                for item in vec.iter().take(vec.len() - 1) {
                    if item.precedence() >= 1 {
                        f.write_fmt(format_args!("{}", item))?;
                    } else {
                        f.write_fmt(format_args!("({})", item))?;
                    }
                    f.write_fmt(format_args!("|"))?;
                }
                if vec.last().unwrap().precedence() >= 1 {
                    f.write_fmt(format_args!("{}", vec.last().unwrap()))?;
                } else {
                    f.write_fmt(format_args!("({})", vec.last().unwrap()))?;
                }
                return Ok(());
            }
            RegularExpression::Concatenation(vec) => {
                for item in vec {
                    if item.precedence() >= 2 {
                        f.write_fmt(format_args!("{}", item))?;
                    } else {
                        f.write_fmt(format_args!("({})", item))?;
                    }
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
