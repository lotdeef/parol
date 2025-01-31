// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{
    LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans, UserActionsTrait,
};
use parol_runtime::{ParolError, ParseTree};
use parol_runtime::{TokenStream, Tokenizer};
use std::cell::RefCell;
use std::path::Path;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 18] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r###"(?i)AND"###,
    /*  6 */ r###"(?i)OR"###,
    /*  7 */ r###"(?i)XOR"###,
    /*  8 */ r###"(?i)NOR"###,
    /*  9 */ r###"(?i)NAND"###,
    /* 10 */ r###"(?i)XNOR"###,
    /* 11 */ r###"(?i)TRUE"###,
    /* 12 */ r###"(?i)FALSE"###,
    /* 13 */ r###"(?i)NOT"###,
    /* 14 */ r###";"###,
    /* 15 */ r###"\("###,
    /* 16 */ r###"\)"###,
    /* 17 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 18] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "AndOp",
    /*  6 */ "OrOp",
    /*  7 */ "XorOp",
    /*  8 */ "NorOp",
    /*  9 */ "NandOp",
    /* 10 */ "XnorOp",
    /* 11 */ "True",
    /* 12 */ "False",
    /* 13 */ "Not",
    /* 14 */ "Semicolon",
    /* 15 */ "LeftParenthesis",
    /* 16 */ "RightParenthesis",
    /* 17 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[usize; 12]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r###"(//.*(\r\n|\r|\n|$))"###,
        /*  4 */ r###"((?ms)\(\*.*?\*\))"###,
    ],
    &[
        5,  /* AndOp */
        6,  /* OrOp */
        7,  /* XorOp */
        8,  /* NorOp */
        9,  /* NandOp */
        10, /* XnorOp */
        11, /* True */
        12, /* False */
        13, /* Not */
        14, /* Semicolon */
        15, /* LeftParenthesis */
        16, /* RightParenthesis */
    ],
);

const MAX_K: usize = 2;

