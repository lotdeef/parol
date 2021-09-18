use crate::errors::*;
use crate::generators::{generate_terminal_name, GrammarConfig};

use crate::StrVec;
use std::fmt::Debug;

#[derive(BartDisplay, Debug, Default)]
#[template = "templates/lexer_template.rs"]
struct LexerData {
    augmented_terminals: StrVec,
    terminal_names: StrVec,
    terminal_count: usize,
    lookahead_size: usize,
}

pub fn generate_lexer_source(grammar_config: &GrammarConfig) -> Result<String> {
    let original_augmented_terminals = grammar_config.generate_augmented_terminals();

    let terminal_count = original_augmented_terminals.len();
    let width = (terminal_count as f32).log10() as usize + 1;

    let augmented_terminals =
        original_augmented_terminals
            .iter()
            .enumerate()
            .fold(StrVec::new(4), |mut acc, (i, e)| {
                let e = match e.as_str() {
                    "UNMATCHABLE_TOKEN" | "NEW_LINE_TOKEN" | "WHITESPACE_TOKEN" | "ERROR_TOKEN" => {
                        e.to_owned()
                    }
                    _ => format!(r####"r###"{}"###"####, e),
                };
                acc.push(format!("/* {:w$} */ {},", i, e, w = width));
                acc
            });

    let terminal_names =
        original_augmented_terminals
            .iter()
            .enumerate()
            .fold(StrVec::new(4), |mut acc, (i, e)| {
                let n = generate_terminal_name(e, i, &grammar_config.cfg);
                acc.push(format!(r#"/* {:w$} */ "{}","#, i, n, w = width));
                acc
            });

    let lexer_data = LexerData {
        augmented_terminals,
        terminal_names,
        terminal_count,
        lookahead_size: grammar_config.lookahead_size,
    };

    Ok(format!("{}", lexer_data))
}
