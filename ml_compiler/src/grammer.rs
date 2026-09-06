
// ================================User Codes Begin================================
use crate::enums::Token;
use crate::enums::Expr;

// =================================User Codes End=================================
/*
====================================Grammar=====================================

# of terminal classes: 28
# of states: 193

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
13: Expr -> Expr And Expr
14: Expr -> Expr Or Expr
15: Expr -> Not Expr
16: Expr -> Let ArgExpr Eq Expr In Expr
17: Expr -> Let Rec ArgExpr Eq Expr In Expr
18: Expr -> Fun ArgExpr Arrow Expr
19: Expr -> If Expr Then Expr Else Expr
20: Expr -> Expr Semi Expr
21: Expr -> ArgExpr
22: Expr -> Expr ArgExpr
23: Augmented -> VirtualStart(0) Expr eof

*/
// =============================Generated Codes Begin==============================
#[allow(non_camel_case_types, dead_code)]
pub type ExprContext = ::rusty_lr::parser::deterministic::Context<
    Parser,
    Data,
    ExprExtracter,
    u8,
>;
#[allow(non_camel_case_types, dead_code)]
pub type Rule = ::rusty_lr::production::Production<TerminalClasses, NonTerminals>;
#[allow(non_camel_case_types, dead_code)]
pub type Tables = ::rusty_lr::parser::table::DenseFlatTables<
    TerminalClasses,
    NonTerminals,
    u8,
    u8,
