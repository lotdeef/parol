// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use anyhow::Result;
use id_tree::Tree;
use parol_runtime::lexer::{TokenStream, Tokenizer};
use parol_runtime::parser::{
    DFATransition, LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, UserActionsTrait,
};
use std::cell::RefCell;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 32] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r###"%start"###,
    /*  6 */ r###"%title"###,
    /*  7 */ r###"%comment"###,
    /*  8 */ r###"%line_comment"###,
    /*  9 */ r###"%block_comment"###,
    /* 10 */ r###"%auto_newline_off"###,
    /* 11 */ r###"%auto_ws_off"###,
    /* 12 */ r###"%%"###,
    /* 13 */ r###":"###,
    /* 14 */ r###";"###,
    /* 15 */ r###"\|"###,
    /* 16 */ r###"<"###,
    /* 17 */ r###">"###,
    /* 18 */ r###"\("###,
    /* 19 */ r###"\)"###,
    /* 20 */ r###"\["###,
    /* 21 */ r###"\]"###,
    /* 22 */ r###"\{"###,
    /* 23 */ r###"\}"###,
    /* 24 */ r###"[a-zA-Z_]\w*"###,
    /* 25 */ r###"\u{0022}([^\\]|\\.)*?\u{0022}"###,
    /* 26 */ r###"%scanner"###,
    /* 27 */ r###","###,
    /* 28 */ r###"%sc"###,
    /* 29 */ r###"%push"###,
    /* 30 */ r###"%pop"###,
    /* 31 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 32] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "PercentStart",
    /*  6 */ "PercentTitle",
    /*  7 */ "PercentComment",
    /*  8 */ "PercentLineUnderscoreComment",
    /*  9 */ "PercentBlockUnderscoreComment",
    /* 10 */ "PercentAutoUnderscoreNewlineUnderscoreOff",
    /* 11 */ "PercentAutoUnderscoreWsUnderscoreOff",
    /* 12 */ "PercentPercent",
    /* 13 */ "Colon",
    /* 14 */ "Semicolon",
    /* 15 */ "Or",
    /* 16 */ "LT",
    /* 17 */ "GT",
    /* 18 */ "LParen",
    /* 19 */ "RParen",
    /* 20 */ "LBracket",
    /* 21 */ "RBracket",
    /* 22 */ "LBrace",
    /* 23 */ "RBrace",
    /* 24 */ "Identifier",
    /* 25 */ "String",
    /* 26 */ "PercentScanner",
    /* 27 */ "Comma",
    /* 28 */ "PercentSc",
    /* 29 */ "PercentPush",
    /* 30 */ "PercentPop",
    /* 31 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[usize; 26]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r###"(//.*(\r\n|\r|\n|$))"###,
        /*  4 */ r###"((?ms)/\*.*?\*/)"###,
    ],
    &[
        5,  /* PercentStart */
        6,  /* PercentTitle */
        7,  /* PercentComment */
        8,  /* PercentLineUnderscoreComment */
        9,  /* PercentBlockUnderscoreComment */
        10, /* PercentAutoUnderscoreNewlineUnderscoreOff */
        11, /* PercentAutoUnderscoreWsUnderscoreOff */
        12, /* PercentPercent */
        13, /* Colon */
        14, /* Semicolon */
        15, /* Or */
        16, /* LT */
        17, /* GT */
        18, /* LParen */
        19, /* RParen */
        20, /* LBracket */
        21, /* RBracket */
        22, /* LBrace */
        23, /* RBrace */
        24, /* Identifier */
        25, /* String */
        26, /* PercentScanner */
        27, /* Comma */
        28, /* PercentSc */
        29, /* PercentPush */
        30, /* PercentPop */
    ],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 29] = &[
    /*  0 */ "Alternation",
    /*  1 */ "AlternationList",
    /*  2 */ "Alternations",
    /*  3 */ "AlternationsList",
    /*  4 */ "Declaration",
    /*  5 */ "Declarations",
    /*  6 */ "Factor",
    /*  7 */ "GrammarDefinition",
    /*  8 */ "GrammarDefinitionList",
    /*  9 */ "Group",
    /* 10 */ "Identifier",
    /* 11 */ "Optional",
    /* 12 */ "Parol",
    /* 13 */ "Production",
    /* 14 */ "Prolog",
    /* 15 */ "Repeat",
    /* 16 */ "ScannerDirectives",
    /* 17 */ "ScannerNameOpt",
    /* 18 */ "ScannerState",
    /* 19 */ "ScannerStateList",
    /* 20 */ "ScannerStates",
    /* 21 */ "ScannerSwitch",
    /* 22 */ "SimpleToken",
    /* 23 */ "StartDeclaration",
    /* 24 */ "StateList",
    /* 25 */ "StateListRest",
    /* 26 */ "String",
    /* 27 */ "Symbol",
    /* 28 */ "TokenWithStates",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 29] = &[
    /* 0 - "Alternation" */
    LookaheadDFA {
        states: &[Some(21)],
        transitions: &[],
        k: 0,
    },
    /* 1 - "AlternationList" */
    LookaheadDFA {
        states: &[None, Some(22), Some(23)],
        transitions: &[
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 2),
            DFATransition(0, 16, 1),
            DFATransition(0, 18, 1),
            DFATransition(0, 19, 2),
            DFATransition(0, 20, 1),
            DFATransition(0, 21, 2),
            DFATransition(0, 22, 1),
            DFATransition(0, 23, 2),
            DFATransition(0, 24, 1),
            DFATransition(0, 25, 1),
            DFATransition(0, 28, 1),
            DFATransition(0, 29, 1),
            DFATransition(0, 30, 1),
        ],
        k: 1,
    },
    /* 2 - "Alternations" */
    LookaheadDFA {
        states: &[Some(18)],
        transitions: &[],
        k: 0,
    },
    /* 3 - "AlternationsList" */
    LookaheadDFA {
        states: &[None, Some(19), Some(20)],
        transitions: &[
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 1),
            DFATransition(0, 19, 2),
            DFATransition(0, 21, 2),
            DFATransition(0, 23, 2),
        ],
        k: 1,
    },
    /* 4 - "Declaration" */
    LookaheadDFA {
        states: &[None, Some(5), Some(6), Some(7)],
        transitions: &[
            DFATransition(0, 6, 1),
            DFATransition(0, 7, 2),
            DFATransition(0, 8, 3),
            DFATransition(0, 9, 3),
            DFATransition(0, 10, 3),
            DFATransition(0, 11, 3),
        ],
        k: 1,
    },
    /* 5 - "Declarations" */
    LookaheadDFA {
        states: &[None, Some(3), Some(4)],
        transitions: &[
            DFATransition(0, 6, 1),
            DFATransition(0, 7, 1),
            DFATransition(0, 8, 1),
            DFATransition(0, 9, 1),
            DFATransition(0, 10, 1),
            DFATransition(0, 11, 1),
            DFATransition(0, 12, 2),
            DFATransition(0, 26, 2),
        ],
        k: 1,
    },
    /* 6 - "Factor" */
    LookaheadDFA {
        states: &[None, Some(24), Some(25), Some(26), Some(27)],
        transitions: &[
            DFATransition(0, 16, 4),
            DFATransition(0, 18, 1),
            DFATransition(0, 20, 3),
            DFATransition(0, 22, 2),
            DFATransition(0, 24, 4),
            DFATransition(0, 25, 4),
            DFATransition(0, 28, 4),
            DFATransition(0, 29, 4),
            DFATransition(0, 30, 4),
        ],
        k: 1,
    },
    /* 7 - "GrammarDefinition" */
    LookaheadDFA {
        states: &[Some(14)],
        transitions: &[],
        k: 0,
    },
    /* 8 - "GrammarDefinitionList" */
    LookaheadDFA {
        states: &[None, Some(15), Some(16)],
        transitions: &[DFATransition(0, 0, 2), DFATransition(0, 24, 1)],
        k: 1,
    },
    /* 9 - "Group" */
    LookaheadDFA {
        states: &[Some(34)],
        transitions: &[],
        k: 0,
    },
    /* 10 - "Identifier" */
    LookaheadDFA {
        states: &[Some(37)],
        transitions: &[],
        k: 0,
    },
    /* 11 - "Optional" */
    LookaheadDFA {
        states: &[Some(35)],
        transitions: &[],
        k: 0,
    },
    /* 12 - "Parol" */
    LookaheadDFA {
        states: &[Some(0)],
        transitions: &[],
        k: 0,
    },
    /* 13 - "Production" */
    LookaheadDFA {
        states: &[Some(17)],
        transitions: &[],
        k: 0,
    },
    /* 14 - "Prolog" */
    LookaheadDFA {
        states: &[Some(1)],
        transitions: &[],
        k: 0,
    },
    /* 15 - "Repeat" */
    LookaheadDFA {
        states: &[Some(36)],
        transitions: &[],
        k: 0,
    },
    /* 16 - "ScannerDirectives" */
    LookaheadDFA {
        states: &[None, Some(8), Some(9), Some(10), Some(11)],
        transitions: &[
            DFATransition(0, 8, 1),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 3),
            DFATransition(0, 11, 4),
        ],
        k: 1,
    },
    /* 17 - "ScannerNameOpt" */
    LookaheadDFA {
        states: &[None, Some(48), Some(49)],
        transitions: &[DFATransition(0, 19, 2), DFATransition(0, 24, 1)],
        k: 1,
    },
    /* 18 - "ScannerState" */
    LookaheadDFA {
        states: &[Some(39)],
        transitions: &[],
        k: 0,
    },
    /* 19 - "ScannerStateList" */
    LookaheadDFA {
        states: &[None, Some(40), Some(41)],
        transitions: &[
            DFATransition(0, 8, 1),
            DFATransition(0, 9, 1),
            DFATransition(0, 10, 1),
            DFATransition(0, 11, 1),
            DFATransition(0, 23, 2),
        ],
        k: 1,
    },
    /* 20 - "ScannerStates" */
    LookaheadDFA {
        states: &[None, Some(12), Some(13)],
        transitions: &[DFATransition(0, 12, 2), DFATransition(0, 26, 1)],
        k: 1,
    },
    /* 21 - "ScannerSwitch" */
    LookaheadDFA {
        states: &[None, Some(45), Some(46), Some(47)],
        transitions: &[
            DFATransition(0, 28, 1),
            DFATransition(0, 29, 2),
            DFATransition(0, 30, 3),
        ],
        k: 1,
    },
    /* 22 - "SimpleToken" */
    LookaheadDFA {
        states: &[Some(32)],
        transitions: &[],
        k: 0,
    },
    /* 23 - "StartDeclaration" */
    LookaheadDFA {
        states: &[Some(2)],
        transitions: &[],
        k: 0,
    },
    /* 24 - "StateList" */
    LookaheadDFA {
        states: &[Some(42)],
        transitions: &[],
        k: 0,
    },
    /* 25 - "StateListRest" */
    LookaheadDFA {
        states: &[None, Some(43), Some(44)],
        transitions: &[DFATransition(0, 17, 2), DFATransition(0, 27, 1)],
        k: 1,
    },
    /* 26 - "String" */
    LookaheadDFA {
        states: &[Some(38)],
        transitions: &[],
        k: 0,
    },
    /* 27 - "Symbol" */
    LookaheadDFA {
        states: &[None, Some(28), Some(29), Some(30), Some(31)],
        transitions: &[
            DFATransition(0, 16, 3),
            DFATransition(0, 24, 1),
            DFATransition(0, 25, 2),
            DFATransition(0, 28, 4),
            DFATransition(0, 29, 4),
            DFATransition(0, 30, 4),
        ],
        k: 1,
    },
    /* 28 - "TokenWithStates" */
    LookaheadDFA {
        states: &[Some(33)],
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 50] = &[
    // 0 - Parol: Prolog GrammarDefinition;
    Production {
        lhs: 12,
        production: &[ParseType::N(7), ParseType::N(14)],
    },
    // 1 - Prolog: StartDeclaration Declarations ScannerStates;
    Production {
        lhs: 14,
        production: &[ParseType::N(20), ParseType::N(5), ParseType::N(23)],
    },
    // 2 - StartDeclaration: "%start" Identifier;
    Production {
        lhs: 23,
        production: &[ParseType::N(10), ParseType::T(5)],
    },
    // 3 - Declarations: Declaration Declarations;
    Production {
        lhs: 5,
        production: &[ParseType::N(5), ParseType::N(4)],
    },
    // 4 - Declarations: ;
    Production {
        lhs: 5,
        production: &[],
    },
    // 5 - Declaration: "%title" String;
    Production {
        lhs: 4,
        production: &[ParseType::N(26), ParseType::T(6)],
    },
    // 6 - Declaration: "%comment" String;
    Production {
        lhs: 4,
        production: &[ParseType::N(26), ParseType::T(7)],
    },
    // 7 - Declaration: ScannerDirectives;
    Production {
        lhs: 4,
        production: &[ParseType::N(16)],
    },
    // 8 - ScannerDirectives: "%line_comment" String;
    Production {
        lhs: 16,
        production: &[ParseType::N(26), ParseType::T(8)],
    },
    // 9 - ScannerDirectives: "%block_comment" String String;
    Production {
        lhs: 16,
        production: &[ParseType::N(26), ParseType::N(26), ParseType::T(9)],
    },
    // 10 - ScannerDirectives: "%auto_newline_off";
    Production {
        lhs: 16,
        production: &[ParseType::T(10)],
    },
    // 11 - ScannerDirectives: "%auto_ws_off";
    Production {
        lhs: 16,
        production: &[ParseType::T(11)],
    },
    // 12 - ScannerStates: ScannerState ScannerStates;
    Production {
        lhs: 20,
        production: &[ParseType::N(20), ParseType::N(18)],
    },
    // 13 - ScannerStates: ;
    Production {
        lhs: 20,
        production: &[],
    },
    // 14 - GrammarDefinition: "%%" Production GrammarDefinitionList;
    Production {
        lhs: 7,
        production: &[ParseType::N(8), ParseType::N(13), ParseType::T(12)],
    },
    // 15 - GrammarDefinitionList: Production GrammarDefinitionList;
    Production {
        lhs: 8,
        production: &[ParseType::N(8), ParseType::N(13)],
    },
    // 16 - GrammarDefinitionList: ;
    Production {
        lhs: 8,
        production: &[],
    },
    // 17 - Production: Identifier ":" Alternations ";";
    Production {
        lhs: 13,
        production: &[
            ParseType::T(14),
            ParseType::N(2),
            ParseType::T(13),
            ParseType::N(10),
        ],
    },
    // 18 - Alternations: Alternation AlternationsList;
    Production {
        lhs: 2,
        production: &[ParseType::N(3), ParseType::N(0)],
    },
    // 19 - AlternationsList: "\|" Alternation AlternationsList;
    Production {
        lhs: 3,
        production: &[ParseType::N(3), ParseType::N(0), ParseType::T(15)],
    },
    // 20 - AlternationsList: ;
    Production {
        lhs: 3,
        production: &[],
    },
    // 21 - Alternation: AlternationList;
    Production {
        lhs: 0,
        production: &[ParseType::N(1)],
    },
    // 22 - AlternationList: Factor AlternationList;
    Production {
        lhs: 1,
        production: &[ParseType::N(1), ParseType::N(6)],
    },
    // 23 - AlternationList: ;
    Production {
        lhs: 1,
        production: &[],
    },
    // 24 - Factor: Group;
    Production {
        lhs: 6,
        production: &[ParseType::N(9)],
    },
    // 25 - Factor: Repeat;
    Production {
        lhs: 6,
        production: &[ParseType::N(15)],
    },
    // 26 - Factor: Optional;
    Production {
        lhs: 6,
        production: &[ParseType::N(11)],
    },
    // 27 - Factor: Symbol;
    Production {
        lhs: 6,
        production: &[ParseType::N(27)],
    },
    // 28 - Symbol: Identifier;
    Production {
        lhs: 27,
        production: &[ParseType::N(10)],
    },
    // 29 - Symbol: SimpleToken;
    Production {
        lhs: 27,
        production: &[ParseType::N(22)],
    },
    // 30 - Symbol: TokenWithStates;
    Production {
        lhs: 27,
        production: &[ParseType::N(28)],
    },
    // 31 - Symbol: ScannerSwitch;
    Production {
        lhs: 27,
        production: &[ParseType::N(21)],
    },
    // 32 - SimpleToken: String;
    Production {
        lhs: 22,
        production: &[ParseType::N(26)],
    },
    // 33 - TokenWithStates: "<" StateList ">" String;
    Production {
        lhs: 28,
        production: &[
            ParseType::N(26),
            ParseType::T(17),
            ParseType::N(24),
            ParseType::T(16),
        ],
    },
    // 34 - Group: "\(" Factor Alternations "\)";
    Production {
        lhs: 9,
        production: &[
            ParseType::T(19),
            ParseType::N(2),
            ParseType::N(6),
            ParseType::T(18),
        ],
    },
    // 35 - Optional: "\[" Factor Alternations "\]";
    Production {
        lhs: 11,
        production: &[
            ParseType::T(21),
            ParseType::N(2),
            ParseType::N(6),
            ParseType::T(20),
        ],
    },
    // 36 - Repeat: "\{" Factor Alternations "\}";
    Production {
        lhs: 15,
        production: &[
            ParseType::T(23),
            ParseType::N(2),
            ParseType::N(6),
            ParseType::T(22),
        ],
    },
    // 37 - Identifier: "[a-zA-Z_]\w*";
    Production {
        lhs: 10,
        production: &[ParseType::T(24)],
    },
    // 38 - String: "\u{0022}([^\\]|\\.)*?\u{0022}";
    Production {
        lhs: 26,
        production: &[ParseType::T(25)],
    },
    // 39 - ScannerState: "%scanner" Identifier "\{" ScannerStateList "\}";
    Production {
        lhs: 18,
        production: &[
            ParseType::T(23),
            ParseType::N(19),
            ParseType::T(22),
            ParseType::N(10),
            ParseType::T(26),
        ],
    },
    // 40 - ScannerStateList: ScannerDirectives ScannerStateList;
    Production {
        lhs: 19,
        production: &[ParseType::N(19), ParseType::N(16)],
    },
    // 41 - ScannerStateList: ;
    Production {
        lhs: 19,
        production: &[],
    },
    // 42 - StateList: Identifier StateListRest;
    Production {
        lhs: 24,
        production: &[ParseType::N(25), ParseType::N(10)],
    },
    // 43 - StateListRest: "," Identifier StateListRest;
    Production {
        lhs: 25,
        production: &[ParseType::N(25), ParseType::N(10), ParseType::T(27)],
    },
    // 44 - StateListRest: ;
    Production {
        lhs: 25,
        production: &[],
    },
    // 45 - ScannerSwitch: "%sc" "\(" ScannerNameOpt "\)";
    Production {
        lhs: 21,
        production: &[
            ParseType::T(19),
            ParseType::N(17),
            ParseType::T(18),
            ParseType::T(28),
        ],
    },
    // 46 - ScannerSwitch: "%push" "\(" Identifier "\)";
    Production {
        lhs: 21,
        production: &[
            ParseType::T(19),
            ParseType::N(10),
            ParseType::T(18),
            ParseType::T(29),
        ],
    },
    // 47 - ScannerSwitch: "%pop" "\(" "\)";
    Production {
        lhs: 21,
        production: &[ParseType::T(19), ParseType::T(18), ParseType::T(30)],
    },
    // 48 - ScannerNameOpt: Identifier;
    Production {
        lhs: 17,
        production: &[ParseType::N(10)],
    },
    // 49 - ScannerNameOpt: ;
    Production {
        lhs: 17,
        production: &[],
    },
];

lazy_static! {
    static ref TOKENIZERS: Vec<(&'static str, Tokenizer)> = vec![(
        "INITIAL",
        Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap()
    ),];
}

pub fn parse(
    input: &str,
    file_name: String,
    user_actions: &mut dyn UserActionsTrait,
) -> Result<Tree<ParseTreeType>> {
    let mut llk_parser = LLKParser::new(
        12,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    let token_stream =
        RefCell::new(TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap());
    let result = llk_parser.parse(token_stream, user_actions);
    match result {
        Ok(()) => Ok(llk_parser.parse_tree),
        Err(e) => Err(e),
    }
}
