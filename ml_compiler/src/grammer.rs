// ================================User Codes Begin================================
use crate::enums::Expr;
use crate::enums::Token;

// =================================User Codes End=================================
/*
====================================Grammar=====================================

# of terminal classes: 28
# of states: 235

0: ArgExpr -> Int
1: ArgExpr -> Bool
2: ArgExpr -> String
3: ArgExpr -> Id
4: ArgExpr -> Unit
5: ArgExpr -> Open Expr Close
6: Expr -> Expr Plus Expr
7: Expr -> Expr Minus Expr
8: Expr -> Expr Geq Expr
9: Expr -> Expr Leq Expr
10: Expr -> Expr Gt Expr
11: Expr -> Expr Lt Expr
12: Expr -> Expr Eq Expr
13: Expr -> Expr Neq Expr
14: Expr -> Expr And Expr
15: Expr -> Expr Or Expr
16: Expr -> Not Expr
17: Expr -> Let ArgExpr Eq Expr In Expr
18: Expr -> Let Rec ArgExpr Eq Expr In Expr
19: Expr -> Fun ArgExpr Arrow Expr
20: Expr -> If Expr Then Expr Else Expr
21: Expr -> Expr Semi Expr
22: Expr -> ArgExpr
23: Expr -> Expr ArgExpr
24: Augmented -> VirtualStart(0) Expr eof

*/
// =============================Generated Codes Begin==============================
#[allow(non_camel_case_types, dead_code)]
pub type ExprContext = ::rusty_lr::parser::deterministic::Context<Parser, Data, ExprExtracter, u8>;
#[allow(non_camel_case_types, dead_code)]
pub type Rule = ::rusty_lr::production::Production<TerminalClasses, NonTerminals>;
#[allow(non_camel_case_types, dead_code)]
pub type Tables = ::rusty_lr::parser::table::DenseFlatTables<TerminalClasses, NonTerminals, u8, u8>;
#[allow(non_camel_case_types, dead_code)]
pub type ParseError = ::rusty_lr::parser::deterministic::ParseError<
    Token,
    ::rusty_lr::DefaultLocation,
    ::rusty_lr::DefaultReduceActionError,
    (),