>;
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
    std::cmp::Ord
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
            value < 31usize, "Terminal class index {} is out of bounds (max {})", value,
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
    std::cmp::Ord
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
            value < 3usize, "Non-terminal index {} is out of bounds (max {})", value,
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
    ///Expr -> Expr And Expr
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
    fn reduce_Expr_16(
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
            22usize => {
                Self::reduce_Expr_16(
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
#[allow(unused_braces, unused_parens, unused_variables, non_snake_case, unused_mut)]
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
                    2,
                ];
                static RULE_LENGTHS: &[u32] = &[
                    1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 6, 7, 4, 6, 3, 1, 2,
                    3,
                ];
                static SHIFT_TERM_DATA: &[u32] = &[
                    2147516446, 65536, 5341186, 5439493, 5603346, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 98305, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 196608,
                    622594, 720901, 1081362, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 229377, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 425996, 458752, 589826, 884741,
                    2097170, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 491521, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 557068, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    688131, 196608, 622594, 720901, 1081362, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 753664, 917506, 1015813, 1048594,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    786433, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 851980, 458752, 589826, 884741, 2097170, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 983043, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1146888, 1245193, 1310730, 1376267, 1441804, 1507342,
                    1572879, 1638416, 1703953, 131091, 163860, 1769493, 1802262,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 196608, 622594, 720901, 1081362, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 1146888, 1245193,
                    1310730, 1376267, 1441804, 1507342, 1572879, 1638416, 1703953,
                    131091, 163860, 1802262, 2147745815, 2147778584, 2147811353,
                    2147844122, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1933318, 2883592, 2916361, 2949130, 2981899, 3014668,
                    3047438, 3080207, 3112976, 3145745, 131091, 163860, 3178518,
                    2147745815, 2147778584, 2147811353, 2147844122, 1966080, 2195458,
                    2293765, 2392082, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1998849, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 2064396, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2097170, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 2162692, 3309576, 3342345,
                    3375114, 3407883, 3440652, 3473422, 3506191, 3538960, 3571729,
                    131091, 163860, 3604502, 2147745815, 2147778584, 2147811353,
                    2147844122, 1966080, 2195458, 2293765, 2392082, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2260995, 1966080,
                    2195458, 2293765, 2392082, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 753664, 917506, 1015813, 1048594, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 2359302,
                    2883592, 2916361, 2949130, 2981899, 3014668, 3047438, 3080207,
                    3112976, 3145745, 131091, 163860, 3178518, 2147745815, 2147778584,
                    2147811353, 2147844122, 1966080, 2195458, 2293765, 2392082, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 1966080,
                    2195458, 2293765, 2392082, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 2457607, 2523144, 2555913, 2588682, 2621451,
                    2654220, 2686990, 2719759, 2752528, 2785297, 131091, 163860, 2818070,
                    2147745815, 2147778584, 2147811353, 2147844122, 1966080, 2195458,
                    2293765, 2392082, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2523144, 2555913, 2588682, 2621451, 2654220, 2686990,
                    2719759, 2752528, 2785297, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 1966080, 2195458, 2293765, 2392082, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 1966080,
                    2195458, 2293765, 2392082, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 1966080, 2195458, 2293765, 2392082, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 1966080,
                    2195458, 2293765, 2392082, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 1966080, 2195458, 2293765, 2392082, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 1966080,
                    2195458, 2293765, 2392082, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 1966080, 2195458, 2293765, 2392082, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 1966080,
                    2195458, 2293765, 2392082, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 1966080, 2195458, 2293765, 2392082, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 1966080,
                    2195458, 2293765, 2392082, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 2523144, 2555913, 2588682, 2621451, 2654220,
                    2686990, 2719759, 2752528, 2785297, 131091, 163860, 2818070,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 753664, 917506, 1015813, 1048594, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2883592, 2916361,
                    2949130, 2981899, 3014668, 3047438, 3080207, 3112976, 3145745,
                    131091, 163860, 3178518, 2147745815, 2147778584, 2147811353,
                    2147844122, 2523144, 2555913, 2588682, 2621451, 2654220, 2686990,
                    2719759, 2752528, 2785297, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 2523144, 2555913, 2588682, 2621451, 2654220,
                    2686990, 2719759, 2752528, 2785297, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2097170, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2097170, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2097170, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2097170, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    458752, 589826, 884741, 2097170, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 3309576, 3342345, 3375114,
                    3407883, 3440652, 3473422, 3506191, 3538960, 3571729, 131091, 163860,
                    3604502, 2147745815, 2147778584, 2147811353, 2147844122, 3702796,
                    458752, 589826, 884741, 2097170, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 3768324, 3309576, 3342345,
                    3375114, 3407883, 3440652, 3473422, 3506191, 3538960, 3571729,
                    131091, 163860, 3604502, 2147745815, 2147778584, 2147811353,
                    2147844122, 1966080, 2195458, 2293765, 2392082, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 2523144, 2555913,
                    2588682, 2621451, 2654220, 2686990, 2719759, 2752528, 2785297,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    3866631, 2523144, 2555913, 2588682, 2621451, 2654220, 2686990,
                    2719759, 2752528, 2785297, 131091, 163860, 2818070, 2147745815,
                    2147778584, 2147811353, 2147844122, 753664, 917506, 1015813, 1048594,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    2883592, 2916361, 2949130, 2981899, 3014668, 3047438, 3080207,
                    3112976, 3145745, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2883592, 2916361, 2949130, 2981899, 3014668, 3047438,
                    3080207, 3112976, 3145745, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 3997702, 2883592, 2916361, 2949130, 2981899,
                    3014668, 3047438, 3080207, 3112976, 3145745, 131091, 163860, 3178518,
                    2147745815, 2147778584, 2147811353, 2147844122, 1966080, 2195458,
                    2293765, 2392082, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 4063239, 2523144, 2555913, 2588682, 2621451, 2654220,
                    2686990, 2719759, 2752528, 2785297, 131091, 163860, 2818070,
                    2147745815, 2147778584, 2147811353, 2147844122, 458752, 589826,
                    884741, 2097170, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 3309576, 3342345, 3375114, 3407883, 3440652, 3473422,
                    3506191, 3538960, 3571729, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 4161540, 3309576, 3342345, 3375114, 3407883,
                    3440652, 3473422, 3506191, 3538960, 3571729, 131091, 163860, 3604502,
                    2147745815, 2147778584, 2147811353, 2147844122, 753664, 917506,
                    1015813, 1048594, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 2883592, 2916361, 2949130, 2981899, 3014668, 3047438,
                    3080207, 3112976, 3145745, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 4259852, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    4325380, 3309576, 3342345, 3375114, 3407883, 3440652, 3473422,
                    3506191, 3538960, 3571729, 131091, 163860, 3604502, 2147745815,
                    2147778584, 2147811353, 2147844122, 753664, 917506, 1015813, 1048594,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    2883592, 2916361, 2949130, 2981899, 3014668, 3047438, 3080207,
                    3112976, 3145745, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 4423686, 2883592, 2916361, 2949130, 2981899, 3014668,
                    3047438, 3080207, 3112976, 3145745, 131091, 163860, 3178518,
                    2147745815, 2147778584, 2147811353, 2147844122, 1966080, 2195458,
                    2293765, 2392082, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 4489223, 2523144, 2555913, 2588682, 2621451, 2654220,
                    2686990, 2719759, 2752528, 2785297, 131091, 163860, 2818070,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1146888, 1245193, 1310730, 1376267, 1441804, 1507342,
                    1572879, 1638416, 1703953, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 1146888, 1245193, 1310730, 1376267, 1441804,
                    1507342, 1572879, 1638416, 1703953, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 4620291, 458752, 589826, 884741,
                    2097170, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 3309576, 3342345, 3375114, 3407883, 3440652, 3473422,
                    3506191, 3538960, 3571729, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 4718596, 3309576, 3342345, 3375114, 3407883,
                    3440652, 3473422, 3506191, 3538960, 3571729, 131091, 163860, 3604502,
                    2147745815, 2147778584, 2147811353, 2147844122, 458752, 589826,
                    884741, 2097170, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 3309576, 3342345, 3375114, 3407883, 3440652, 3473422,
                    3506191, 3538960, 3571729, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 4816908, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    4882436, 3309576, 3342345, 3375114, 3407883, 3440652, 3473422,
                    3506191, 3538960, 3571729, 131091, 163860, 3604502, 2147745815,
                    2147778584, 2147811353, 2147844122, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    3309576, 3342345, 3375114, 3407883, 3440652, 3473422, 3506191,
                    3538960, 3571729, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 4980740, 3309576, 3342345, 3375114, 3407883, 3440652,
                    3473422, 3506191, 3538960, 3571729, 131091, 163860, 3604502,
                    2147745815, 2147778584, 2147811353, 2147844122, 196608, 622594,
                    720901, 1081362, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 1146888, 1245193, 1310730, 1376267, 1441804, 1507342,
                    1572879, 1638416, 1703953, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 5079052, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    5144580, 3309576, 3342345, 3375114, 3407883, 3440652, 3473422,
                    3506191, 3538960, 3571729, 131091, 163860, 3604502, 2147745815,
                    2147778584, 2147811353, 2147844122, 196608, 622594, 720901, 1081362,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    1146888, 1245193, 1310730, 1376267, 1441804, 1507342, 1572879,
                    1638416, 1703953, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 5242892, 458752, 589826, 884741, 2097170, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 5308420, 3309576,
                    3342345, 3375114, 3407883, 3440652, 3473422, 3506191, 3538960,
                    3571729, 131091, 163860, 3604502, 2147745815, 2147778584, 2147811353,
                    2147844122, 65536, 5341186, 5439493, 5603346, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 131091, 163860,
                    2147745815, 2147778584, 2147811353, 2147844122, 5406723, 65536,
                    5341186, 5439493, 5603346, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 753664, 917506, 1015813, 1048594, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 5505030,
                    2883592, 2916361, 2949130, 2981899, 3014668, 3047438, 3080207,
                    3112976, 3145745, 131091, 163860, 3178518, 2147745815, 2147778584,
                    2147811353, 2147844122, 1966080, 2195458, 2293765, 2392082, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 5570567,
                    2523144, 2555913, 2588682, 2621451, 2654220, 2686990, 2719759,
                    2752528, 2785297, 131091, 163860, 2818070, 2147745815, 2147778584,
                    2147811353, 2147844122, 65536, 5341186, 5439493, 5603346, 131091,
                    163860, 2147745815, 2147778584, 2147811353, 2147844122, 65536,
                    5341186, 5439493, 5603346, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 5668872, 5701641, 5734410, 5767179, 5799948,
                    5832718, 5865487, 5898256, 5931025, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 65536, 5341186, 5439493, 5603346,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    65536, 5341186, 5439493, 5603346, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 65536, 5341186, 5439493, 5603346,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    65536, 5341186, 5439493, 5603346, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 65536, 5341186, 5439493, 5603346,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    65536, 5341186, 5439493, 5603346, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 65536, 5341186, 5439493, 5603346,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    65536, 5341186, 5439493, 5603346, 131091, 163860, 2147745815,
                    2147778584, 2147811353, 2147844122, 65536, 5341186, 5439493, 5603346,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    5668872, 5701641, 5734410, 5767179, 5799948, 5832718, 5865487,
                    5898256, 5931025, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 5668872, 5701641, 5734410, 5767179, 5799948, 5832718,
                    5865487, 5898256, 5931025, 131091, 163860, 2147745815, 2147778584,
                    2147811353, 2147844122, 6062092, 458752, 589826, 884741, 2097170,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    6127620, 3309576, 3342345, 3375114, 3407883, 3440652, 3473422,
                    3506191, 3538960, 3571729, 131091, 163860, 3604502, 2147745815,
                    2147778584, 2147811353, 2147844122, 65536, 5341186, 5439493, 5603346,
                    131091, 163860, 2147745815, 2147778584, 2147811353, 2147844122,
                    5668872, 5701641, 5734410, 5767179, 5799948, 5832718, 5865487,
                    5898256, 5931025, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 5668872, 5701641, 5734410, 5767179, 5799948, 5832718,
                    5865487, 5898256, 5931025, 131091, 163860, 6225942, 2147745815,
                    2147778584, 2147811353, 2147844122, 2153775133, 65536, 5341186,
                    5439493, 5603346, 131091, 163860, 2147745815, 2147778584, 2147811353,
                    2147844122, 5668872, 5701641, 5734410, 5767179, 5799948, 5832718,
                    5865487, 5898256, 5931025, 131091, 163860, 6225942, 2147745815,
                    2147778584, 2147811353, 2147844122,
                ];
                static SHIFT_TERM_OFFSETS: &[u32] = &[
                    0, 1, 11, 18, 24, 24, 34, 41, 47, 47, 47, 47, 47, 48, 58, 65, 71, 72,
                    82, 88, 94, 95, 105, 115, 122, 128, 129, 139, 149, 155, 156, 166,
                    176, 186, 196, 213, 223, 229, 229, 239, 245, 255, 261, 271, 277, 287,
                    293, 303, 309, 319, 325, 335, 341, 351, 357, 357, 367, 383, 389, 406,
                    416, 423, 429, 430, 440, 450, 467, 477, 483, 484, 494, 504, 521, 531,
                    541, 558, 568, 583, 593, 603, 613, 623, 633, 643, 653, 663, 673, 683,
                    699, 709, 719, 729, 739, 749, 759, 769, 779, 789, 799, 815, 830, 845,
                    855, 865, 875, 885, 895, 905, 915, 925, 935, 945, 961, 962, 972, 989,
                    999, 1014, 1031, 1041, 1056, 1071, 1088, 1098, 1115, 1125, 1140,
                    1157, 1167, 1182, 1183, 1193, 1210, 1220, 1235, 1252, 1262, 1279,
                    1289, 1304, 1319, 1320, 1330, 1345, 1362, 1372, 1387, 1388, 1398,
                    1415, 1425, 1440, 1457, 1467, 1482, 1483, 1493, 1510, 1520, 1535,
                    1536, 1546, 1563, 1573, 1579, 1580, 1590, 1600, 1617, 1627, 1644,
                    1654, 1664, 1679, 1689, 1699, 1709, 1719, 1729, 1739, 1749, 1759,
                    1769, 1784, 1799, 1800, 1810, 1827, 1837, 1852, 1869, 1879, 1895,
                    1895,
                ];
                static SHIFT_NONTERM_DATA: &[u32] = &[
                    2153676800, 2153676801, 2153512960, 2152693760, 2148597760,
                    2148597761, 2152529920, 2147876864, 2152431616, 2152431617,
                    2152267776, 2148007936, 2152169472, 2152169473, 2152071168,
                    2148139008, 2152038400, 2152038401, 2151874560, 2151874561,
                    2151710720, 2148302848, 2151612416, 2151612417, 2151448576,
                    2151448577, 2148433920, 2151415808, 2151415809, 2149384192,
                    2149384193, 2149351424, 2149351425, 2149351424, 2149351425,
                    2148696064, 2148663296, 2148663297, 2148696064, 2148761600,
                    2148761601, 2148696064, 2148827136, 2148827137, 2148696064,
                    2148892672, 2148892673, 2148696064, 2148958208, 2148958209,
                    2148696064, 2149023744, 2149023745, 2148696064, 2149089280,
                    2149089281, 2148696064, 2149154816, 2149154817, 2148696064,
                    2149220352, 2149220353, 2148696064, 2149318656, 2149318657,
                    2148696064, 2148696064, 2148696064, 2151317504, 2151317505,
                    2151153664, 2149515264, 2149613568, 2149613569, 2149351424,
                    2149351425, 2148696064, 2150760448, 2150760449, 2149711872,
                    2150727680, 2150727681, 2149810176, 2149810177, 2148696064,
                    2149908480, 2149908481, 2149351424, 2149351425, 2148696064,
                    2149974016, 2149974017, 2148696064, 2148663296, 2148663297,
                    2148761600, 2148761601, 2148827136, 2148827137, 2148892672,
                    2148892673, 2148958208, 2148958209, 2149023744, 2149023745,
                    2149089280, 2149089281, 2149154816, 2149154817, 2149220352,
                    2149220353, 2150334464, 2150334465, 2148696064, 2148663296,
                    2148663297, 2148761600, 2148761601, 2148827136, 2148827137,
                    2148892672, 2148892673, 2148958208, 2148958209, 2149023744,
                    2149023745, 2149089280, 2149089281, 2149154816, 2149154817,
                    2149220352, 2149220353, 2150694912, 2150694913, 2148696064,
                    2148696064, 2148696064, 2148663296, 2148663297, 2148761600,
                    2148761601, 2148827136, 2148827137, 2148892672, 2148892673,
                    2148958208, 2148958209, 2149023744, 2149023745, 2149089280,
                    2149089281, 2149154816, 2149154817, 2149220352, 2149220353,
                    2151120896, 2151120897, 2148696064, 2151219200, 2151219201,
                    2148696064, 2151284736, 2151284737, 2148696064, 2148696064,
                    2151383040, 2151383041, 2148696064, 2148696064, 2148696064,
                    2151514112, 2151514113, 2148696064, 2151579648, 2151579649,
                    2148696064, 2148696064, 2151677952, 2151677953, 2148696064,
                    2151776256, 2151776257, 2148696064, 2151841792, 2151841793,
                    2148696064, 2148696064, 2151940096, 2151940097, 2148696064,
                    2152005632, 2152005633, 2148696064, 2148696064, 2152136704,
                    2152136705, 2148696064, 2148696064, 2152235008, 2152235009,
                    2148696064, 2152333312, 2152333313, 2148696064, 2152398848,
                    2152398849, 2148696064, 2148696064, 2152497152, 2152497153,
                    2148696064, 2152595456, 2152595457, 2148696064, 2152660992,
                    2152660993, 2148696064, 2152759296, 2152759297, 2148696064,
                    2153480192, 2153480193, 2152857600, 2153447424, 2153447425,
                    2152955904, 2152955905, 2148696064, 2153021440, 2153021441,
                    2148696064, 2153119744, 2153119745, 2149351424, 2149351425,
                    2148696064, 2148663296, 2148663297, 2148761600, 2148761601,
                    2148827136, 2148827137, 2148892672, 2148892673, 2148958208,
                    2148958209, 2149023744, 2149023745, 2149089280, 2149089281,
                    2149154816, 2149154817, 2149220352, 2149220353, 2148696064,
                    2148696064, 2153578496, 2153578497, 2148696064, 2153644032,
                    2153644033, 2148696064, 2148696064, 2153742336, 2153742337,
                    2148696064,
                ];
                static SHIFT_NONTERM_OFFSETS: &[u32] = &[
                    0, 0, 2, 3, 4, 4, 6, 7, 8, 8, 8, 8, 8, 8, 10, 11, 12, 12, 14, 15, 16,
                    16, 18, 20, 21, 22, 22, 24, 26, 27, 27, 29, 31, 33, 35, 36, 38, 39,
                    39, 41, 42, 44, 45, 47, 48, 50, 51, 53, 54, 56, 57, 59, 60, 62, 63,
                    63, 65, 66, 67, 68, 70, 71, 72, 72, 74, 76, 77, 79, 80, 80, 82, 84,
                    85, 87, 89, 90, 92, 93, 95, 97, 99, 101, 103, 105, 107, 109, 111,
                    113, 114, 116, 118, 120, 122, 124, 126, 128, 130, 132, 134, 135, 136,
                    137, 139, 141, 143, 145, 147, 149, 151, 153, 155, 157, 158, 158, 160,
                    161, 163, 164, 165, 167, 168, 169, 170, 172, 173, 175, 176, 177, 179,
                    180, 180, 182, 183, 185, 186, 187, 189, 190, 192, 193, 194, 194, 196,
                    197, 198, 200, 201, 201, 203, 204, 206, 207, 208, 210, 211, 211, 213,
                    214, 216, 217, 217, 219, 220, 222, 223, 223, 225, 227, 228, 230, 231,
                    233, 235, 236, 238, 240, 242, 244, 246, 248, 250, 252, 254, 255, 256,
                    256, 258, 259, 261, 262, 263, 265, 266, 266,
                ];
                static REDUCE_DATA: &[u32] = &[
                    3, 1, 4, 4, 1, 4, 6, 1, 4, 7, 1, 4, 8, 1, 4, 9, 1, 4, 10, 1, 4, 11,
                    1, 4, 12, 1, 4, 14, 1, 4, 15, 1, 4, 16, 1, 4, 17, 1, 4, 19, 1, 4, 20,
                    1, 4, 21, 1, 4, 22, 1, 4, 23, 1, 4, 24, 1, 4, 25, 1, 4, 26, 1, 4, 29,
                    1, 4, 3, 1, 2, 4, 1, 2, 6, 1, 2, 7, 1, 2, 8, 1, 2, 9, 1, 2, 10, 1, 2,
                    11, 1, 2, 12, 1, 2, 14, 1, 2, 15, 1, 2, 16, 1, 2, 17, 1, 2, 19, 1, 2,
                    20, 1, 2, 21, 1, 2, 22, 1, 2, 23, 1, 2, 24, 1, 2, 25, 1, 2, 26, 1, 2,
                    29, 1, 2, 3, 1, 0, 4, 1, 0, 6, 1, 0, 7, 1, 0, 8, 1, 0, 9, 1, 0, 10,
                    1, 0, 11, 1, 0, 12, 1, 0, 14, 1, 0, 15, 1, 0, 16, 1, 0, 17, 1, 0, 19,
                    1, 0, 20, 1, 0, 21, 1, 0, 22, 1, 0, 23, 1, 0, 24, 1, 0, 25, 1, 0, 26,
                    1, 0, 29, 1, 0, 3, 1, 1, 4, 1, 1, 6, 1, 1, 7, 1, 1, 8, 1, 1, 9, 1, 1,
                    10, 1, 1, 11, 1, 1, 12, 1, 1, 14, 1, 1, 15, 1, 1, 16, 1, 1, 17, 1, 1,
                    19, 1, 1, 20, 1, 1, 21, 1, 1, 22, 1, 1, 23, 1, 1, 24, 1, 1, 25, 1, 1,
                    26, 1, 1, 29, 1, 1, 3, 1, 3, 4, 1, 3, 6, 1, 3, 7, 1, 3, 8, 1, 3, 9,
                    1, 3, 10, 1, 3, 11, 1, 3, 12, 1, 3, 14, 1, 3, 15, 1, 3, 16, 1, 3, 17,
                    1, 3, 19, 1, 3, 20, 1, 3, 21, 1, 3, 22, 1, 3, 23, 1, 3, 24, 1, 3, 25,
                    1, 3, 26, 1, 3, 29, 1, 3, 4, 1, 8, 6, 1, 8, 7, 1, 8, 8, 1, 8, 9, 1,
                    8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 14, 1, 8, 15, 1, 8, 16, 1, 8, 17, 1,
                    8, 21, 1, 8, 22, 1, 8, 29, 1, 8, 4, 1, 22, 6, 1, 22, 7, 1, 22, 8, 1,
                    22, 9, 1, 22, 10, 1, 22, 11, 1, 22, 12, 1, 22, 14, 1, 22, 15, 1, 22,
                    16, 1, 22, 17, 1, 22, 19, 1, 22, 20, 1, 22, 21, 1, 22, 22, 1, 22, 23,
                    1, 22, 24, 1, 22, 25, 1, 22, 26, 1, 22, 29, 1, 22, 4, 1, 9, 6, 1, 9,
                    7, 1, 9, 8, 1, 9, 9, 1, 9, 10, 1, 9, 11, 1, 9, 12, 1, 9, 14, 1, 9,
                    15, 1, 9, 16, 1, 9, 17, 1, 9, 21, 1, 9, 22, 1, 9, 29, 1, 9, 4, 1, 10,
                    6, 1, 10, 7, 1, 10, 8, 1, 10, 9, 1, 10, 10, 1, 10, 11, 1, 10, 12, 1,
                    10, 14, 1, 10, 15, 1, 10, 16, 1, 10, 17, 1, 10, 21, 1, 10, 22, 1, 10,
                    29, 1, 10, 4, 1, 11, 6, 1, 11, 7, 1, 11, 8, 1, 11, 9, 1, 11, 10, 1,
                    11, 11, 1, 11, 12, 1, 11, 14, 1, 11, 15, 1, 11, 16, 1, 11, 17, 1, 11,
                    21, 1, 11, 22, 1, 11, 29, 1, 11, 4, 1, 12, 6, 1, 12, 7, 1, 12, 8, 1,
                    12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 14, 1, 12, 15, 1, 12,
                    16, 1, 12, 17, 1, 12, 21, 1, 12, 22, 1, 12, 29, 1, 12, 4, 1, 6, 6, 1,
                    6, 7, 1, 6, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11, 1, 6, 12, 1, 6, 14, 1, 6,
                    15, 1, 6, 16, 1, 6, 17, 1, 6, 21, 1, 6, 22, 1, 6, 29, 1, 6, 4, 1, 7,
                    6, 1, 7, 7, 1, 7, 8, 1, 7, 9, 1, 7, 10, 1, 7, 11, 1, 7, 12, 1, 7, 14,
                    1, 7, 15, 1, 7, 16, 1, 7, 17, 1, 7, 21, 1, 7, 22, 1, 7, 29, 1, 7, 4,
                    1, 13, 6, 1, 13, 7, 1, 13, 8, 1, 13, 9, 1, 13, 10, 1, 13, 11, 1, 13,
                    12, 1, 13, 14, 1, 13, 15, 1, 13, 16, 1, 13, 17, 1, 13, 21, 1, 13, 22,
                    1, 13, 29, 1, 13, 4, 1, 14, 6, 1, 14, 7, 1, 14, 8, 1, 14, 9, 1, 14,
                    10, 1, 14, 11, 1, 14, 12, 1, 14, 14, 1, 14, 15, 1, 14, 16, 1, 14, 17,
                    1, 14, 21, 1, 14, 22, 1, 14, 29, 1, 14, 3, 1, 5, 4, 1, 5, 6, 1, 5, 7,
                    1, 5, 8, 1, 5, 9, 1, 5, 10, 1, 5, 11, 1, 5, 12, 1, 5, 14, 1, 5, 15,
                    1, 5, 16, 1, 5, 17, 1, 5, 19, 1, 5, 20, 1, 5, 21, 1, 5, 22, 1, 5, 23,
                    1, 5, 24, 1, 5, 25, 1, 5, 26, 1, 5, 29, 1, 5, 21, 1, 20, 4, 1, 15, 6,
                    1, 15, 7, 1, 15, 8, 1, 15, 9, 1, 15, 10, 1, 15, 11, 1, 15, 12, 1, 15,
                    14, 1, 15, 15, 1, 15, 16, 1, 15, 17, 1, 15, 21, 1, 15, 22, 1, 15, 29,
                    1, 15, 7, 1, 19, 22, 1, 19, 7, 1, 20, 6, 1, 20, 7, 1, 18, 22, 1, 18,
                    7, 1, 17, 22, 1, 17, 4, 1, 20, 7, 1, 16, 22, 1, 16, 6, 1, 19, 22, 1,
                    19, 6, 1, 18, 22, 1, 18, 4, 1, 19, 22, 1, 19, 6, 1, 17, 22, 1, 17, 6,
                    1, 16, 22, 1, 16, 21, 1, 19, 22, 1, 19, 21, 1, 18, 22, 1, 18, 4, 1,
                    18, 22, 1, 18, 4, 1, 17, 22, 1, 17, 4, 1, 16, 22, 1, 16, 21, 1, 17,
                    22, 1, 17, 21, 1, 16, 22, 1, 16, 22, 1, 19, 29, 1, 19, 22, 1, 18, 29,
                    1, 18, 22, 1, 17, 29, 1, 17, 22, 1, 16, 29, 1, 16, 29, 1, 20,
                ];
                static REDUCE_OFFSETS: &[u32] = &[
                    0, 0, 0, 0, 0, 66, 66, 66, 66, 132, 198, 264, 330, 330, 330, 330,
                    330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330, 330,
                    330, 330, 330, 330, 330, 330, 330, 375, 438, 438, 483, 483, 528, 528,
                    573, 573, 618, 618, 663, 663, 708, 708, 753, 753, 798, 864, 864, 867,
                    912, 912, 912, 912, 912, 912, 912, 912, 912, 912, 912, 912, 912, 912,
                    912, 912, 912, 912, 912, 918, 918, 918, 918, 918, 918, 918, 918, 918,
                    918, 918, 921, 921, 921, 921, 921, 921, 921, 921, 921, 921, 921, 924,
                    930, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 936, 939, 939,
                    939, 939, 939, 945, 945, 945, 951, 957, 957, 957, 957, 957, 963, 963,
                    963, 969, 969, 969, 969, 969, 975, 975, 975, 975, 975, 981, 987, 987,
                    987, 993, 993, 993, 999, 999, 999, 999, 999, 1005, 1005, 1005, 1011,
                    1011, 1011, 1011, 1011, 1017, 1017, 1017, 1017, 1017, 1017, 1017,
                    1017, 1017, 1017, 1017, 1017, 1017, 1017, 1023, 1023, 1023, 1023,
                    1023, 1023, 1023, 1023, 1023, 1023, 1029, 1035, 1035, 1035, 1035,
                    1035, 1041, 1041, 1041, 1044, 1044,
                ];
                let num_rules = 24usize;
                let mut rules = Vec::with_capacity(num_rules);
                for i in 0..num_rules {
                    let lhs = NonTerminals::from_usize(RULE_NAMES[i] as usize);
                    rules
                        .push(::rusty_lr::parser::table::RuleInfo {
                            lhs,
                            len: RULE_LENGTHS[i] as usize,
                        });
                }
                let num_states = 193usize;
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
        