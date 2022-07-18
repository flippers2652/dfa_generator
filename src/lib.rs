pub mod table;
mod dfa_minimiser;
mod dfa_to_table;
mod nfa_to_dfa;
mod re_to_nfa;
pub mod regular_expressions;

use re_to_nfa::TokenRequirements;

mod tests;

pub fn regular_expression_to_table<Token: TokenRequirements>(
    tokens: Vec<(Token, regular_expressions::RegularExpression)>,
) -> table::Table<Token> {
    let nfa = re_to_nfa::converter(&tokens);
    let dfa = nfa_to_dfa::converter(nfa, &tokens);
    let min = dfa_minimiser::minimise(dfa);
    let table = dfa_to_table::converter(min);
    return table;
}
