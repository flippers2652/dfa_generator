use crate::re_to_nfa::TokenRequirements;
use std::collections::HashMap;
#[derive(Copy, PartialEq, Clone)]
pub enum State<Token: TokenRequirements> {
    Standard,
    Token(Token),
    Error,
}
pub struct Table<Token: TokenRequirements> {
    pub start_id: usize,
    pub table: Vec<Vec<usize>>,
    pub states: Vec<State<Token>>,
    pub current_id: usize,
    pub alphabet: HashMap<char, usize>,
}
impl<Token: TokenRequirements> Table<Token> {
    fn next(&mut self, c: char) -> State<Token> {
        self.current_id = self.table[self.current_id][self.alphabet[&c]];
        return self.states[self.current_id];
    }
    fn reset(&mut self) {
        self.current_id = self.start_id;
    }
    fn peek(&self) -> State<Token> {
        return self.states[self.current_id];
    }
}