pub const NON_TERMINALS: &[&str; 25] = &[
    /*  0 */ "AndOp",
    /*  1 */ "BinaryOperator",
    /*  2 */ "Boolean",
    /*  3 */ "Expression",
    /*  4 */ "Expressions",
    /*  5 */ "ExpressionsList",
    /*  6 */ "ExpressionsOpt",
    /*  7 */ "Factor",
    /*  8 */ "False",
    /*  9 */ "LeftParenthesis",
    /* 10 */ "NandOp",
    /* 11 */ "NorOp",
    /* 12 */ "Not",
    /* 13 */ "OrOp",
    /* 14 */ "Parenthesized",
    /* 15 */ "RightParenthesis",
    /* 16 */ "Semicolon",
    /* 17 */ "TailExpression",
    /* 18 */ "TailExpressionList",
    /* 19 */ "Term",
    /* 20 */ "TermOpt",
    /* 21 */ "True",
    /* 22 */ "UnaryOperator",
    /* 23 */ "XnorOp",
    /* 24 */ "XorOp",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 25] = &[
    /* 0 - "AndOp" */
    LookaheadDFA {
        prod0: 21,
        transitions: &[],
        k: 0,
    },
    /* 1 - "BinaryOperator" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 1, 15),
            Trans(0, 6, 2, 16),
            Trans(0, 7, 3, 17),
            Trans(0, 8, 4, 18),
            Trans(0, 9, 5, 19),
            Trans(0, 10, 6, 20),
        ],
        k: 1,
    },
    /* 2 - "Boolean" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 11, 1, 12), Trans(0, 12, 2, 13)],
        k: 1,
    },
    /* 3 - "Expression" */
    LookaheadDFA {
        prod0: 5,
        transitions: &[],
        k: 0,
    },
    /* 4 - "Expressions" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 5 - "ExpressionsList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 3, 2),
            Trans(0, 14, 1, -1),
            Trans(1, 0, 3, 2),
            Trans(1, 11, 2, 1),
            Trans(1, 12, 2, 1),
            Trans(1, 13, 2, 1),
            Trans(1, 15, 2, 1),
        ],
        k: 2,
    },
    /* 6 - "ExpressionsOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 0, 2, 4), Trans(0, 14, 1, 3)],
        k: 1,
    },
    /* 7 - "Factor" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 11, 1, 34),
            Trans(0, 12, 1, 34),
            Trans(0, 15, 2, 35),
        ],
        k: 1,
    },
    /* 8 - "False" */
    LookaheadDFA {
        prod0: 28,
        transitions: &[],
        k: 0,
    },
    /* 9 - "LeftParenthesis" */
    LookaheadDFA {
        prod0: 32,
        transitions: &[],
        k: 0,
    },
    /* 10 - "NandOp" */
    LookaheadDFA {
        prod0: 25,
        transitions: &[],
        k: 0,
    },
    /* 11 - "NorOp" */
    LookaheadDFA {
        prod0: 24,
        transitions: &[],
        k: 0,
    },
    /* 12 - "Not" */
    LookaheadDFA {
        prod0: 29,
        transitions: &[],
        k: 0,
    },
    /* 13 - "OrOp" */
    LookaheadDFA {
        prod0: 22,
        transitions: &[],
        k: 0,
    },
    /* 14 - "Parenthesized" */
    LookaheadDFA {
        prod0: 30,
        transitions: &[],
        k: 0,
    },
    /* 15 - "RightParenthesis" */
    LookaheadDFA {
        prod0: 33,
        transitions: &[],
        k: 0,
    },
    /* 16 - "Semicolon" */
    LookaheadDFA {
        prod0: 31,
        transitions: &[],
        k: 0,
    },
    /* 17 - "TailExpression" */
    LookaheadDFA {
        prod0: 6,
        transitions: &[],
        k: 0,
    },
    /* 18 - "TailExpressionList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 8),
            Trans(0, 5, 1, 7),
            Trans(0, 6, 1, 7),
            Trans(0, 7, 1, 7),
            Trans(0, 8, 1, 7),
            Trans(0, 9, 1, 7),
            Trans(0, 10, 1, 7),
            Trans(0, 14, 2, 8),
            Trans(0, 16, 2, 8),
        ],
        k: 1,
    },
    /* 19 - "Term" */
    LookaheadDFA {
        prod0: 9,
        transitions: &[],
        k: 0,
    },
    /* 20 - "TermOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 11, 2, 11),
            Trans(0, 12, 2, 11),
            Trans(0, 13, 1, 10),
            Trans(0, 15, 2, 11),
        ],
        k: 1,
    },
    /* 21 - "True" */
    LookaheadDFA {
        prod0: 27,
        transitions: &[],
        k: 0,
    },
    /* 22 - "UnaryOperator" */
    LookaheadDFA {
        prod0: 14,
        transitions: &[],
        k: 0,
    },
    /* 23 - "XnorOp" */
    LookaheadDFA {
        prod0: 26,
        transitions: &[],
        k: 0,
    },
    /* 24 - "XorOp" */
    LookaheadDFA {
        prod0: 23,
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 36] = &[
    // 0 - Expressions: Expression ExpressionsList /* Vec */ ExpressionsOpt /* Option */;
    Production {
        lhs: 4,
        production: &[ParseType::N(6), ParseType::N(5), ParseType::N(3)],
    },
    // 1 - ExpressionsList: Semicolon Expression ExpressionsList;
    Production {
        lhs: 5,
        production: &[ParseType::N(5), ParseType::N(3), ParseType::N(16)],
    },
    // 2 - ExpressionsList: ;
    Production {
        lhs: 5,
        production: &[],
    },
    // 3 - ExpressionsOpt: Semicolon;
    Production {
        lhs: 6,
        production: &[ParseType::N(16)],
    },
    // 4 - ExpressionsOpt: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 5 - Expression: Term TailExpression;
    Production {
        lhs: 3,
        production: &[ParseType::N(17), ParseType::N(19)],
    },
    // 6 - TailExpression: TailExpressionList /* Vec */;
    Production {
        lhs: 17,
        production: &[ParseType::N(18)],
    },
    // 7 - TailExpressionList: BinaryOperator Term TailExpressionList;
    Production {
        lhs: 18,
        production: &[ParseType::N(18), ParseType::N(19), ParseType::N(1)],
    },
    // 8 - TailExpressionList: ;
    Production {
        lhs: 18,
        production: &[],
    },
    // 9 - Term: TermOpt /* Option */ Factor;
    Production {
        lhs: 19,
        production: &[ParseType::N(7), ParseType::N(20)],
    },
    // 10 - TermOpt: UnaryOperator;
    Production {
        lhs: 20,
        production: &[ParseType::N(22)],
    },
    // 11 - TermOpt: ;
    Production {
        lhs: 20,
        production: &[],
    },
    // 12 - Boolean: True;
    Production {
        lhs: 2,
        production: &[ParseType::N(21)],
    },
    // 13 - Boolean: False;
    Production {
        lhs: 2,
        production: &[ParseType::N(8)],
    },
    // 14 - UnaryOperator: Not;
    Production {
        lhs: 22,
        production: &[ParseType::N(12)],
    },
    // 15 - BinaryOperator: AndOp;
    Production {
        lhs: 1,
        production: &[ParseType::N(0)],
    },
    // 16 - BinaryOperator: OrOp;
    Production {
        lhs: 1,
        production: &[ParseType::N(13)],
    },
    // 17 - BinaryOperator: XorOp;
    Production {
        lhs: 1,
        production: &[ParseType::N(24)],
    },
    // 18 - BinaryOperator: NorOp;
    Production {
        lhs: 1,
        production: &[ParseType::N(11)],
    },
    // 19 - BinaryOperator: NandOp;
    Production {
        lhs: 1,
        production: &[ParseType::N(10)],
    },
    // 20 - BinaryOperator: XnorOp;
    Production {
        lhs: 1,
        production: &[ParseType::N(23)],
    },
    // 21 - AndOp: "(?i)AND";
    Production {
        lhs: 0,
        production: &[ParseType::T(5)],
    },
    // 22 - OrOp: "(?i)OR";
    Production {
        lhs: 13,
        production: &[ParseType::T(6)],
    },
    // 23 - XorOp: "(?i)XOR";
    Production {
        lhs: 24,
        production: &[ParseType::T(7)],
    },
    // 24 - NorOp: "(?i)NOR";
    Production {
        lhs: 11,
        production: &[ParseType::T(8)],
    },
    // 25 - NandOp: "(?i)NAND";
    Production {
        lhs: 10,
        production: &[ParseType::T(9)],
    },
    // 26 - XnorOp: "(?i)XNOR";
    Production {
        lhs: 23,
        production: &[ParseType::T(10)],
    },
    // 27 - True: "(?i)TRUE";
    Production {
        lhs: 21,
        production: &[ParseType::T(11)],
    },
    // 28 - False: "(?i)FALSE";
    Production {
        lhs: 8,
        production: &[ParseType::T(12)],
    },
    // 29 - Not: "(?i)NOT";
    Production {
        lhs: 12,
        production: &[ParseType::T(13)],
    },
    // 30 - Parenthesized: LeftParenthesis Expression RightParenthesis;
    Production {
        lhs: 14,
        production: &[ParseType::N(15), ParseType::N(3), ParseType::N(9)],
    },
    // 31 - Semicolon: ";";
    Production {
        lhs: 16,
        production: &[ParseType::T(14)],
    },
    // 32 - LeftParenthesis: "\(";
    Production {
        lhs: 9,
        production: &[ParseType::T(15)],
    },
    // 33 - RightParenthesis: "\)";
    Production {
        lhs: 15,
        production: &[ParseType::T(16)],
    },
    // 34 - Factor: Boolean;
    Production {
        lhs: 7,
        production: &[ParseType::N(2)],
    },
    // 35 - Factor: Parenthesized;
    Production {
        lhs: 7,
        production: &[ParseType::N(14)],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![(
        "INITIAL",
        Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
    )]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut dyn UserActionsTrait<'t>,
) -> Result<ParseTree<'t>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        4,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    let token_stream =
        RefCell::new(TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap());
    llk_parser.parse(token_stream, user_actions)
}
