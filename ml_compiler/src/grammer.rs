
// ================================User Codes Begin================================
use crate::enums::Token;
use crate::enums::Expr;

// =================================User Codes End=================================
/*
====================================Grammar=====================================

# of terminal classes: 28
# of states: 262

0: Expr -> Int
1: Expr -> Bool
2: Expr -> String
3: Expr -> Id
4: Expr -> Unit
5: Expr -> Open Expr Close
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
16: Expr -> Let Expr Eq Expr In Expr
17: Expr -> Let Rec Expr Eq Expr In Expr
18: Expr -> Fun Expr Arrow Expr
19: Expr -> If Expr Then Expr Else Expr
20: Expr -> Expr Semi Expr
21: Augmented -> VirtualStart(0) Expr eof

*/
// =============================Generated Codes Begin==============================
#[allow(non_camel_case_types, dead_code)]
pub type ExprContext = ::rusty_lr::parser::deterministic::Context<
    Parser,
    Data,
    ExprExtracter,
    u16,
>;
#[allow(non_camel_case_types, dead_code)]
pub type Rule = ::rusty_lr::production::Production<TerminalClasses, NonTerminals>;
#[allow(non_camel_case_types, dead_code)]
pub type Tables = ::rusty_lr::parser::table::DenseFlatTables<
    TerminalClasses,
    NonTerminals,
    u8,
    u16,
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
    Expr,
    Augmented,
}
impl NonTerminals {
    #[inline]
    pub fn from_usize(value: usize) -> Self {
        debug_assert!(
            value < 2usize, "Non-terminal index {} is out of bounds (max {})", value,
            2usize
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
            NonTerminals::Expr => "Expr",
            NonTerminals::Augmented => "Augmented",
        }
    }
    fn nonterm_type(&self) -> Option<::rusty_lr::parser::nonterminal::NonTerminalType> {
        match self {
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
    ///Expr -> Int
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
    ///Expr -> Bool
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
    ///Expr -> String
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
    ///Expr -> Id
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
    ///Expr -> Unit
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
    ///Expr -> Open Expr Close
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
    ///Expr -> Let Expr Eq Expr In Expr
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
    ///Expr -> Let Rec Expr Eq Expr In Expr
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
    ///Expr -> Fun Expr Arrow Expr
    #[inline]
    fn reduce_Expr_18(
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
    fn reduce_Expr_19(
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
    fn reduce_Expr_20(
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
            1usize => {
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
            2usize => {
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
            3usize => {
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
            4usize => {
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
            5usize => {
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
            6usize => {
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
            7usize => {
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
            8usize => {
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
            9usize => {
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
            10usize => {
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
            11usize => {
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
            12usize => {
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
            13usize => {
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
            14usize => {
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
            15usize => {
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
            16usize => {
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
            17usize => {
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
            18usize => {
                Self::reduce_Expr_18(
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
                Self::reduce_Expr_19(
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
                Self::reduce_Expr_20(
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
    type StateIndex = u16;
    type ReduceRules = u8;
    type Tables = Tables;
    const ERROR_USED: bool = false;
    fn get_tables() -> &'static Tables {
        static TABLES: std::sync::OnceLock<Tables> = std::sync::OnceLock::new();
        TABLES
            .get_or_init(|| {
                static RULE_NAMES: &[u32] = &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ];
                static RULE_LENGTHS: &[u32] = &[
                    1, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 6, 7, 4, 6, 3, 3,
                ];
                static SHIFT_TERM_DATA: &[u32] = &[
                    2147516446, 65536, 7569410, 7667717, 7831570, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 98304, 7405569,
                    163842, 262149, 360466, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 98304, 131073, 163842, 262149, 360466,
                    393235, 426004, 2148007959, 2148040728, 2148073497, 2148106266,
                    98304, 163842, 262149, 360466, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 196608, 1146882, 1179653,
                    1245202, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 98304, 229377, 163842, 262149, 360466, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 98304, 163842,
                    262149, 360466, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 294912, 1212418, 1409029, 1441810, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 98304, 327681,
                    163842, 262149, 360466, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 98304, 163842, 262149, 360466, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 98304,
                    163842, 262149, 360466, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 458752, 1277954, 1376261, 1474578, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 98304,
                    491521, 163842, 262149, 360466, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 98304, 163842, 262149, 360466,
                    393235, 426004, 2148007959, 2148040728, 2148073497, 2148106266,
                    688136, 753673, 819210, 884747, 950284, 4456462, 4489231, 4522000,
                    4554769, 4587542, 98304, 163842, 262149, 360466, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 98304, 163842,
                    262149, 360466, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 98304, 163842, 262149, 360466, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 98304, 163842,
                    262149, 360466, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 98304, 1015809,
                    163842, 262149, 360466, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 98304, 163842, 262149, 360466, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 688136,
                    753673, 819210, 884747, 1081356, 4456462, 4489231, 4522000, 4554769,
                    4587542, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 196608, 1146882,
                    1179653, 1245202, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 196608, 1146882, 1179653, 1245202, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 294912, 1212418,
                    1409029, 1441810, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 196608, 1146882, 1179653, 1245202, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 196608, 1146882,
                    1179653, 1245202, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 196608, 1146882, 1179653, 1245202, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 1343491, 3571720,
                    3604489, 3637258, 3670027, 3702796, 3768334, 3801103, 3833872,
                    3866641, 3899414, 458752, 1277954, 1376261, 1474578, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 294912, 1212418,
                    1409029, 1441810, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 294912, 1212418, 1409029, 1441810, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 294912, 1212418,
                    1409029, 1441810, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 458752, 1277954, 1376261, 1474578, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 1540104, 1572873,
                    1605642, 1638411, 1671180, 1736718, 1802255, 1867792, 1933329,
                    1998869, 2031638, 458752, 1277954, 1376261, 1474578, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 458752, 1277954,
                    1376261, 1474578, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 458752, 1277954, 1376261, 1474578, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 458752, 1277954,
                    1376261, 1474578, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 458752, 1277954, 1376261, 1474578, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 1540104, 1572873,
                    1605642, 1638411, 1736718, 1802255, 1867792, 1933329, 458752,
                    1277954, 1376261, 1474578, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 458752, 1277954, 1376261, 1474578, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 458752,
                    1277954, 1376261, 1474578, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 458752, 1277954, 1376261, 1474578, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 458752,
                    1277954, 1376261, 1474578, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 1540104, 1572873, 1605642, 1638411, 1671180,
                    1736718, 1802255, 1867792, 1933329, 2031638, 2162694, 3145736,
                    3178505, 3211274, 3244043, 3276812, 3342350, 3375119, 3407888,
                    3440657, 3473430, 2195456, 2424834, 2523141, 2621458, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 98304, 2228225,
                    163842, 262149, 360466, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 98304, 163842, 262149, 360466, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 688136,
                    753673, 819210, 884747, 2293772, 4456462, 4489231, 4522000, 4554769,
                    4587542, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 294912, 1212418,
                    1409029, 1441810, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 2392070, 3145736, 3178505, 3211274, 3244043, 3276812,
                    3342350, 3375119, 3407888, 3440657, 3473430, 2195456, 2424834,
                    2523141, 2621458, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 196608, 1146882, 1179653, 1245202, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 2490371, 3571720,
                    3604489, 3637258, 3670027, 3702796, 3768334, 3801103, 3833872,
                    3866641, 3899414, 2195456, 2424834, 2523141, 2621458, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 294912, 1212418,
                    1409029, 1441810, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 2588678, 3145736, 3178505, 3211274, 3244043, 3276812,
                    3342350, 3375119, 3407888, 3440657, 3473430, 2195456, 2424834,
                    2523141, 2621458, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 2195456, 2424834, 2523141, 2621458, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 2686983, 2752520,
                    2785289, 2818058, 2850827, 3014668, 2883598, 2916367, 2949136,
                    2981905, 3080214, 2195456, 2424834, 2523141, 2621458, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 2752520, 2785289,
                    2818058, 2850827, 2883598, 2916367, 2949136, 2981905, 2195456,
                    2424834, 2523141, 2621458, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 2195456, 2424834, 2523141, 2621458, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 2195456,
                    2424834, 2523141, 2621458, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 2195456, 2424834, 2523141, 2621458, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 2195456,
                    2424834, 2523141, 2621458, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 2195456, 2424834, 2523141, 2621458, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 2195456,
                    2424834, 2523141, 2621458, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 2195456, 2424834, 2523141, 2621458, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 2195456,
                    2424834, 2523141, 2621458, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 2752520, 2785289, 2818058, 2850827, 2883598,
                    2916367, 2949136, 2981905, 2195456, 2424834, 2523141, 2621458,
                    393235, 426004, 2148007959, 2148040728, 2148073497, 2148106266,
                    2752520, 2785289, 2818058, 2850827, 3014668, 2883598, 2916367,
                    2949136, 2981905, 3080214, 294912, 1212418, 1409029, 1441810, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 294912,
                    1212418, 1409029, 1441810, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 294912, 1212418, 1409029, 1441810, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 294912,
                    1212418, 1409029, 1441810, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 294912, 1212418, 1409029, 1441810, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 3145736,
                    3178505, 3211274, 3244043, 3342350, 3375119, 3407888, 3440657,
                    294912, 1212418, 1409029, 1441810, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 294912, 1212418, 1409029,
                    1441810, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 294912, 1212418, 1409029, 1441810, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 294912, 1212418,
                    1409029, 1441810, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 294912, 1212418, 1409029, 1441810, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 3145736, 3178505,
                    3211274, 3244043, 3276812, 3342350, 3375119, 3407888, 3440657,
                    3473430, 2752520, 2785289, 2818058, 2850827, 2883598, 2916367,
                    2949136, 2981905, 196608, 1146882, 1179653, 1245202, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 196608, 1146882,
                    1179653, 1245202, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 196608, 1146882, 1179653, 1245202, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 196608, 1146882,
                    1179653, 1245202, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 196608, 1146882, 1179653, 1245202, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 3571720, 3604489,
                    3637258, 3670027, 3768334, 3801103, 3833872, 3866641, 196608,
                    1146882, 1179653, 1245202, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 196608, 1146882, 1179653, 1245202, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 196608,
                    1146882, 1179653, 1245202, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 196608, 1146882, 1179653, 1245202, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 196608,
                    1146882, 1179653, 1245202, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 3571720, 3604489, 3637258, 3670027, 3702796,
                    3768334, 3801103, 3833872, 3866641, 3899414, 3997703, 2752520,
                    2785289, 2818058, 2850827, 3014668, 2883598, 2916367, 2949136,
                    2981905, 3080214, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 983040, 1114114,
                    2326533, 4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 4096008, 4128777, 4161546, 4194315, 4227086, 4259855,
                    4292624, 4325393, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 983040, 1114114,
                    2326533, 4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 983040, 1114114,
                    2326533, 4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 983040, 1114114,
                    2326533, 4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 983040, 1114114,
                    2326533, 4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 4390916, 4096008, 4128777, 4161546, 4194315, 4227086,
                    4259855, 4292624, 4325393, 2195456, 2424834, 2523141, 2621458,
                    393235, 426004, 2148007959, 2148040728, 2148073497, 2148106266,
                    2752520, 2785289, 2818058, 2850827, 2883598, 2916367, 2949136,
                    2981905, 98304, 163842, 262149, 360466, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 98304, 163842, 262149, 360466,
                    393235, 426004, 2148007959, 2148040728, 2148073497, 2148106266,
                    98304, 163842, 262149, 360466, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 98304, 163842, 262149, 360466,
                    393235, 426004, 2148007959, 2148040728, 2148073497, 2148106266,
                    98304, 163842, 262149, 360466, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 688136, 753673, 819210, 884747,
                    4653068, 4456462, 4489231, 4522000, 4554769, 4587542, 98304, 163842,
                    262149, 360466, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 688136, 753673, 819210, 884747, 4456462, 4489231,
                    4522000, 4554769, 688136, 753673, 819210, 884747, 4751372, 4456462,
                    4489231, 4522000, 4554769, 4587542, 983040, 1114114, 2326533,
                    4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 4816900, 4096008, 4128777, 4161546, 4194315, 4227086,
                    4259855, 4292624, 4325393, 2195456, 2424834, 2523141, 2621458,
                    393235, 426004, 2148007959, 2148040728, 2148073497, 2148106266,
                    2752520, 2785289, 2818058, 2850827, 2883598, 2916367, 2949136,
                    2981905, 4915207, 2752520, 2785289, 2818058, 2850827, 3014668,
                    2883598, 2916367, 2949136, 2981905, 3080214, 294912, 1212418,
                    1409029, 1441810, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 3145736, 3178505, 3211274, 3244043, 3342350, 3375119,
                    3407888, 3440657, 5013510, 3145736, 3178505, 3211274, 3244043,
                    3276812, 3342350, 3375119, 3407888, 3440657, 3473430, 2195456,
                    2424834, 2523141, 2621458, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 5079047, 2752520, 2785289, 2818058, 2850827,
                    3014668, 2883598, 2916367, 2949136, 2981905, 3080214, 458752,
                    1277954, 1376261, 1474578, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 1540104, 1572873, 1605642, 1638411, 1736718,
                    1802255, 1867792, 1933329, 1540104, 1572873, 1605642, 1638411,
                    1736718, 1802255, 1867792, 1933329, 5210115, 3571720, 3604489,
                    3637258, 3670027, 3702796, 3768334, 3801103, 3833872, 3866641,
                    3899414, 294912, 1212418, 1409029, 1441810, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 3145736, 3178505,
                    3211274, 3244043, 3342350, 3375119, 3407888, 3440657, 5308422,
                    3145736, 3178505, 3211274, 3244043, 3276812, 3342350, 3375119,
                    3407888, 3440657, 3473430, 2195456, 2424834, 2523141, 2621458,
                    393235, 426004, 2148007959, 2148040728, 2148073497, 2148106266,
                    5373959, 2752520, 2785289, 2818058, 2850827, 3014668, 2883598,
                    2916367, 2949136, 2981905, 3080214, 196608, 1146882, 1179653,
                    1245202, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 3571720, 3604489, 3637258, 3670027, 3768334, 3801103,
                    3833872, 3866641, 5472259, 3571720, 3604489, 3637258, 3670027,
                    3702796, 3768334, 3801103, 3833872, 3866641, 3899414, 196608,
                    1146882, 1179653, 1245202, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 3571720, 3604489, 3637258, 3670027, 3768334,
                    3801103, 3833872, 3866641, 5570563, 3571720, 3604489, 3637258,
                    3670027, 3702796, 3768334, 3801103, 3833872, 3866641, 3899414,
                    983040, 1114114, 2326533, 4030482, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 4096008, 4128777, 4161546,
                    4194315, 4227086, 4259855, 4292624, 4325393, 5668868, 4096008,
                    4128777, 4161546, 4194315, 4227086, 4259855, 4292624, 4325393,
                    983040, 1114114, 2326533, 4030482, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 4096008, 4128777, 4161546,
                    4194315, 4227086, 4259855, 4292624, 4325393, 688136, 753673, 819210,
                    884747, 5767180, 4456462, 4489231, 4522000, 4554769, 4587542, 983040,
                    1114114, 2326533, 4030482, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 5832708, 4096008, 4128777, 4161546, 4194315,
                    4227086, 4259855, 4292624, 4325393, 983040, 1114114, 2326533,
                    4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 4096008, 4128777, 4161546, 4194315, 4227086, 4259855,
                    4292624, 4325393, 5931012, 4096008, 4128777, 4161546, 4194315,
                    4227086, 4259855, 4292624, 4325393, 458752, 1277954, 1376261,
                    1474578, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 1540104, 1572873, 1605642, 1638411, 1736718, 1802255,
                    1867792, 1933329, 688136, 753673, 819210, 884747, 6029324, 4456462,
                    4489231, 4522000, 4554769, 4587542, 983040, 1114114, 2326533,
                    4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 6094852, 4096008, 4128777, 4161546, 4194315, 4227086,
                    4259855, 4292624, 4325393, 458752, 1277954, 1376261, 1474578, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 1540104,
                    1572873, 1605642, 1638411, 1736718, 1802255, 1867792, 1933329,
                    688136, 753673, 819210, 884747, 6193164, 4456462, 4489231, 4522000,
                    4554769, 4587542, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 6258692, 4096008,
                    4128777, 4161546, 4194315, 4227086, 4259855, 4292624, 4325393,
                    294912, 1212418, 1409029, 1441810, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 3145736, 3178505, 3211274,
                    3244043, 3342350, 3375119, 3407888, 3440657, 688136, 753673, 819210,
                    884747, 6357004, 4456462, 4489231, 4522000, 4554769, 4587542, 983040,
                    1114114, 2326533, 4030482, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 6422532, 4096008, 4128777, 4161546, 4194315,
                    4227086, 4259855, 4292624, 4325393, 294912, 1212418, 1409029,
                    1441810, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 3145736, 3178505, 3211274, 3244043, 3342350, 3375119,
                    3407888, 3440657, 6520838, 3145736, 3178505, 3211274, 3244043,
                    3276812, 3342350, 3375119, 3407888, 3440657, 3473430, 2195456,
                    2424834, 2523141, 2621458, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 6586375, 2752520, 2785289, 2818058, 2850827,
                    3014668, 2883598, 2916367, 2949136, 2981905, 3080214, 98304, 163842,
                    262149, 360466, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 688136, 753673, 819210, 884747, 4456462, 4489231,
                    4522000, 4554769, 688136, 753673, 819210, 884747, 6684684, 4456462,
                    4489231, 4522000, 4554769, 4587542, 983040, 1114114, 2326533,
                    4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 6750212, 4096008, 4128777, 4161546, 4194315, 4227086,
                    4259855, 4292624, 4325393, 196608, 1146882, 1179653, 1245202, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 3571720,
                    3604489, 3637258, 3670027, 3768334, 3801103, 3833872, 3866641,
                    688136, 753673, 819210, 884747, 6848524, 4456462, 4489231, 4522000,
                    4554769, 4587542, 983040, 1114114, 2326533, 4030482, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 6914052, 4096008,
                    4128777, 4161546, 4194315, 4227086, 4259855, 4292624, 4325393,
                    196608, 1146882, 1179653, 1245202, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 3571720, 3604489, 3637258,
                    3670027, 3768334, 3801103, 3833872, 3866641, 7012355, 3571720,
                    3604489, 3637258, 3670027, 3702796, 3768334, 3801103, 3833872,
                    3866641, 3899414, 98304, 163842, 262149, 360466, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 688136, 753673,
                    819210, 884747, 4456462, 4489231, 4522000, 4554769, 688136, 753673,
                    819210, 884747, 7110668, 4456462, 4489231, 4522000, 4554769, 4587542,
                    983040, 1114114, 2326533, 4030482, 393235, 426004, 2148007959,
                    2148040728, 2148073497, 2148106266, 7176196, 4096008, 4128777,
                    4161546, 4194315, 4227086, 4259855, 4292624, 4325393, 98304, 163842,
                    262149, 360466, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 688136, 753673, 819210, 884747, 4456462, 4489231,
                    4522000, 4554769, 688136, 753673, 819210, 884747, 7274508, 4456462,
                    4489231, 4522000, 4554769, 4587542, 983040, 1114114, 2326533,
                    4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 7340036, 4096008, 4128777, 4161546, 4194315, 4227086,
                    4259855, 4292624, 4325393, 98304, 163842, 262149, 360466, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 688136,
                    753673, 819210, 884747, 4456462, 4489231, 4522000, 4554769, 98304,
                    163842, 262149, 360466, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 688136, 753673, 819210, 884747, 7471116,
                    4456462, 4489231, 4522000, 4554769, 4587542, 983040, 1114114,
                    2326533, 4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 7536644, 4096008, 4128777, 4161546, 4194315, 4227086,
                    4259855, 4292624, 4325393, 65536, 7569410, 7667717, 7831570, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 196608,
                    1146882, 1179653, 1245202, 393235, 426004, 2148007959, 2148040728,
                    2148073497, 2148106266, 7634947, 3571720, 3604489, 3637258, 3670027,
                    3702796, 3768334, 3801103, 3833872, 3866641, 3899414, 65536, 7569410,
                    7667717, 7831570, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 294912, 1212418, 1409029, 1441810, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 7733254, 3145736,
                    3178505, 3211274, 3244043, 3276812, 3342350, 3375119, 3407888,
                    3440657, 3473430, 2195456, 2424834, 2523141, 2621458, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 7798791, 2752520,
                    2785289, 2818058, 2850827, 3014668, 2883598, 2916367, 2949136,
                    2981905, 3080214, 65536, 7569410, 7667717, 7831570, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 65536, 7569410,
                    7667717, 7831570, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 7897096, 7929865, 7962634, 7995403, 8028174, 8060943,
                    8093712, 8126481, 65536, 7569410, 7667717, 7831570, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 65536, 7569410,
                    7667717, 7831570, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 65536, 7569410, 7667717, 7831570, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 65536, 7569410,
                    7667717, 7831570, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 65536, 7569410, 7667717, 7831570, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 65536, 7569410,
                    7667717, 7831570, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 65536, 7569410, 7667717, 7831570, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 65536, 7569410,
                    7667717, 7831570, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 7897096, 7929865, 7962634, 7995403, 8028174, 8060943,
                    8093712, 8126481, 7897096, 7929865, 7962634, 7995403, 8028174,
                    8060943, 8093712, 8126481, 688136, 753673, 819210, 884747, 8257548,
                    4456462, 4489231, 4522000, 4554769, 4587542, 983040, 1114114,
                    2326533, 4030482, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 8323076, 4096008, 4128777, 4161546, 4194315, 4227086,
                    4259855, 4292624, 4325393, 65536, 7569410, 7667717, 7831570, 393235,
                    426004, 2148007959, 2148040728, 2148073497, 2148106266, 7897096,
                    7929865, 7962634, 7995403, 8028174, 8060943, 8093712, 8126481,
                    7897096, 7929865, 7962634, 7995403, 8421388, 8028174, 8060943,
                    8093712, 8126481, 8486934, 2156036125, 65536, 7569410, 7667717,
                    7831570, 393235, 426004, 2148007959, 2148040728, 2148073497,
                    2148106266, 7897096, 7929865, 7962634, 7995403, 8028174, 8060943,
                    8093712, 8126481, 65536, 7569410, 7667717, 7831570, 393235, 426004,
                    2148007959, 2148040728, 2148073497, 2148106266, 7897096, 7929865,
                    7962634, 7995403, 8421388, 8028174, 8060943, 8093712, 8126481,
                    8486934,
                ];
                static SHIFT_TERM_OFFSETS: &[u32] = &[
                    0, 1, 11, 22, 33, 43, 53, 64, 74, 84, 95, 105, 115, 115, 125, 136,
                    146, 146, 146, 146, 146, 156, 166, 166, 176, 176, 186, 186, 196, 196,
                    206, 217, 227, 237, 247, 257, 267, 277, 287, 297, 307, 318, 328, 338,
                    348, 358, 368, 379, 389, 399, 409, 419, 429, 437, 447, 447, 457, 457,
                    467, 467, 477, 477, 477, 487, 497, 497, 508, 518, 529, 539, 549, 559,
                    569, 580, 590, 600, 611, 621, 631, 642, 652, 662, 673, 683, 691, 701,
                    711, 721, 731, 741, 751, 761, 771, 781, 789, 799, 809, 819, 829, 839,
                    849, 859, 867, 877, 887, 897, 907, 917, 927, 935, 945, 955, 965, 975,
                    985, 993, 1003, 1013, 1023, 1033, 1043, 1053, 1064, 1074, 1084, 1092,
                    1102, 1112, 1122, 1132, 1142, 1152, 1162, 1172, 1181, 1191, 1199,
                    1209, 1219, 1229, 1239, 1249, 1259, 1269, 1277, 1287, 1297, 1306,
                    1316, 1324, 1335, 1345, 1353, 1364, 1374, 1385, 1395, 1403, 1411,
                    1422, 1432, 1440, 1451, 1461, 1472, 1482, 1490, 1501, 1511, 1519,
                    1530, 1540, 1548, 1557, 1567, 1575, 1585, 1595, 1604, 1614, 1622,
                    1631, 1641, 1649, 1659, 1669, 1678, 1688, 1696, 1706, 1716, 1725,
                    1735, 1743, 1753, 1763, 1772, 1782, 1790, 1801, 1811, 1822, 1832,
                    1840, 1850, 1860, 1869, 1879, 1887, 1897, 1907, 1916, 1926, 1934,
                    1945, 1955, 1963, 1973, 1983, 1992, 2002, 2010, 2020, 2030, 2039,
                    2049, 2057, 2067, 2077, 2087, 2096, 2106, 2116, 2127, 2137, 2147,
                    2158, 2168, 2179, 2189, 2199, 2207, 2217, 2227, 2237, 2247, 2257,
                    2267, 2277, 2287, 2295, 2303, 2313, 2323, 2332, 2342, 2350, 2361,
                    2371, 2379, 2389, 2399, 2399,
                ];
                static SHIFT_NONTERM_DATA: &[u32] = &[
                    2155872256, 2155708416, 2154725376, 2154561536, 2154463232,
                    2154299392, 2154135552, 2153971712, 2153807872, 2153644032,
                    2149580800, 2148990976, 2153480192, 2148139008, 2148204544,
                    2148270080, 2148335616, 2148401152, 2153381888, 2153218048,
                    2148532224, 2153119744, 2153021440, 2152923136, 2152759296,
                    2152660992, 2149580800, 2148794368, 2152628224, 2152464384,
                    2149613568, 2149580800, 2149580800, 2148204544, 2148270080,
                    2148335616, 2148401152, 2149187584, 2149253120, 2149318656,
                    2149384192, 2149449728, 2149548032, 2152366080, 2152202240,
                    2149744640, 2151841792, 2149842944, 2151448576, 2149941248,
                    2151022592, 2150039552, 2150137856, 2149580800, 2150203392,
                    2148204544, 2148270080, 2148335616, 2148401152, 2149253120,
                    2149318656, 2149384192, 2149449728, 2150531072, 2150596608,
                    2148204544, 2148270080, 2148335616, 2148401152, 2150793216,
                    2149253120, 2149318656, 2149384192, 2149449728, 2150989824,
                    2148204544, 2148270080, 2148335616, 2148401152, 2151219200,
                    2149253120, 2149318656, 2149384192, 2149449728, 2151415808,
                    2151546880, 2149580800, 2148204544, 2148270080, 2148335616,
                    2148401152, 2149253120, 2149318656, 2149384192, 2149449728,
                    2151907328, 2149253120, 2149318656, 2149384192, 2149449728,
                    2152103936, 2152169472, 2152267776, 2152333312, 2152431616,
                    2152529920, 2152595456, 2152726528, 2152824832, 2152890368,
                    2152988672, 2153086976, 2153185280, 2153283584, 2153349120,
                    2153447424, 2153545728, 2153611264, 2153709568, 2153775104,
                    2153873408, 2153938944, 2154037248, 2154102784, 2154201088,
                    2154266624, 2154364928, 2154430464, 2154528768, 2154627072,
                    2154692608, 2154790912, 2154856448, 2154921984, 2154987520,
                    2155675648, 2155085824, 2155642880, 2155184128, 2155249664,
                    2155347968, 2149580800, 2148204544, 2148270080, 2148335616,
                    2148401152, 2149253120, 2149318656, 2149384192, 2149449728,
                    2155773952, 2155839488, 2155937792, 2156003328,
                ];
                static SHIFT_NONTERM_OFFSETS: &[u32] = &[
                    0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11, 12, 13, 14, 14, 14, 14,
                    14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 20, 21, 21, 22, 23, 24,
                    25, 26, 27, 28, 28, 29, 30, 31, 32, 33, 33, 34, 35, 36, 37, 38, 38,
                    39, 39, 40, 40, 41, 41, 42, 42, 42, 43, 43, 43, 43, 44, 45, 46, 46,
                    47, 48, 48, 49, 50, 50, 51, 52, 52, 53, 54, 54, 55, 55, 56, 57, 58,
                    59, 60, 61, 62, 63, 64, 64, 65, 65, 66, 67, 68, 69, 70, 70, 71, 72,
                    73, 74, 75, 75, 75, 76, 77, 78, 79, 80, 80, 81, 82, 83, 84, 85, 85,
                    85, 86, 87, 87, 88, 89, 90, 91, 92, 93, 94, 95, 95, 96, 96, 97, 98,
                    99, 100, 101, 101, 102, 102, 102, 103, 103, 104, 104, 104, 105, 105,
                    105, 106, 106, 107, 107, 107, 107, 108, 108, 108, 109, 109, 110, 110,
                    110, 111, 111, 111, 112, 112, 112, 113, 113, 113, 114, 114, 115, 115,
                    115, 116, 116, 116, 117, 117, 118, 118, 118, 119, 119, 120, 120, 120,
                    121, 121, 122, 122, 122, 123, 123, 124, 124, 124, 125, 125, 126, 126,
                    126, 127, 127, 128, 128, 128, 129, 129, 129, 130, 130, 131, 131, 131,
                    132, 132, 133, 133, 134, 134, 135, 135, 136, 137, 137, 138, 139, 139,
                    140, 140, 141, 142, 142, 143, 144, 145, 146, 147, 148, 149, 150, 150,
                    150, 150, 151, 151, 152, 152, 152, 153, 153, 154, 154, 154,
                ];
                static REDUCE_DATA: &[u32] = &[
                    3, 1, 4, 4, 1, 4, 6, 1, 4, 7, 1, 4, 8, 1, 4, 9, 1, 4, 10, 1, 4, 11,
                    1, 4, 12, 1, 4, 14, 1, 4, 15, 1, 4, 16, 1, 4, 17, 1, 4, 21, 1, 4, 22,
                    1, 4, 29, 1, 4, 3, 1, 2, 4, 1, 2, 6, 1, 2, 7, 1, 2, 8, 1, 2, 9, 1, 2,
                    10, 1, 2, 11, 1, 2, 12, 1, 2, 14, 1, 2, 15, 1, 2, 16, 1, 2, 17, 1, 2,
                    21, 1, 2, 22, 1, 2, 29, 1, 2, 3, 1, 0, 4, 1, 0, 6, 1, 0, 7, 1, 0, 8,
                    1, 0, 9, 1, 0, 10, 1, 0, 11, 1, 0, 12, 1, 0, 14, 1, 0, 15, 1, 0, 16,
                    1, 0, 17, 1, 0, 21, 1, 0, 22, 1, 0, 29, 1, 0, 3, 1, 1, 4, 1, 1, 6, 1,
                    1, 7, 1, 1, 8, 1, 1, 9, 1, 1, 10, 1, 1, 11, 1, 1, 12, 1, 1, 14, 1, 1,
                    15, 1, 1, 16, 1, 1, 17, 1, 1, 21, 1, 1, 22, 1, 1, 29, 1, 1, 3, 1, 3,
                    4, 1, 3, 6, 1, 3, 7, 1, 3, 8, 1, 3, 9, 1, 3, 10, 1, 3, 11, 1, 3, 12,
                    1, 3, 14, 1, 3, 15, 1, 3, 16, 1, 3, 17, 1, 3, 21, 1, 3, 22, 1, 3, 29,
                    1, 3, 3, 1, 8, 4, 1, 8, 6, 1, 8, 7, 1, 8, 8, 1, 8, 9, 1, 8, 10, 1, 8,
                    11, 1, 8, 12, 1, 8, 14, 1, 8, 15, 1, 8, 16, 1, 8, 17, 1, 8, 21, 1, 8,
                    22, 1, 8, 29, 1, 8, 3, 1, 9, 4, 1, 9, 6, 1, 9, 7, 1, 9, 8, 1, 9, 9,
                    1, 9, 10, 1, 9, 11, 1, 9, 12, 1, 9, 14, 1, 9, 15, 1, 9, 16, 1, 9, 17,
                    1, 9, 21, 1, 9, 22, 1, 9, 29, 1, 9, 3, 1, 10, 4, 1, 10, 6, 1, 10, 7,
                    1, 10, 8, 1, 10, 9, 1, 10, 10, 1, 10, 11, 1, 10, 12, 1, 10, 14, 1,
                    10, 15, 1, 10, 16, 1, 10, 17, 1, 10, 21, 1, 10, 22, 1, 10, 29, 1, 10,
                    3, 1, 11, 4, 1, 11, 6, 1, 11, 7, 1, 11, 8, 1, 11, 9, 1, 11, 10, 1,
                    11, 11, 1, 11, 12, 1, 11, 14, 1, 11, 15, 1, 11, 16, 1, 11, 17, 1, 11,
                    21, 1, 11, 22, 1, 11, 29, 1, 11, 12, 1, 12, 21, 1, 12, 22, 1, 12, 3,
                    1, 6, 4, 1, 6, 6, 1, 6, 7, 1, 6, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11, 1,
                    6, 12, 1, 6, 14, 1, 6, 15, 1, 6, 16, 1, 6, 17, 1, 6, 21, 1, 6, 22, 1,
                    6, 29, 1, 6, 3, 1, 7, 4, 1, 7, 6, 1, 7, 7, 1, 7, 8, 1, 7, 9, 1, 7,
                    10, 1, 7, 11, 1, 7, 12, 1, 7, 14, 1, 7, 15, 1, 7, 16, 1, 7, 17, 1, 7,
                    21, 1, 7, 22, 1, 7, 29, 1, 7, 3, 1, 13, 4, 1, 13, 6, 1, 13, 7, 1, 13,
                    8, 1, 13, 9, 1, 13, 10, 1, 13, 11, 1, 13, 12, 1, 13, 14, 1, 13, 15,
                    1, 13, 16, 1, 13, 17, 1, 13, 21, 1, 13, 22, 1, 13, 29, 1, 13, 3, 1,
                    14, 4, 1, 14, 6, 1, 14, 7, 1, 14, 8, 1, 14, 9, 1, 14, 10, 1, 14, 11,
                    1, 14, 12, 1, 14, 14, 1, 14, 15, 1, 14, 16, 1, 14, 17, 1, 14, 21, 1,
                    14, 22, 1, 14, 29, 1, 14, 3, 1, 5, 4, 1, 5, 6, 1, 5, 7, 1, 5, 8, 1,
                    5, 9, 1, 5, 10, 1, 5, 11, 1, 5, 12, 1, 5, 14, 1, 5, 15, 1, 5, 16, 1,
                    5, 17, 1, 5, 21, 1, 5, 22, 1, 5, 29, 1, 5, 21, 1, 20, 3, 1, 15, 4, 1,
                    15, 6, 1, 15, 7, 1, 15, 8, 1, 15, 9, 1, 15, 10, 1, 15, 11, 1, 15, 12,
                    1, 15, 14, 1, 15, 15, 1, 15, 16, 1, 15, 17, 1, 15, 21, 1, 15, 22, 1,
                    15, 29, 1, 15, 7, 1, 19, 12, 1, 19, 22, 1, 19, 7, 1, 12, 12, 1, 12,
                    22, 1, 12, 7, 1, 20, 6, 1, 12, 12, 1, 12, 22, 1, 12, 6, 1, 20, 7, 1,
                    18, 12, 1, 18, 22, 1, 18, 3, 1, 12, 12, 1, 12, 22, 1, 12, 3, 1, 20,
                    4, 1, 19, 12, 1, 19, 22, 1, 19, 12, 1, 12, 22, 1, 12, 7, 1, 17, 12,
                    1, 17, 22, 1, 17, 12, 1, 12, 22, 1, 12, 12, 1, 12, 22, 1, 12, 7, 1,
                    16, 12, 1, 16, 22, 1, 16, 6, 1, 19, 12, 1, 19, 22, 1, 19, 12, 1, 19,
                    21, 1, 19, 22, 1, 19, 12, 1, 18, 21, 1, 18, 22, 1, 18, 6, 1, 18, 12,
                    1, 18, 22, 1, 18, 3, 1, 19, 12, 1, 19, 22, 1, 19, 3, 1, 18, 12, 1,
                    18, 22, 1, 18, 4, 1, 18, 12, 1, 18, 22, 1, 18, 12, 1, 12, 22, 1, 12,
                    4, 1, 17, 12, 1, 17, 22, 1, 17, 12, 1, 12, 22, 1, 12, 4, 1, 16, 12,
                    1, 16, 22, 1, 16, 12, 1, 12, 22, 1, 12, 12, 1, 17, 21, 1, 17, 22, 1,
                    17, 12, 1, 12, 22, 1, 12, 12, 1, 16, 21, 1, 16, 22, 1, 16, 12, 1, 12,
                    22, 1, 12, 6, 1, 17, 12, 1, 17, 22, 1, 17, 12, 1, 12, 22, 1, 12, 6,
                    1, 16, 12, 1, 16, 22, 1, 16, 12, 1, 19, 22, 1, 19, 12, 1, 12, 22, 1,
                    12, 3, 1, 17, 12, 1, 17, 22, 1, 17, 12, 1, 12, 22, 1, 12, 3, 1, 16,
                    12, 1, 16, 22, 1, 16, 12, 1, 18, 22, 1, 18, 12, 1, 12, 22, 1, 12, 12,
                    1, 17, 22, 1, 17, 12, 1, 12, 22, 1, 12, 12, 1, 16, 22, 1, 16, 12, 1,
                    12, 22, 1, 12, 12, 1, 19, 22, 1, 19, 29, 1, 19, 12, 1, 18, 22, 1, 18,
                    29, 1, 18, 12, 1, 17, 22, 1, 17, 29, 1, 17, 12, 1, 12, 22, 1, 12, 12,
                    1, 16, 22, 1, 16, 29, 1, 16, 12, 1, 12, 22, 1, 12, 29, 1, 12, 29, 1,
                    20,
                ];
                static REDUCE_OFFSETS: &[u32] = &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48, 48, 48, 96, 144, 192,
                    240, 240, 240, 288, 288, 336, 336, 384, 384, 432, 432, 432, 432, 432,
                    432, 432, 432, 432, 432, 432, 432, 432, 432, 432, 432, 432, 432, 432,
                    432, 432, 432, 432, 432, 441, 441, 489, 489, 537, 537, 585, 585, 633,
                    681, 681, 684, 732, 732, 732, 732, 732, 732, 732, 732, 732, 732, 732,
                    732, 732, 732, 732, 732, 732, 732, 732, 741, 741, 741, 741, 741, 741,
                    741, 741, 741, 741, 750, 750, 753, 753, 753, 753, 753, 753, 762, 762,
                    762, 762, 762, 762, 765, 774, 774, 774, 774, 774, 774, 783, 783, 783,
                    783, 783, 783, 786, 786, 786, 786, 795, 795, 795, 795, 795, 795, 795,
                    795, 795, 801, 801, 810, 810, 810, 810, 810, 810, 810, 810, 816, 816,
                    816, 822, 822, 831, 831, 831, 840, 840, 840, 840, 840, 849, 858, 858,
                    858, 867, 867, 867, 867, 867, 876, 876, 876, 885, 885, 885, 894, 900,
                    900, 909, 909, 909, 915, 915, 924, 930, 930, 939, 939, 939, 945, 945,
                    954, 954, 954, 960, 960, 969, 969, 969, 975, 975, 984, 984, 984, 984,
                    984, 990, 990, 990, 996, 996, 1005, 1005, 1005, 1011, 1011, 1020,
                    1020, 1020, 1026, 1026, 1026, 1032, 1032, 1038, 1038, 1038, 1044,
                    1044, 1050, 1050, 1050, 1050, 1056, 1056, 1056, 1056, 1056, 1056,
                    1056, 1056, 1056, 1056, 1056, 1065, 1065, 1065, 1065, 1065, 1065,
                    1065, 1065, 1065, 1074, 1083, 1083, 1083, 1089, 1089, 1098, 1098,
                    1098, 1107, 1107, 1110, 1110,
                ];
                let num_rules = 22usize;
                let mut rules = Vec::with_capacity(num_rules);
                for i in 0..num_rules {
                    let lhs = NonTerminals::from_usize(RULE_NAMES[i] as usize);
                    rules
                        .push(::rusty_lr::parser::table::RuleInfo {
                            lhs,
                            len: RULE_LENGTHS[i] as usize,
                        });
                }
                let num_states = 262usize;
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
        