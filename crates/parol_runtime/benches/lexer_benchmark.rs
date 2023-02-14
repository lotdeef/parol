use std::{borrow::Cow, cell::RefCell, path::Path};

use criterion::{criterion_group, criterion_main, Criterion};
use parol_runtime::{once_cell::sync::Lazy, TokenStream, Tokenizer};

const LEXER_INPUT: &str = include_str!("./input_1.txt");

// The regex generated by parol for `verly` grammar
const PATTERNS: &[&str] = &[
    parol_runtime::lexer::tokenizer::UNMATCHABLE_TOKEN,
    parol_runtime::lexer::tokenizer::UNMATCHABLE_TOKEN,
    parol_runtime::lexer::tokenizer::UNMATCHABLE_TOKEN,
    parol_runtime::lexer::tokenizer::UNMATCHABLE_TOKEN,
    parol_runtime::lexer::tokenizer::UNMATCHABLE_TOKEN,
    r"(?:(?:(?://.*(?:\r\n|\r|\n|$))|(?:(?ms)/\u{2a}.*?\u{2a}/))\s*)+",
    r"[0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*[eE][+-]?[0-9]+(?:_[0-9]+)*",
    r"[0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*",
    r"[0-9]+(?:_[0-9]+)*'[bodh][0-9a-fA-FxzXZ]+(?:_[0-9a-fA-FxzXZ]+)*",
    r"[0-9]+(?:_[0-9]+)*",
    r"'[01xzXZ]",
    r"\-:",
    r"\->",
    r"\+:",
    r"\+=|-=|\*=|/=|%=|&=|\|=|\^=|<<=|>>=|<<<=|>>>=",
    r"\*\*",
    r"/|%",
    r"\+|-",
    r"<<<|>>>|<<|>>",
    r"<=|>=|<|>",
    r"===|==\?|!==|!=\?|==|!=",
    r"&&",
    r"\|\|",
    r"&",
    r"\^~|\^|~\^",
    r"\|",
    r"~&|~\||!|~",
    r"::",
    r":",
    r",",
    r"\$",
    r"\.\.",
    r"\.",
    r"=",
    r"\#",
    r"\{",
    r"\[",
    r"\(",
    r"\}",
    r"\]",
    r"\)",
    r";",
    r"\*",
    r"(?-u:\b)always_comb(?-u:\b)",
    r"(?-u:\b)always_ff(?-u:\b)",
    r"(?-u:\b)assign(?-u:\b)",
    r"(?-u:\b)async_high(?-u:\b)",
    r"(?-u:\b)async_low(?-u:\b)",
    r"(?-u:\b)as(?-u:\b)",
    r"(?-u:\b)bit(?-u:\b)",
    r"(?-u:\b)case(?-u:\b)",
    r"(?-u:\b)default(?-u:\b)",
    r"(?-u:\b)else(?-u:\b)",
    r"(?-u:\b)enum(?-u:\b)",
    r"(?-u:\b)export(?-u:\b)",
    r"(?-u:\b)f32(?-u:\b)",
    r"(?-u:\b)f64(?-u:\b)",
    r"(?-u:\b)for(?-u:\b)",
    r"(?-u:\b)function(?-u:\b)",
    r"(?-u:\b)i32(?-u:\b)",
    r"(?-u:\b)i64(?-u:\b)",
    r"(?-u:\b)if_reset(?-u:\b)",
    r"(?-u:\b)if(?-u:\b)",
    r"(?-u:\b)import(?-u:\b)",
    r"(?-u:\b)inout(?-u:\b)",
    r"(?-u:\b)input(?-u:\b)",
    r"(?-u:\b)inst(?-u:\b)",
    r"(?-u:\b)interface(?-u:\b)",
    r"(?-u:\b)in(?-u:\b)",
    r"(?-u:\b)localparam(?-u:\b)",
    r"(?-u:\b)logic(?-u:\b)",
    r"(?-u:\b)modport(?-u:\b)",
    r"(?-u:\b)module(?-u:\b)",
    r"(?-u:\b)negedge(?-u:\b)",
    r"(?-u:\b)output(?-u:\b)",
    r"(?-u:\b)package(?-u:\b)",
    r"(?-u:\b)parameter(?-u:\b)",
    r"(?-u:\b)posedge(?-u:\b)",
    r"(?-u:\b)ref(?-u:\b)",
    r"(?-u:\b)repeat(?-u:\b)",
    r"(?-u:\b)return(?-u:\b)",
    r"(?-u:\b)step(?-u:\b)",
    r"(?-u:\b)struct(?-u:\b)",
    r"(?-u:\b)sync_high(?-u:\b)",
    r"(?-u:\b)sync_low(?-u:\b)",
    r"(?-u:\b)tri(?-u:\b)",
    r"(?-u:\b)u32(?-u:\b)",
    r"(?-u:\b)u64(?-u:\b)",
    r"(?-u:\b)var(?-u:\b)",
    r"[a-zA-Z_][0-9a-zA-Z_]*",
    r".",
];

const SCANNER_SPECIFICS: &[&str] = &[
    /*  0 */ parol_runtime::lexer::tokenizer::UNMATCHABLE_TOKEN,
    /*  1 */ parol_runtime::lexer::tokenizer::NEW_LINE_TOKEN,
    /*  2 */ parol_runtime::lexer::tokenizer::WHITESPACE_TOKEN,
    /*  3 */ parol_runtime::lexer::tokenizer::UNMATCHABLE_TOKEN,
    /*  4 */ parol_runtime::lexer::tokenizer::UNMATCHABLE_TOKEN,
];

const SCANNER_TERMINAL_INDICES: &[usize] = &[
    5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53,
    54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77,
    78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
];

const MAX_K: usize = 3;
const ERROR_TOKEN_INDEX: usize = 90;

static TOKENIZERS_1: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![(
        "INITIAL",
        Tokenizer::build(PATTERNS, SCANNER_SPECIFICS, SCANNER_TERMINAL_INDICES).unwrap(),
    )]
});

fn tokenize_1() {
    let file_name: Cow<Path> = Path::new("./input_1.txt").to_owned().into();
    let token_stream =
        RefCell::new(TokenStream::new(LEXER_INPUT, file_name, &TOKENIZERS_1, MAX_K).unwrap());
    while !token_stream.borrow().all_input_consumed() {
        let tok = token_stream.borrow_mut().lookahead(0).unwrap();
        assert_ne!(tok.token_type, ERROR_TOKEN_INDEX);
        token_stream.borrow_mut().consume().unwrap();
    }
}

fn regex_1_benchmark(c: &mut Criterion) {
    c.bench_function("tokenize_1", |b| b.iter(|| tokenize_1()));
}

criterion_group!(benches, regex_1_benchmark,);
criterion_main!(benches);