>;
/// A enum that represents terminal classes
#[allow(non_camel_case_types, dead_code)]
#[derive(
    Clone,
    Copy,
    std::hash::Hash,
    std::cmp::PartialEq,
    std::cmp::Eq,
    std::cmp::PartialOrd,
    std::cmp::Ord,
)]
#[repr(usize)]
pub enum TerminalClasses {
    Let,
    Rec,
    Fun,
    Arrow,
    In,
    If,
    Then,
    Else,
    Geq,
    Leq,
    Gt,
    Lt,
    Eq,
    Neq,
    Plus,
    Minus,
    And,
    Or,
    Not,
    Unit,
    Open,
    Close,
    Semi,
    String,
    Int,
    Bool,
    Id,
    __rustylr_other_terminals,
    error,
    eof,
    VirtualStart0,
}
impl TerminalClasses {
    #[inline]
    pub fn from_usize(value: usize) -> Self {
        debug_assert!(
            value < 31usize,
            "Terminal class index {} is out of bounds (max {})",
            value,
            31usize
        );
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::rusty_lr::parser::terminalclass::TerminalClass for TerminalClasses {
    type Term = Token;
    const ERROR: Self = Self::error;
    const EOF: Self = Self::eof;
    fn as_str(&self) -> &'static str {
        match self {
            TerminalClasses::Let => "Let",
            TerminalClasses::Rec => "Rec",
            TerminalClasses::Fun => "Fun",
            TerminalClasses::Arrow => "Arrow",
            TerminalClasses::In => "In",
            TerminalClasses::If => "If",
            TerminalClasses::Then => "Then",
            TerminalClasses::Else => "Else",
            TerminalClasses::Geq => "Geq",
            TerminalClasses::Leq => "Leq",
            TerminalClasses::Gt => "Gt",
            TerminalClasses::Lt => "Lt",
            TerminalClasses::Eq => "Eq",
            TerminalClasses::Neq => "Neq",
            TerminalClasses::Plus => "Plus",
            TerminalClasses::Minus => "Minus",
            TerminalClasses::And => "And",
            TerminalClasses::Or => "Or",
            TerminalClasses::Not => "Not",
            TerminalClasses::Unit => "Unit",
            TerminalClasses::Open => "Open",
            TerminalClasses::Close => "Close",
            TerminalClasses::Semi => "Semi",
            TerminalClasses::String => "String",
            TerminalClasses::Int => "Int",
            TerminalClasses::Bool => "Bool",
            TerminalClasses::Id => "Id",
            TerminalClasses::__rustylr_other_terminals => "__rustylr_other_terminals",
            TerminalClasses::error => "error",
            TerminalClasses::eof => "eof",
            TerminalClasses::VirtualStart0 => "virtual_start",
        }
    }
    fn to_usize(&self) -> usize {
        *self as usize
    }
    fn from_term(terminal: &Self::Term) -> Self {
        #[allow(unreachable_patterns, unused_variables)]
        match terminal {
            Token::LET => TerminalClasses::Let,
            Token::REC => TerminalClasses::Rec,
            Token::FUN => TerminalClasses::Fun,
            Token::ARROW => TerminalClasses::Arrow,
            Token::IN => TerminalClasses::In,
            Token::IF => TerminalClasses::If,
            Token::THEN => TerminalClasses::Then,
            Token::ELSE => TerminalClasses::Else,
            Token::GEQ => TerminalClasses::Geq,
            Token::LEQ => TerminalClasses::Leq,
            Token::GT => TerminalClasses::Gt,
            Token::LT => TerminalClasses::Lt,
            Token::EQ => TerminalClasses::Eq,
            Token::NEQ => TerminalClasses::Neq,
            Token::PLUS => TerminalClasses::Plus,
            Token::MINUS => TerminalClasses::Minus,
            Token::AND => TerminalClasses::And,
            Token::OR => TerminalClasses::Or,
            Token::NOT => TerminalClasses::Not,
            Token::UNIT => TerminalClasses::Unit,
            Token::OPEN => TerminalClasses::Open,
            Token::CLOSE => TerminalClasses::Close,
            Token::SEMI => TerminalClasses::Semi,
            Token::STRING(_) => TerminalClasses::String,
            Token::INT(_) => TerminalClasses::Int,
            Token::BOOL(_) => TerminalClasses::Bool,
            Token::ID(_) => TerminalClasses::Id,
            _ => TerminalClasses::__rustylr_other_terminals,
        }
    }
    fn from_virtual_start(branch_idx: u32) -> Self {
        match branch_idx {
            0u32 => Self::VirtualStart0,
            _ => panic!("Invalid virtual start branch index: {}", branch_idx),
        }
    }
}
impl std::fmt::Display for TerminalClasses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ::rusty_lr::parser::terminalclass::TerminalClass;
        write!(f, "{}", self.as_str())
    }
}
impl std::fmt::Debug for TerminalClasses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ::rusty_lr::parser::terminalclass::TerminalClass;
        write!(f, "{}", self.as_str())
    }
}
/// An enum that represents non-terminal symbols
#[allow(non_camel_case_types, dead_code)]
#[derive(
    Clone,
    Copy,
    std::hash::Hash,
    std::cmp::PartialEq,
    std::cmp::Eq,
    std::cmp::PartialOrd,
    std::cmp::Ord,
)]
#[repr(usize)]
pub enum NonTerminals {
    ArgExpr,
    Expr,
    Augmented,
}
impl NonTerminals {
    #[inline]
    pub fn from_usize(value: usize) -> Self {
        debug_assert!(
            value < 3usize,
            "Non-terminal index {} is out of bounds (max {})",
            value,
            3usize
        );
        unsafe { ::std::mem::transmute(value) }
    }
}
impl std::fmt::Display for NonTerminals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ::rusty_lr::parser::nonterminal::NonTerminal;
        write!(f, "{}", self.as_str())
    }
}
impl std::fmt::Debug for NonTerminals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ::rusty_lr::parser::nonterminal::NonTerminal;
        write!(f, "{}", self.as_str())
    }
}
impl ::rusty_lr::parser::nonterminal::NonTerminal for NonTerminals {
    fn as_str(&self) -> &'static str {
        match self {
            NonTerminals::ArgExpr => "ArgExpr",
            NonTerminals::Expr => "Expr",
            NonTerminals::Augmented => "Augmented",
        }
    }
    fn nonterm_type(&self) -> Option<::rusty_lr::parser::nonterminal::NonTerminalType> {
        match self {
            NonTerminals::ArgExpr => None,
            NonTerminals::Expr => None,
            NonTerminals::Augmented => {
                Some(::rusty_lr::parser::nonterminal::NonTerminalType::Augmented)
            }
        }
    }
    fn to_usize(&self) -> usize {
        *self as usize
    }
}
/// enum for each non-terminal and terminal symbol, that actually hold data
#[rustfmt::skip]
#[allow(unused_braces, unused_parens, non_snake_case, non_camel_case_types)]
#[doc(hidden)]
#[derive(Clone)]
pub enum __RustyLRData<__RustyLRData0, __RustyLRData1> {
    __terminals(__RustyLRData0),
    __variant1(__RustyLRData1),
    Empty,
}
pub type Data = __RustyLRData<Token, Expr>;
impl ::std::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::__terminals(..) => f.write_str(stringify!(__terminals)),
            Self::__variant1(..) => f.write_str(stringify!(__variant1)),
            Self::Empty => f.write_str("Empty"),
        }
    }
}
#[doc(hidden)]
#[allow(non_camel_case_types, dead_code)]
pub struct ExprExtracter;
impl ::rusty_lr::parser::semantic_value::StartExtractor<Data> for ExprExtracter {
    type StartType = Expr;
    const BRANCH_INDEX: u32 = 0u32;
    fn extract(value: Data) -> Option<Self::StartType> {
        #[allow(unreachable_patterns, unused_variables)]
        match value {
            Data::__variant1(val) => Some(val),
            _ => None,
        }
    }
}
#[rustfmt::skip]
#[allow(
    unused_braces,
    unused_parens,
    unused_variables,
    non_snake_case,
    unused_mut,
    dead_code,
    unreachable_patterns
)]
impl Data {
    ///ArgExpr -> Int
    #[inline]
    fn reduce_ArgExpr_0(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__terminals(_)))
            );
        }
        __location_stack.pop();
        let mut Int = match __data_stack.pop().unwrap() {
            Data::__terminals(val) => val,
            _ => unreachable!(),
        };
        let __res = {
            match Int {
                Token::INT(val) => Expr::INT(val),
                _ => unreachable!(),
            }
        };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///ArgExpr -> Bool
    #[inline]
    fn reduce_ArgExpr_1(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__terminals(_)))
            );
        }
        __location_stack.pop();
        let mut Bool = match __data_stack.pop().unwrap() {
            Data::__terminals(val) => val,
            _ => unreachable!(),
        };
        let __res = {
            match Bool {
                Token::BOOL(val) => Expr::BOOL(val),
                _ => unreachable!(),
            }
        };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///ArgExpr -> String
    #[inline]
    fn reduce_ArgExpr_2(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__terminals(_)))
            );
        }
        __location_stack.pop();
        let mut String = match __data_stack.pop().unwrap() {
            Data::__terminals(val) => val,
            _ => unreachable!(),
        };
        let __res = {
            match String {
                Token::STRING(val) => Expr::STRING(val),
                _ => unreachable!(),
            }
        };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///ArgExpr -> Id
    #[inline]
    fn reduce_ArgExpr_3(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__terminals(_)))
            );
        }
        __location_stack.pop();
        let mut Id = match __data_stack.pop().unwrap() {
            Data::__terminals(val) => val,
            _ => unreachable!(),
        };
        let __res = {
            match Id {
                Token::ID(val) => Expr::ID(val),
                _ => unreachable!(),
            }
        };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///ArgExpr -> Unit
    #[inline]
    fn reduce_ArgExpr_4(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::Empty))
            );
        }
        __location_stack.pop();
        __data_stack.pop();
        let __res = { Expr::UNIT };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///ArgExpr -> Open Expr Close
    #[inline]
    fn reduce_ArgExpr_5(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::Empty))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let __res = e1;
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Plus Expr
    #[inline]
    fn reduce_Expr_0(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::ADD(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Minus Expr
    #[inline]
    fn reduce_Expr_1(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::SUB(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Geq Expr
    #[inline]
    fn reduce_Expr_2(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::GEQ(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Leq Expr
    #[inline]
    fn reduce_Expr_3(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::LEQ(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Gt Expr
    #[inline]
    fn reduce_Expr_4(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::GT(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Lt Expr
    #[inline]
    fn reduce_Expr_5(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::LT(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Eq Expr
    #[inline]
    fn reduce_Expr_6(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::EQ(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Neq Expr
    #[inline]
    fn reduce_Expr_7(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::NEQ(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr And Expr
    #[inline]
    fn reduce_Expr_8(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::AND(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Or Expr
    #[inline]
    fn reduce_Expr_9(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::OR(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Not Expr
    #[inline]
    fn reduce_Expr_10(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
        }
        __location_stack.truncate(__location_stack.len() - 2);
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let __res = { Expr::NOT(Box::new(e1)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Let ArgExpr Eq Expr In Expr
    #[inline]
    fn reduce_Expr_11(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 3usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 4usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 5usize), Some(&
                Data::Empty))
            );
        }
        __location_stack.truncate(__location_stack.len() - 6);
        let mut e3 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let __res = { Expr::LET(Box::new(e1), Box::new(e2), Box::new(e3)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Let Rec ArgExpr Eq Expr In Expr
    #[inline]
    fn reduce_Expr_12(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 3usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 4usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 5usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 6usize), Some(&
                Data::Empty))
            );
        }
        __location_stack.truncate(__location_stack.len() - 7);
        let mut e3 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.truncate(__data_stack.len() - 2);
        let __res = { Expr::LETREC(Box::new(e1), Box::new(e2), Box::new(e3)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Fun ArgExpr Arrow Expr
    #[inline]
    fn reduce_Expr_13(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 3usize), Some(&
                Data::Empty))
            );
        }
        __location_stack.truncate(__location_stack.len() - 4);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let __res = { Expr::FUN(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> If Expr Then Expr Else Expr
    #[inline]
    fn reduce_Expr_14(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 3usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 4usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 5usize), Some(&
                Data::Empty))
            );
        }
        __location_stack.truncate(__location_stack.len() - 6);
        let mut e3 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let __res = { Expr::IF(Box::new(e1), Box::new(e2), Box::new(e3)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr Semi Expr
    #[inline]
    fn reduce_Expr_15(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::Empty))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 2usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 3);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        __data_stack.pop();
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::SEMI(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
    ///Expr -> Expr ArgExpr
    #[inline]
    fn reduce_Expr_17(
        __data_stack: &mut Vec<Self>,
        __location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        __push_data: bool,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Token>,
        data: &mut (),
        __rustylr_location0: &mut ::rusty_lr::DefaultLocation,
    ) -> Result<(), ::rusty_lr::DefaultReduceActionError> {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 0usize), Some(&
                Data::__variant1(_)))
            );
            debug_assert!(
                matches!(__data_stack.get(__data_stack.len() - 1 - 1usize), Some(&
                Data::__variant1(_)))
            );
        }
        __location_stack.truncate(__location_stack.len() - 2);
        let mut e2 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let mut e1 = match __data_stack.pop().unwrap() {
            Data::__variant1(val) => val,
            _ => unreachable!(),
        };
        let __res = { Expr::APP(Box::new(e1), Box::new(e2)) };
        if __push_data {
            __data_stack.push(Self::__variant1(__res));
        } else {
            __data_stack.push(Self::Empty);
        }
        Ok(())
    }
}
#[rustfmt::skip]
#[allow(
    unused_braces,
    unused_parens,
    non_snake_case,
    non_camel_case_types,
    unused_variables
)]
impl ::rusty_lr::parser::semantic_value::SemanticValue for Data {
    type Term = Token;
    type NonTerm = NonTerminals;
    type ReduceActionError = ::rusty_lr::DefaultReduceActionError;
    type UserData = ();
    type Location = ::rusty_lr::DefaultLocation;
    fn new_empty() -> Self {
        Self::Empty
    }
    fn new_terminal(term: Self::Term) -> Self {
        Self::__terminals(term)
    }
    fn reduce_action(
        data_stack: &mut Vec<Self>,
        location_stack: &mut Vec<::rusty_lr::DefaultLocation>,
        push_data: bool,
        rule_index: usize,
        shift: &mut bool,
        lookahead: &::rusty_lr::TerminalSymbol<Self::Term>,
        user_data: &mut Self::UserData,
        location0: &mut Self::Location,
    ) -> Result<(), Self::ReduceActionError> {
        match rule_index {
            0usize => {
                Self::reduce_ArgExpr_0(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            1usize => {
                Self::reduce_ArgExpr_1(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            2usize => {
                Self::reduce_ArgExpr_2(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            3usize => {
                Self::reduce_ArgExpr_3(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            4usize => {
                Self::reduce_ArgExpr_4(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            5usize => {
                Self::reduce_ArgExpr_5(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            6usize => {
                Self::reduce_Expr_0(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            7usize => {
                Self::reduce_Expr_1(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            8usize => {
                Self::reduce_Expr_2(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            9usize => {
                Self::reduce_Expr_3(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            10usize => {
                Self::reduce_Expr_4(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            11usize => {
                Self::reduce_Expr_5(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            12usize => {
                Self::reduce_Expr_6(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            13usize => {
                Self::reduce_Expr_7(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            14usize => {
                Self::reduce_Expr_8(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            15usize => {
                Self::reduce_Expr_9(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            16usize => {
                Self::reduce_Expr_10(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            17usize => {
                Self::reduce_Expr_11(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            18usize => {
                Self::reduce_Expr_12(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            19usize => {
                Self::reduce_Expr_13(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            20usize => {
                Self::reduce_Expr_14(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            21usize => {
                Self::reduce_Expr_15(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            23usize => {
                Self::reduce_Expr_17(
                    data_stack,
                    location_stack,
                    push_data,
                    shift,
                    lookahead,
                    user_data,
                    location0,
                )
            }
            _ => {
                unreachable!("Invalid Rule: {}", rule_index);
            }
        }
    }
}
/// A lightweight parser struct that references the static parser tables and production rules.
///
/// Since this struct only holds `'static` references to shared, read-only static parser tables,
/// it is extremely cheap to instantiate, copy, or clone, and takes very little space.
#[allow(
    unused_braces,
    unused_parens,
    unused_variables,
    non_snake_case,
    unused_mut
)]
#[derive(Clone, Copy)]
pub struct Parser;
unsafe impl ::std::marker::Send for Parser {}
unsafe impl ::std::marker::Sync for Parser {}
#[rustfmt::skip]
impl ::rusty_lr::parser::Parser for Parser {
    type Term = Token;
    type TermClass = TerminalClasses;
    type NonTerm = NonTerminals;
    type StateIndex = u8;
    type ReduceRules = u8;
    type Tables = Tables;
    const ERROR_USED: bool = false;
    fn get_tables() -> &'static Tables {
        static TABLES: std::sync::OnceLock<Tables> = std::sync::OnceLock::new();
        TABLES
            .get_or_init(|| {
                static RULE_NAMES: &[u32] = &[
                    0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    1, 2,
                ];
                static RULE_LENGTHS: &[u32] = &[
                    1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 6, 7, 4, 6, 3, 1,
                    2, 3,
                ];
                static SHIFT_TERM_DATA: &[u32] = &[
                    2147516446, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 98305, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 196608,
                    622594, 720901, 1081362, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 229377, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 425996, 458752, 589826, 884741,
                    2162706, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 491521, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 557068, 458752, 589826, 884741, 2162706,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    688131, 196608, 622594, 720901, 1081362, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 753664, 917506, 1015813, 1048594,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    786433, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 851980, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 983043, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1146888, 1507337, 1572874, 1638411, 1703948, 1769485,
                    1212430, 1441807, 1277968, 1376273, 131091, 163860, 1835029, 1867798,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1212430, 1441807, 1277968, 1376273, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1277968, 1376273, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 196608, 622594, 720901, 1081362, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 196608,
                    622594, 720901, 1081362, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 1277968, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 196608, 622594, 720901, 1081362,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    1277968, 1376273, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 1212430, 1441807,
                    1277968, 1376273, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 1212430, 1441807,
                    1277968, 1376273, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 1212430, 1441807,
                    1277968, 1376273, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 1212430, 1441807,
                    1277968, 1376273, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 1212430, 1441807,
                    1277968, 1376273, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 1146888, 1507337,
                    1572874, 1638411, 1703948, 1769485, 1212430, 1441807, 1277968,
                    1376273, 131091, 163860, 1867798, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1998854, 3276808, 3571721, 3637258, 3702795, 3768332,
                    3833869, 3342350, 3506191, 3407888, 3440657, 131091, 163860, 3899414,
                    2147745815, 2147778584, 2147811353, 2147844122, 2031616, 2260994,
                    2359301, 2457618, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2064385, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 2129932, 458752, 589826, 884741, 2162706,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2162706, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 2228228, 4030472, 4325385,
                    4390922, 4456459, 4521996, 4587533, 4096014, 4259855, 4161552,
                    4194321, 131091, 163860, 4653078, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2326531, 2031616,
                    2260994, 2359301, 2457618, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 753664, 917506, 1015813, 1048594, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 2424838,
                    3276808, 3571721, 3637258, 3702795, 3768332, 3833869, 3342350,
                    3506191, 3407888, 3440657, 131091, 163860, 3899414, 2147745815,
                    2147778584, 2147811353, 2147844122, 2031616, 2260994, 2359301,
                    2457618, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2523143, 2588680,
                    2883593, 2949130, 3014667, 3080204, 3145741, 2654222, 2818063,
                    2719760, 2752529, 131091, 163860, 3211286, 2147745815, 2147778584,
                    2147811353, 2147844122, 2031616, 2260994, 2359301, 2457618, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 2588680,
                    2883593, 2949130, 3014667, 3080204, 3145741, 2654222, 2818063,
                    2719760, 2752529, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2654222, 2818063,
                    2719760, 2752529, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2719760, 2752529,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    2031616, 2260994, 2359301, 2457618, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 2031616, 2260994, 2359301,
                    2457618, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2719760, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 2031616, 2260994, 2359301, 2457618, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 2719760,
                    2752529, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2654222, 2818063,
                    2719760, 2752529, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2654222, 2818063,
                    2719760, 2752529, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2654222, 2818063,
                    2719760, 2752529, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2654222, 2818063,
                    2719760, 2752529, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2654222, 2818063,
                    2719760, 2752529, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2031616, 2260994, 2359301, 2457618, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2588680, 2883593,
                    2949130, 3014667, 3080204, 3145741, 2654222, 2818063, 2719760,
                    2752529, 131091, 163860, 3211286, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 3342350, 3506191,
                    3407888, 3440657, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 3407888, 3440657,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    753664, 917506, 1015813, 1048594, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 753664, 917506, 1015813, 1048594,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    3407888, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 3407888, 3440657,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    753664, 917506, 1015813, 1048594, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 3342350, 3506191, 3407888,
                    3440657, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 3342350, 3506191,
                    3407888, 3440657, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 3342350, 3506191,
                    3407888, 3440657, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 3342350, 3506191,
                    3407888, 3440657, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 3342350, 3506191,
                    3407888, 3440657, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 3276808, 3571721,
                    3637258, 3702795, 3768332, 3833869, 3342350, 3506191, 3407888,
                    3440657, 131091, 163860, 3899414, 2147745815, 2147778584, 2147811353,
                    2147844122, 2588680, 2883593, 2949130, 3014667, 3080204, 3145741,
                    2654222, 2818063, 2719760, 2752529, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 2588680, 2883593, 2949130,
                    3014667, 3080204, 3145741, 2654222, 2818063, 2719760, 2752529,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2162706, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 4096014, 4259855, 4161552,
                    4194321, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 4161552, 4194321,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2162706, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 458752, 589826, 884741, 2162706,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    4161552, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 4161552, 4194321,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2162706, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 4096014, 4259855, 4161552,
                    4194321, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 4096014, 4259855,
                    4161552, 4194321, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 4096014, 4259855,
                    4161552, 4194321, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 4096014, 4259855,
                    4161552, 4194321, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 4096014, 4259855,
                    4161552, 4194321, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 4030472, 4325385,
                    4390922, 4456459, 4521996, 4587533, 4096014, 4259855, 4161552,
                    4194321, 131091, 163860, 4653078, 2147745815, 2147778584, 2147811353,
                    2147844122, 4751372, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 4816900, 4030472,
                    4325385, 4390922, 4456459, 4521996, 4587533, 4096014, 4259855,
                    4161552, 4194321, 131091, 163860, 4653078, 2147745815, 2147778584,
                    2147811353, 2147844122, 2031616, 2260994, 2359301, 2457618, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 2588680,
                    2883593, 2949130, 3014667, 3080204, 3145741, 2654222, 2818063,
                    2719760, 2752529, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 4915207, 2588680, 2883593, 2949130, 3014667, 3080204,
                    3145741, 2654222, 2818063, 2719760, 2752529, 131091, 163860, 3211286,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 3276808, 3571721, 3637258, 3702795, 3768332, 3833869,
                    3342350, 3506191, 3407888, 3440657, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 3276808, 3571721, 3637258,
                    3702795, 3768332, 3833869, 3342350, 3506191, 3407888, 3440657,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    5046278, 3276808, 3571721, 3637258, 3702795, 3768332, 3833869,
                    3342350, 3506191, 3407888, 3440657, 131091, 163860, 3899414,
                    2147745815, 2147778584, 2147811353, 2147844122, 2031616, 2260994,
                    2359301, 2457618, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 5111815, 2588680, 2883593, 2949130, 3014667, 3080204,
                    3145741, 2654222, 2818063, 2719760, 2752529, 131091, 163860, 3211286,
                    2147745815, 2147778584, 2147811353, 2147844122, 458752, 589826,
                    884741, 2162706, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 4030472, 4325385, 4390922, 4456459, 4521996, 4587533,
                    4096014, 4259855, 4161552, 4194321, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 5210116, 4030472, 4325385,
                    4390922, 4456459, 4521996, 4587533, 4096014, 4259855, 4161552,
                    4194321, 131091, 163860, 4653078, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 3276808, 3571721,
                    3637258, 3702795, 3768332, 3833869, 3342350, 3506191, 3407888,
                    3440657, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 5308428, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 5373956, 4030472,
                    4325385, 4390922, 4456459, 4521996, 4587533, 4096014, 4259855,
                    4161552, 4194321, 131091, 163860, 4653078, 2147745815, 2147778584,
                    2147811353, 2147844122, 753664, 917506, 1015813, 1048594, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 3276808,
                    3571721, 3637258, 3702795, 3768332, 3833869, 3342350, 3506191,
                    3407888, 3440657, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 5472262, 3276808, 3571721, 3637258, 3702795, 3768332,
                    3833869, 3342350, 3506191, 3407888, 3440657, 131091, 163860, 3899414,
                    2147745815, 2147778584, 2147811353, 2147844122, 2031616, 2260994,
                    2359301, 2457618, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 5537799, 2588680, 2883593, 2949130, 3014667, 3080204,
                    3145741, 2654222, 2818063, 2719760, 2752529, 131091, 163860, 3211286,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1146888, 1507337, 1572874, 1638411, 1703948, 1769485,
                    1212430, 1441807, 1277968, 1376273, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 1146888, 1507337, 1572874,
                    1638411, 1703948, 1769485, 1212430, 1441807, 1277968, 1376273,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    5668867, 458752, 589826, 884741, 2162706, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 4030472, 4325385, 4390922,
                    4456459, 4521996, 4587533, 4096014, 4259855, 4161552, 4194321,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    5767172, 4030472, 4325385, 4390922, 4456459, 4521996, 4587533,
                    4096014, 4259855, 4161552, 4194321, 131091, 163860, 4653078,
                    2147745815, 2147778584, 2147811353, 2147844122, 458752, 589826,
                    884741, 2162706, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 4030472, 4325385, 4390922, 4456459, 4521996, 4587533,
                    4096014, 4259855, 4161552, 4194321, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 5865484, 458752, 589826, 884741,
                    2162706, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 5931012, 4030472, 4325385, 4390922, 4456459, 4521996,
                    4587533, 4096014, 4259855, 4161552, 4194321, 131091, 163860, 4653078,
                    2147745815, 2147778584, 2147811353, 2147844122, 458752, 589826,
                    884741, 2162706, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 4030472, 4325385, 4390922, 4456459, 4521996, 4587533,
                    4096014, 4259855, 4161552, 4194321, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 6029316, 4030472, 4325385,
                    4390922, 4456459, 4521996, 4587533, 4096014, 4259855, 4161552,
                    4194321, 131091, 163860, 4653078, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 1146888, 1507337,
                    1572874, 1638411, 1703948, 1769485, 1212430, 1441807, 1277968,
                    1376273, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 6127628, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6193156, 4030472,
                    4325385, 4390922, 4456459, 4521996, 4587533, 4096014, 4259855,
                    4161552, 4194321, 131091, 163860, 4653078, 2147745815, 2147778584,
                    2147811353, 2147844122, 196608, 622594, 720901, 1081362, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 1146888,
                    1507337, 1572874, 1638411, 1703948, 1769485, 1212430, 1441807,
                    1277968, 1376273, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 6291468, 458752, 589826, 884741, 2162706, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6356996, 4030472,
                    4325385, 4390922, 4456459, 4521996, 4587533, 4096014, 4259855,
                    4161552, 4194321, 131091, 163860, 4653078, 2147745815, 2147778584,
                    2147811353, 2147844122, 65536, 6389762, 6488069, 6651922, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 6455299,
                    65536, 6389762, 6488069, 6651922, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 753664, 917506, 1015813, 1048594,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    6553606, 3276808, 3571721, 3637258, 3702795, 3768332, 3833869,
                    3342350, 3506191, 3407888, 3440657, 131091, 163860, 3899414,
                    2147745815, 2147778584, 2147811353, 2147844122, 2031616, 2260994,
                    2359301, 2457618, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 6619143, 2588680, 2883593, 2949130, 3014667, 3080204,
                    3145741, 2654222, 2818063, 2719760, 2752529, 131091, 163860, 3211286,
                    2147745815, 2147778584, 2147811353, 2147844122, 65536, 6389762,
                    6488069, 6651922, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6717448, 7012361,
                    7077898, 7143435, 7208972, 7274509, 6782990, 6946831, 6848528,
                    6881297, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6782990, 6946831,
                    6848528, 6881297, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6848528, 6881297,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    65536, 6389762, 6488069, 6651922, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 65536, 6389762, 6488069, 6651922,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    6848528, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6848528, 6881297,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    65536, 6389762, 6488069, 6651922, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 6782990, 6946831, 6848528,
                    6881297, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6782990, 6946831,
                    6848528, 6881297, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6782990, 6946831,
                    6848528, 6881297, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6782990, 6946831,
                    6848528, 6881297, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6782990, 6946831,
                    6848528, 6881297, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 6717448, 7012361, 7077898, 7143435, 7208972, 7274509,
                    6782990, 6946831, 6848528, 6881297, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 6717448, 7012361, 7077898,
                    7143435, 7208972, 7274509, 6782990, 6946831, 6848528, 6881297,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    7438348, 458752, 589826, 884741, 2162706, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 7503876, 4030472, 4325385,
                    4390922, 4456459, 4521996, 4587533, 4096014, 4259855, 4161552,
                    4194321, 131091, 163860, 4653078, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 6389762, 6488069, 6651922, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 6717448, 7012361,
                    7077898, 7143435, 7208972, 7274509, 6782990, 6946831, 6848528,
                    6881297, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 6717448, 7012361, 7077898, 7143435, 7208972, 7274509,
                    6782990, 6946831, 6848528, 6881297, 131091, 163860, 7602198,
                    2147745815, 2147778584, 2147811353, 2147844122, 2155151389, 65536,
                    6389762, 6488069, 6651922, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 6717448, 7012361, 7077898, 7143435, 7208972,
                    7274509, 6782990, 6946831, 6848528, 6881297, 131091, 163860, 7602198,
                    2147745815, 2147778584, 2147811353, 2147844122,
                ];
                static SHIFT_TERM_OFFSETS: &[u32] = &[
                    0, 1, 11, 18, 24, 24, 34, 41, 47, 47, 47, 47, 47, 48, 58, 65, 71, 72,
                    82, 88, 94, 95, 105, 115, 122, 128, 129, 139, 149, 155, 156, 166,
                    176, 186, 196, 214, 224, 234, 244, 252, 262, 268, 268, 278, 285, 295,
                    303, 313, 323, 333, 343, 353, 363, 373, 383, 393, 403, 403, 413, 430,
                    436, 454, 464, 471, 477, 478, 488, 498, 516, 526, 532, 533, 543, 553,
                    571, 581, 591, 609, 619, 635, 645, 655, 665, 673, 683, 693, 700, 710,
                    718, 728, 738, 748, 758, 768, 778, 788, 798, 808, 818, 828, 845, 855,
                    865, 875, 883, 893, 903, 910, 920, 928, 938, 948, 958, 968, 978, 988,
                    998, 1008, 1018, 1028, 1038, 1055, 1071, 1087, 1097, 1107, 1117,
                    1125, 1135, 1145, 1152, 1162, 1170, 1180, 1190, 1200, 1210, 1220,
                    1230, 1240, 1250, 1260, 1270, 1280, 1297, 1298, 1308, 1326, 1336,
                    1352, 1370, 1380, 1396, 1412, 1430, 1440, 1458, 1468, 1484, 1502,
                    1512, 1528, 1529, 1539, 1557, 1567, 1583, 1601, 1611, 1629, 1639,
                    1655, 1671, 1672, 1682, 1698, 1716, 1726, 1742, 1743, 1753, 1771,
                    1781, 1797, 1815, 1825, 1841, 1842, 1852, 1870, 1880, 1896, 1897,
                    1907, 1925, 1935, 1941, 1942, 1952, 1962, 1980, 1990, 2008, 2018,
                    2028, 2044, 2054, 2064, 2074, 2082, 2092, 2102, 2109, 2119, 2127,
                    2137, 2147, 2157, 2167, 2177, 2187, 2197, 2207, 2217, 2227, 2243,
                    2259, 2260, 2270, 2288, 2298, 2314, 2332, 2342, 2359, 2359,
                ];
                static SHIFT_NONTERM_DATA: &[u32] = &[
                    2155053056, 2155053057, 2154889216, 2153742336, 2148597760,
                    2148597761, 2153578496, 2147876864, 2153480192, 2153480193,
                    2153316352, 2148007936, 2153218048, 2153218049, 2153119744,
                    2148139008, 2153086976, 2153086977, 2152923136, 2152923137,
                    2152759296, 2148302848, 2152660992, 2152660993, 2152497152,
                    2152497153, 2148433920, 2152464384, 2152464385, 2149449728,
                    2149449729, 2149416960, 2149416961, 2149416960, 2149416961,
                    2148827136, 2148663296, 2148663297, 2148827136, 2148728832,
                    2148728833, 2148827136, 2148794368, 2148794369, 2148827136,
                    2148892672, 2148892673, 2148827136, 2148958208, 2148958209,
                    2148827136, 2149023744, 2149023745, 2148827136, 2149089280,
                    2149089281, 2148827136, 2149154816, 2149154817, 2148827136,
                    2149220352, 2149220353, 2148827136, 2149285888, 2149285889,
                    2148827136, 2149384192, 2149384193, 2148827136, 2148827136,
                    2148827136, 2152366080, 2152366081, 2152202240, 2149580800,
                    2149679104, 2149679105, 2149416960, 2149416961, 2148827136,
                    2151481344, 2151481345, 2149777408, 2151448576, 2151448577,
                    2149875712, 2149875713, 2148827136, 2149974016, 2149974017,
                    2149416960, 2149416961, 2148827136, 2150039552, 2150039553,
                    2148827136, 2150105088, 2150105089, 2148827136, 2150170624,
                    2150170625, 2148827136, 2148794368, 2148794369, 2150268928,
                    2150268929, 2148827136, 2150334464, 2150334465, 2148827136,
                    2150400000, 2150400001, 2148827136, 2150465536, 2150465537,
                    2148827136, 2150531072, 2150531073, 2148827136, 2150596608,
                    2150596609, 2148827136, 2150662144, 2150662145, 2148827136,
                    2150727680, 2150727681, 2148827136, 2150793216, 2150793217,
                    2148827136, 2150858752, 2150858753, 2148827136, 2148794368,
                    2148794369, 2150957056, 2150957057, 2148827136, 2151022592,
                    2151022593, 2148827136, 2151088128, 2151088129, 2148827136,
                    2151153664, 2151153665, 2148827136, 2151219200, 2151219201,
                    2148827136, 2151284736, 2151284737, 2148827136, 2151350272,
                    2151350273, 2148827136, 2151415808, 2151415809, 2148827136,
                    2148827136, 2148827136, 2151546880, 2151546881, 2148827136,
                    2151612416, 2151612417, 2148827136, 2148794368, 2148794369,
                    2151710720, 2151710721, 2148827136, 2151776256, 2151776257,
                    2148827136, 2151841792, 2151841793, 2148827136, 2151907328,
                    2151907329, 2148827136, 2151972864, 2151972865, 2148827136,
                    2152038400, 2152038401, 2148827136, 2152103936, 2152103937,
                    2148827136, 2152169472, 2152169473, 2148827136, 2152267776,
                    2152267777, 2148827136, 2152333312, 2152333313, 2148827136,
                    2148827136, 2152431616, 2152431617, 2148827136, 2148827136,
                    2148827136, 2152562688, 2152562689, 2148827136, 2152628224,
                    2152628225, 2148827136, 2148827136, 2152726528, 2152726529,
                    2148827136, 2152824832, 2152824833, 2148827136, 2152890368,
                    2152890369, 2148827136, 2148827136, 2152988672, 2152988673,
                    2148827136, 2153054208, 2153054209, 2148827136, 2148827136,
                    2153185280, 2153185281, 2148827136, 2148827136, 2153283584,
                    2153283585, 2148827136, 2153381888, 2153381889, 2148827136,
                    2153447424, 2153447425, 2148827136, 2148827136, 2153545728,
                    2153545729, 2148827136, 2153644032, 2153644033, 2148827136,
                    2153709568, 2153709569, 2148827136, 2153807872, 2153807873,
                    2148827136, 2154856448, 2154856449, 2153906176, 2154823680,
                    2154823681, 2154004480, 2154004481, 2148827136, 2154070016,
                    2154070017, 2148827136, 2154168320, 2154168321, 2149416960,
                    2149416961, 2148827136, 2154233856, 2154233857, 2148827136,
                    2154299392, 2154299393, 2148827136, 2148794368, 2148794369,
                    2154397696, 2154397697, 2148827136, 2154463232, 2154463233,
                    2148827136, 2154528768, 2154528769, 2148827136, 2154594304,
                    2154594305, 2148827136, 2154659840, 2154659841, 2148827136,
                    2154725376, 2154725377, 2148827136, 2154790912, 2154790913,
                    2148827136, 2148827136, 2148827136, 2154954752, 2154954753,
                    2148827136, 2155020288, 2155020289, 2148827136, 2148827136,
                    2155118592, 2155118593, 2148827136,
                ];
                static SHIFT_NONTERM_OFFSETS: &[u32] = &[
                    0, 0, 2, 3, 4, 4, 6, 7, 8, 8, 8, 8, 8, 8, 10, 11, 12, 12, 14, 15, 16,
                    16, 18, 20, 21, 22, 22, 24, 26, 27, 27, 29, 31, 33, 35, 36, 38, 39,
                    41, 42, 44, 45, 45, 47, 48, 50, 51, 53, 54, 56, 57, 59, 60, 62, 63,
                    65, 66, 66, 68, 69, 70, 71, 73, 74, 75, 75, 77, 79, 80, 82, 83, 83,
                    85, 87, 88, 90, 92, 93, 95, 96, 98, 99, 101, 102, 104, 106, 107, 109,
                    110, 112, 113, 115, 116, 118, 119, 121, 122, 124, 125, 127, 128, 130,
                    131, 133, 134, 136, 138, 139, 141, 142, 144, 145, 147, 148, 150, 151,
                    153, 154, 156, 157, 159, 160, 161, 162, 164, 165, 167, 168, 170, 172,
                    173, 175, 176, 178, 179, 181, 182, 184, 185, 187, 188, 190, 191, 193,
                    194, 194, 196, 197, 199, 200, 201, 203, 204, 205, 206, 208, 209, 211,
                    212, 213, 215, 216, 216, 218, 219, 221, 222, 223, 225, 226, 228, 229,
                    230, 230, 232, 233, 234, 236, 237, 237, 239, 240, 242, 243, 244, 246,
                    247, 247, 249, 250, 252, 253, 253, 255, 256, 258, 259, 259, 261, 263,
                    264, 266, 267, 269, 271, 272, 274, 275, 277, 278, 280, 282, 283, 285,
                    286, 288, 289, 291, 292, 294, 295, 297, 298, 300, 301, 302, 303, 303,
                    305, 306, 308, 309, 310, 312, 313, 313,
                ];
                static REDUCE_DATA: &[u32] = &[
                    3, 1, 4, 4, 1, 4, 6, 1, 4, 7, 1, 4, 8, 1, 4, 9, 1, 4, 10, 1, 4, 11,
                    1, 4, 12, 1, 4, 13, 1, 4, 14, 1, 4, 15, 1, 4, 16, 1, 4, 17, 1, 4, 19,
                    1, 4, 20, 1, 4, 21, 1, 4, 22, 1, 4, 23, 1, 4, 24, 1, 4, 25, 1, 4, 26,
                    1, 4, 29, 1, 4, 3, 1, 2, 4, 1, 2, 6, 1, 2, 7, 1, 2, 8, 1, 2, 9, 1, 2,
                    10, 1, 2, 11, 1, 2, 12, 1, 2, 13, 1, 2, 14, 1, 2, 15, 1, 2, 16, 1, 2,
                    17, 1, 2, 19, 1, 2, 20, 1, 2, 21, 1, 2, 22, 1, 2, 23, 1, 2, 24, 1, 2,
                    25, 1, 2, 26, 1, 2, 29, 1, 2, 3, 1, 0, 4, 1, 0, 6, 1, 0, 7, 1, 0, 8,
                    1, 0, 9, 1, 0, 10, 1, 0, 11, 1, 0, 12, 1, 0, 13, 1, 0, 14, 1, 0, 15,
                    1, 0, 16, 1, 0, 17, 1, 0, 19, 1, 0, 20, 1, 0, 21, 1, 0, 22, 1, 0, 23,
                    1, 0, 24, 1, 0, 25, 1, 0, 26, 1, 0, 29, 1, 0, 3, 1, 1, 4, 1, 1, 6, 1,
                    1, 7, 1, 1, 8, 1, 1, 9, 1, 1, 10, 1, 1, 11, 1, 1, 12, 1, 1, 13, 1, 1,
                    14, 1, 1, 15, 1, 1, 16, 1, 1, 17, 1, 1, 19, 1, 1, 20, 1, 1, 21, 1, 1,
                    22, 1, 1, 23, 1, 1, 24, 1, 1, 25, 1, 1, 26, 1, 1, 29, 1, 1, 3, 1, 3,
                    4, 1, 3, 6, 1, 3, 7, 1, 3, 8, 1, 3, 9, 1, 3, 10, 1, 3, 11, 1, 3, 12,
                    1, 3, 13, 1, 3, 14, 1, 3, 15, 1, 3, 16, 1, 3, 17, 1, 3, 19, 1, 3, 20,
                    1, 3, 21, 1, 3, 22, 1, 3, 23, 1, 3, 24, 1, 3, 25, 1, 3, 26, 1, 3, 29,
                    1, 3, 8, 1, 8, 9, 1, 8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 13, 1, 8, 21,
                    1, 8, 22, 1, 8, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11, 1, 6, 12, 1, 6, 13,
                    1, 6, 14, 1, 6, 15, 1, 6, 21, 1, 6, 22, 1, 6, 4, 1, 14, 6, 1, 14, 7,
                    1, 14, 8, 1, 14, 9, 1, 14, 10, 1, 14, 11, 1, 14, 12, 1, 14, 13, 1,
                    14, 14, 1, 14, 15, 1, 14, 16, 1, 14, 17, 1, 14, 21, 1, 14, 22, 1, 14,
                    29, 1, 14, 4, 1, 23, 6, 1, 23, 7, 1, 23, 8, 1, 23, 9, 1, 23, 10, 1,
                    23, 11, 1, 23, 12, 1, 23, 13, 1, 23, 14, 1, 23, 15, 1, 23, 16, 1, 23,
                    17, 1, 23, 19, 1, 23, 20, 1, 23, 21, 1, 23, 22, 1, 23, 23, 1, 23, 24,
                    1, 23, 25, 1, 23, 26, 1, 23, 29, 1, 23, 8, 1, 15, 9, 1, 15, 10, 1,
                    15, 11, 1, 15, 12, 1, 15, 13, 1, 15, 14, 1, 15, 15, 1, 15, 17, 1, 15,
                    21, 1, 15, 22, 1, 15, 8, 1, 7, 9, 1, 7, 10, 1, 7, 11, 1, 7, 12, 1, 7,
                    13, 1, 7, 14, 1, 7, 15, 1, 7, 21, 1, 7, 22, 1, 7, 8, 1, 9, 9, 1, 9,
                    10, 1, 9, 11, 1, 9, 12, 1, 9, 13, 1, 9, 21, 1, 9, 22, 1, 9, 8, 1, 10,
                    9, 1, 10, 10, 1, 10, 11, 1, 10, 12, 1, 10, 13, 1, 10, 21, 1, 10, 22,
                    1, 10, 8, 1, 11, 9, 1, 11, 10, 1, 11, 11, 1, 11, 12, 1, 11, 13, 1,
                    11, 21, 1, 11, 22, 1, 11, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12,
                    12, 1, 12, 13, 1, 12, 21, 1, 12, 22, 1, 12, 8, 1, 13, 9, 1, 13, 10,
                    1, 13, 11, 1, 13, 12, 1, 13, 13, 1, 13, 21, 1, 13, 22, 1, 13, 3, 1,
                    5, 4, 1, 5, 6, 1, 5, 7, 1, 5, 8, 1, 5, 9, 1, 5, 10, 1, 5, 11, 1, 5,
                    12, 1, 5, 13, 1, 5, 14, 1, 5, 15, 1, 5, 16, 1, 5, 17, 1, 5, 19, 1, 5,
                    20, 1, 5, 21, 1, 5, 22, 1, 5, 23, 1, 5, 24, 1, 5, 25, 1, 5, 26, 1, 5,
                    29, 1, 5, 21, 1, 21, 4, 1, 16, 6, 1, 16, 7, 1, 16, 8, 1, 16, 9, 1,
                    16, 10, 1, 16, 11, 1, 16, 12, 1, 16, 13, 1, 16, 14, 1, 16, 15, 1, 16,
                    16, 1, 16, 17, 1, 16, 21, 1, 16, 22, 1, 16, 29, 1, 16, 7, 1, 20, 22,
                    1, 20, 7, 1, 8, 8, 1, 8, 9, 1, 8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 13,
                    1, 8, 22, 1, 8, 7, 1, 6, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11, 1, 6, 12, 1,
                    6, 13, 1, 6, 14, 1, 6, 15, 1, 6, 22, 1, 6, 7, 1, 15, 8, 1, 15, 9, 1,
                    15, 10, 1, 15, 11, 1, 15, 12, 1, 15, 13, 1, 15, 14, 1, 15, 15, 1, 15,
                    17, 1, 15, 22, 1, 15, 7, 1, 7, 8, 1, 7, 9, 1, 7, 10, 1, 7, 11, 1, 7,
                    12, 1, 7, 13, 1, 7, 14, 1, 7, 15, 1, 7, 22, 1, 7, 7, 1, 9, 8, 1, 9,
                    9, 1, 9, 10, 1, 9, 11, 1, 9, 12, 1, 9, 13, 1, 9, 22, 1, 9, 7, 1, 10,
                    8, 1, 10, 9, 1, 10, 10, 1, 10, 11, 1, 10, 12, 1, 10, 13, 1, 10, 22,
                    1, 10, 7, 1, 11, 8, 1, 11, 9, 1, 11, 10, 1, 11, 11, 1, 11, 12, 1, 11,
                    13, 1, 11, 22, 1, 11, 7, 1, 12, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11, 1,
                    12, 12, 1, 12, 13, 1, 12, 22, 1, 12, 7, 1, 13, 8, 1, 13, 9, 1, 13,
                    10, 1, 13, 11, 1, 13, 12, 1, 13, 13, 1, 13, 22, 1, 13, 7, 1, 21, 6,
                    1, 8, 8, 1, 8, 9, 1, 8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 13, 1, 8, 22,
                    1, 8, 6, 1, 6, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11, 1, 6, 12, 1, 6, 13, 1,
                    6, 14, 1, 6, 15, 1, 6, 22, 1, 6, 6, 1, 15, 8, 1, 15, 9, 1, 15, 10, 1,
                    15, 11, 1, 15, 12, 1, 15, 13, 1, 15, 14, 1, 15, 15, 1, 15, 17, 1, 15,
                    22, 1, 15, 6, 1, 7, 8, 1, 7, 9, 1, 7, 10, 1, 7, 11, 1, 7, 12, 1, 7,
                    13, 1, 7, 14, 1, 7, 15, 1, 7, 22, 1, 7, 6, 1, 9, 8, 1, 9, 9, 1, 9,
                    10, 1, 9, 11, 1, 9, 12, 1, 9, 13, 1, 9, 22, 1, 9, 6, 1, 10, 8, 1, 10,
                    9, 1, 10, 10, 1, 10, 11, 1, 10, 12, 1, 10, 13, 1, 10, 22, 1, 10, 6,
                    1, 11, 8, 1, 11, 9, 1, 11, 10, 1, 11, 11, 1, 11, 12, 1, 11, 13, 1,
                    11, 22, 1, 11, 6, 1, 12, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12,
                    12, 1, 12, 13, 1, 12, 22, 1, 12, 6, 1, 13, 8, 1, 13, 9, 1, 13, 10, 1,
                    13, 11, 1, 13, 12, 1, 13, 13, 1, 13, 22, 1, 13, 6, 1, 21, 7, 1, 19,
                    22, 1, 19, 7, 1, 18, 22, 1, 18, 4, 1, 8, 8, 1, 8, 9, 1, 8, 10, 1, 8,
                    11, 1, 8, 12, 1, 8, 13, 1, 8, 22, 1, 8, 4, 1, 6, 8, 1, 6, 9, 1, 6,
                    10, 1, 6, 11, 1, 6, 12, 1, 6, 13, 1, 6, 14, 1, 6, 15, 1, 6, 22, 1, 6,
                    4, 1, 15, 8, 1, 15, 9, 1, 15, 10, 1, 15, 11, 1, 15, 12, 1, 15, 13, 1,
                    15, 14, 1, 15, 15, 1, 15, 17, 1, 15, 22, 1, 15, 4, 1, 7, 8, 1, 7, 9,
                    1, 7, 10, 1, 7, 11, 1, 7, 12, 1, 7, 13, 1, 7, 14, 1, 7, 15, 1, 7, 22,
                    1, 7, 4, 1, 9, 8, 1, 9, 9, 1, 9, 10, 1, 9, 11, 1, 9, 12, 1, 9, 13, 1,
                    9, 22, 1, 9, 4, 1, 10, 8, 1, 10, 9, 1, 10, 10, 1, 10, 11, 1, 10, 12,
                    1, 10, 13, 1, 10, 22, 1, 10, 4, 1, 11, 8, 1, 11, 9, 1, 11, 10, 1, 11,
                    11, 1, 11, 12, 1, 11, 13, 1, 11, 22, 1, 11, 4, 1, 12, 8, 1, 12, 9, 1,
                    12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 22, 1, 12, 4, 1, 13,
                    8, 1, 13, 9, 1, 13, 10, 1, 13, 11, 1, 13, 12, 1, 13, 13, 1, 13, 22,
                    1, 13, 4, 1, 21, 7, 1, 17, 22, 1, 17, 6, 1, 20, 22, 1, 20, 6, 1, 19,
                    22, 1, 19, 4, 1, 20, 22, 1, 20, 6, 1, 18, 22, 1, 18, 6, 1, 17, 22, 1,
                    17, 21, 1, 20, 22, 1, 20, 21, 1, 19, 22, 1, 19, 4, 1, 19, 22, 1, 19,
                    4, 1, 18, 22, 1, 18, 4, 1, 17, 22, 1, 17, 21, 1, 18, 22, 1, 18, 21,
                    1, 17, 22, 1, 17, 22, 1, 20, 29, 1, 20, 8, 1, 8, 9, 1, 8, 10, 1, 8,
                    11, 1, 8, 12, 1, 8, 13, 1, 8, 22, 1, 8, 29, 1, 8, 8, 1, 6, 9, 1, 6,
                    10, 1, 6, 11, 1, 6, 12, 1, 6, 13, 1, 6, 14, 1, 6, 15, 1, 6, 22, 1, 6,
                    29, 1, 6, 8, 1, 15, 9, 1, 15, 10, 1, 15, 11, 1, 15, 12, 1, 15, 13, 1,
                    15, 14, 1, 15, 15, 1, 15, 17, 1, 15, 22, 1, 15, 29, 1, 15, 8, 1, 7,
                    9, 1, 7, 10, 1, 7, 11, 1, 7, 12, 1, 7, 13, 1, 7, 14, 1, 7, 15, 1, 7,
                    22, 1, 7, 29, 1, 7, 8, 1, 9, 9, 1, 9, 10, 1, 9, 11, 1, 9, 12, 1, 9,
                    13, 1, 9, 22, 1, 9, 29, 1, 9, 8, 1, 10, 9, 1, 10, 10, 1, 10, 11, 1,
                    10, 12, 1, 10, 13, 1, 10, 22, 1, 10, 29, 1, 10, 8, 1, 11, 9, 1, 11,
                    10, 1, 11, 11, 1, 11, 12, 1, 11, 13, 1, 11, 22, 1, 11, 29, 1, 11, 8,
                    1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 22, 1,
                    12, 29, 1, 12, 8, 1, 13, 9, 1, 13, 10, 1, 13, 11, 1, 13, 12, 1, 13,
                    13, 1, 13, 22, 1, 13, 29, 1, 13, 22, 1, 19, 29, 1, 19, 22, 1, 18, 29,
                    1, 18, 22, 1, 17, 29, 1, 17, 29, 1, 21,
                ];
                static REDUCE_OFFSETS: &[u32] = &[
                    0, 0, 0, 0, 0, 69, 69, 69, 69, 138, 207, 276, 345, 345, 345, 345,
                    345, 345, 345, 345, 345, 345, 345, 345, 345, 345, 345, 345, 345, 345,
                    345, 345, 345, 345, 345, 345, 345, 369, 369, 399, 399, 447, 513, 513,
                    546, 546, 576, 576, 600, 600, 624, 624, 648, 648, 672, 672, 696, 765,
                    765, 768, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816, 816,
                    816, 816, 816, 816, 816, 816, 816, 822, 822, 846, 846, 876, 876, 876,
                    909, 909, 939, 939, 963, 963, 987, 987, 1011, 1011, 1035, 1035, 1059,
                    1059, 1062, 1062, 1086, 1086, 1116, 1116, 1116, 1149, 1149, 1179,
                    1179, 1203, 1203, 1227, 1227, 1251, 1251, 1275, 1275, 1299, 1299,
                    1302, 1308, 1314, 1314, 1338, 1338, 1368, 1368, 1368, 1401, 1401,
                    1431, 1431, 1455, 1455, 1479, 1479, 1503, 1503, 1527, 1527, 1551,
                    1551, 1554, 1554, 1554, 1554, 1554, 1560, 1560, 1560, 1566, 1572,
                    1572, 1572, 1572, 1572, 1578, 1578, 1578, 1584, 1584, 1584, 1584,
                    1584, 1590, 1590, 1590, 1590, 1590, 1596, 1602, 1602, 1602, 1608,
                    1608, 1608, 1614, 1614, 1614, 1614, 1614, 1620, 1620, 1620, 1626,
                    1626, 1626, 1626, 1626, 1632, 1632, 1632, 1632, 1632, 1632, 1632,
                    1632, 1632, 1632, 1632, 1632, 1632, 1632, 1638, 1638, 1662, 1662,
                    1692, 1692, 1692, 1725, 1725, 1755, 1755, 1779, 1779, 1803, 1803,
                    1827, 1827, 1851, 1851, 1875, 1881, 1887, 1887, 1887, 1887, 1887,
                    1893, 1893, 1893, 1896, 1896,
                ];
                let num_rules = 25usize;
                let mut rules = Vec::with_capacity(num_rules);
                for i in 0..num_rules {
                    let lhs = NonTerminals::from_usize(RULE_NAMES[i] as usize);
                    rules
                        .push(::rusty_lr::parser::table::RuleInfo {
                            lhs,
                            len: RULE_LENGTHS[i] as usize,
                        });
                }
                let num_states = 235usize;
                let mut state_rows = Vec::with_capacity(num_states);
                for i in 0..num_states {
                    let term_start = SHIFT_TERM_OFFSETS[i] as usize;
                    let term_end = SHIFT_TERM_OFFSETS[i + 1] as usize;
                    let mut shift_goto_map_term = Vec::with_capacity(
                        term_end - term_start,
                    );
                    for idx in term_start..term_end {
                        let val = SHIFT_TERM_DATA[idx];
                        let term_class = TerminalClasses::from_usize(
                            (val & 0x7fff) as usize,
                        );
                        let state = ((val >> 15) & 0xffff) as usize;
                        let push = (val >> 31) != 0;
                        shift_goto_map_term
                            .push((
                                term_class,
                                ::rusty_lr::parser::table::ShiftTarget::new(state, push),
                            ));
                    }
                    let nonterm_start = SHIFT_NONTERM_OFFSETS[i] as usize;
                    let nonterm_end = SHIFT_NONTERM_OFFSETS[i + 1] as usize;
                    let mut shift_goto_map_nonterm = Vec::with_capacity(
                        nonterm_end - nonterm_start,
                    );
                    for idx in nonterm_start..nonterm_end {
                        let val = SHIFT_NONTERM_DATA[idx];
                        let nonterm = NonTerminals::from_usize((val & 0x7fff) as usize);
                        let state = ((val >> 15) & 0xffff) as usize;
                        let push = (val >> 31) != 0;
                        shift_goto_map_nonterm
                            .push((
                                nonterm,
                                ::rusty_lr::parser::table::ShiftTarget::new(state, push),
                            ));
                    }
                    let reduce_start = REDUCE_OFFSETS[i] as usize;
                    let reduce_end = REDUCE_OFFSETS[i + 1] as usize;
                    let mut reduce_map = Vec::new();
                    let mut idx = reduce_start;
                    while idx < reduce_end {
                        let term_val = REDUCE_DATA[idx];
                        let term_class = TerminalClasses::from_usize(term_val as usize);
                        let len = REDUCE_DATA[idx + 1] as usize;
                        let mut rules = Vec::with_capacity(len);
                        for r_idx in 0..len {
                            rules.push(REDUCE_DATA[idx + 2 + r_idx] as usize);
                        }
                        reduce_map.push((term_class, rules));
                        idx += 2 + len;
                    }
                    let intermediate = ::rusty_lr::parser::state::IntermediateState {
                        shift_goto_map_term,
                        shift_goto_map_nonterm,
                        reduce_map,
                        ruleset: Vec::new(),
                    };
                    state_rows.push(intermediate);
                }
                ::rusty_lr::parser::table::IntermediateTables {
                    state_rows,
                    rules,
                }
                    .into()
            })
    }
    #[doc(hidden)]
    fn __rusty_lr_parser_version() -> (usize, usize, usize) {
        (4, 7, 0)
    }
    #[doc(hidden)]
    fn __rustylr_version() -> (usize, usize, usize) {
        (1, 38, 0)
    }
    #[doc(hidden)]
    fn __rusty_lr_version() -> (usize, usize, usize) {
        (4, 7, 0)
    }
}

// ==============================Generated Codes End===============================
