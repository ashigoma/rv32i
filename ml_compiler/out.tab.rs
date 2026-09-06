
// ================================User Codes Begin================================
use crate::enums::Token;
use crate::enums::Expr;

// =================================User Codes End=================================
/*
====================================Grammar=====================================

# of terminal classes: 27
# of states: 314

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
            value < 30usize, "Terminal class index {} is out of bounds (max {})", value,
            30usize
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
                    2147516445, 65536, 9011202, 9109509, 9273361, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 98304, 8847361,
                    163842, 262149, 360465, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 98304, 131073, 163842, 262149, 360465,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    98304, 163842, 262149, 360465, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 196608, 1212418, 1245189,
                    1310737, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 98304, 229377, 163842, 262149, 360465, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 98304, 163842,
                    262149, 360465, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 294912, 1277954, 1474565, 1507345, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 98304, 327681,
                    163842, 262149, 360465, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 98304, 163842, 262149, 360465, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 98304,
                    163842, 262149, 360465, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 458752, 1343490, 1441797, 1540113, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 98304,
                    491521, 163842, 262149, 360465, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 98304, 163842, 262149, 360465,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    688136, 819209, 884746, 950283, 1015820, 5767181, 5832718, 5898255,
                    5963792, 753685, 98304, 163842, 262149, 360465, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 753685, 98304,
                    163842, 262149, 360465, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 98304, 163842, 262149, 360465, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 753685,
                    98304, 163842, 262149, 360465, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 753685, 98304, 163842, 262149,
                    360465, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 753685, 1048576, 1179650, 3145733, 4947985, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 98304,
                    1081345, 163842, 262149, 360465, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 98304, 163842, 262149, 360465,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    688136, 819209, 884746, 950283, 1146892, 5767181, 5832718, 5898255,
                    5963792, 753685, 1048576, 1179650, 3145733, 4947985, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 196608, 1212418,
                    1245189, 1310737, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 196608, 1212418, 1245189, 1310737, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 294912, 1277954,
                    1474565, 1507345, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 196608, 1212418, 1245189, 1310737, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 196608, 1212418,
                    1245189, 1310737, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 196608, 1212418, 1245189, 1310737, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 1409027, 4259848,
                    4358153, 4423690, 4489227, 4554764, 4620301, 4685838, 4751375,
                    4816912, 4325397, 458752, 1343490, 1441797, 1540113, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 294912, 1277954,
                    1474565, 1507345, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 294912, 1277954, 1474565, 1507345, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 294912, 1277954,
                    1474565, 1507345, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 458752, 1343490, 1441797, 1540113, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 1605640, 1703945,
                    1769482, 1835019, 1900556, 1966093, 2031630, 2097167, 2162704,
                    2228244, 1671189, 458752, 1343490, 1441797, 1540113, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 1671189, 458752,
                    1343490, 1441797, 1540113, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 458752, 1343490, 1441797, 1540113, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 1671189,
                    458752, 1343490, 1441797, 1540113, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 1671189, 458752, 1343490,
                    1441797, 1540113, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 1671189, 458752, 1343490, 1441797, 1540113, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 1671189,
                    458752, 1343490, 1441797, 1540113, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 1671189, 458752, 1343490,
                    1441797, 1540113, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 1671189, 458752, 1343490, 1441797, 1540113, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 1671189,
                    458752, 1343490, 1441797, 1540113, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 1671189, 1605640, 1703945,
                    1769482, 1835019, 1900556, 1966093, 2031630, 2097167, 2162704,
                    1671189, 2326536, 2424841, 2490378, 2555915, 2621452, 2686989,
                    2752526, 2818063, 2883600, 2392085, 294912, 1277954, 1474565,
                    1507345, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 2392085, 294912, 1277954, 1474565, 1507345, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 294912,
                    1277954, 1474565, 1507345, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 2392085, 294912, 1277954, 1474565, 1507345,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    2392085, 294912, 1277954, 1474565, 1507345, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 2392085, 294912,
                    1277954, 1474565, 1507345, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 2392085, 294912, 1277954, 1474565, 1507345,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    2392085, 294912, 1277954, 1474565, 1507345, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 2392085, 294912,
                    1277954, 1474565, 1507345, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 2392085, 294912, 1277954, 1474565, 1507345,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    2392085, 2981894, 2326536, 2424841, 2490378, 2555915, 2621452,
                    2686989, 2752526, 2818063, 2883600, 2392085, 3014656, 3244034,
                    3342341, 3440657, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 98304, 3047425, 163842, 262149, 360465, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 98304, 163842,
                    262149, 360465, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 688136, 819209, 884746, 950283, 3112972, 5767181,
                    5832718, 5898255, 5963792, 753685, 1048576, 1179650, 3145733,
                    4947985, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 294912, 1277954, 1474565, 1507345, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 3211270, 2326536,
                    2424841, 2490378, 2555915, 2621452, 2686989, 2752526, 2818063,
                    2883600, 2392085, 3014656, 3244034, 3342341, 3440657, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 196608, 1212418,
                    1245189, 1310737, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 3309571, 4259848, 4358153, 4423690, 4489227, 4554764,
                    4620301, 4685838, 4751375, 4816912, 4325397, 3014656, 3244034,
                    3342341, 3440657, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 294912, 1277954, 1474565, 1507345, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 3407878, 2326536,
                    2424841, 2490378, 2555915, 2621452, 2686989, 2752526, 2818063,
                    2883600, 2392085, 3014656, 3244034, 3342341, 3440657, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 3014656, 3244034,
                    3342341, 3440657, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 3506184, 3604489, 3670026, 3735563, 3801100, 3866637,
                    3932174, 3997711, 4063248, 3571733, 3014656, 3244034, 3342341,
                    3440657, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 3571733, 3014656, 3244034, 3342341, 3440657, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 3014656,
                    3244034, 3342341, 3440657, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 3571733, 3014656, 3244034, 3342341, 3440657,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    3571733, 3014656, 3244034, 3342341, 3440657, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 3571733, 3014656,
                    3244034, 3342341, 3440657, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 3571733, 3014656, 3244034, 3342341, 3440657,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    3571733, 3014656, 3244034, 3342341, 3440657, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 3571733, 3014656,
                    3244034, 3342341, 3440657, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 3571733, 3014656, 3244034, 3342341, 3440657,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    3571733, 4161543, 3506184, 3604489, 3670026, 3735563, 3801100,
                    3866637, 3932174, 3997711, 4063248, 3571733, 3014656, 3244034,
                    3342341, 3440657, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 3571733, 3571733, 196608, 1212418, 1245189, 1310737,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    4325397, 196608, 1212418, 1245189, 1310737, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 196608, 1212418,
                    1245189, 1310737, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 4325397, 196608, 1212418, 1245189, 1310737, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 4325397,
                    196608, 1212418, 1245189, 1310737, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 4325397, 196608, 1212418,
                    1245189, 1310737, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 4325397, 196608, 1212418, 1245189, 1310737, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 4325397,
                    196608, 1212418, 1245189, 1310737, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 4325397, 196608, 1212418,
                    1245189, 1310737, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 4325397, 196608, 1212418, 1245189, 1310737, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 4325397,
                    4915207, 3506184, 3604489, 3670026, 3735563, 3801100, 3866637,
                    3932174, 3997711, 4063248, 3571733, 1048576, 1179650, 3145733,
                    4947985, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 1048576, 1179650, 3145733, 4947985, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 5013512, 5111817,
                    5177354, 5242891, 5308428, 5373965, 5439502, 5505039, 5570576,
                    5079061, 1048576, 1179650, 3145733, 4947985, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 5079061, 1048576,
                    1179650, 3145733, 4947985, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 1048576, 1179650, 3145733, 4947985, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 5079061,
                    1048576, 1179650, 3145733, 4947985, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 5079061, 1048576, 1179650,
                    3145733, 4947985, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 5079061, 1048576, 1179650, 3145733, 4947985, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 5079061,
                    1048576, 1179650, 3145733, 4947985, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 5079061, 1048576, 1179650,
                    3145733, 4947985, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 5079061, 1048576, 1179650, 3145733, 4947985, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 5079061,
                    1048576, 1179650, 3145733, 4947985, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 5079061, 5079061, 5701636,
                    5079061, 3014656, 3244034, 3342341, 3440657, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 3571733, 98304,
                    163842, 262149, 360465, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 753685, 98304, 163842, 262149, 360465,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    753685, 98304, 163842, 262149, 360465, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 753685, 98304, 163842, 262149,
                    360465, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 753685, 688136, 819209, 884746, 950283, 6062092, 5767181,
                    5832718, 5898255, 5963792, 753685, 1048576, 1179650, 3145733,
                    4947985, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 6127620, 5079061, 3014656, 3244034, 3342341, 3440657,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    3571733, 6225927, 3506184, 3604489, 3670026, 3735563, 3801100,
                    3866637, 3932174, 3997711, 4063248, 3571733, 294912, 1277954,
                    1474565, 1507345, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 2392085, 6324230, 2326536, 2424841, 2490378, 2555915,
                    2621452, 2686989, 2752526, 2818063, 2883600, 2392085, 3014656,
                    3244034, 3342341, 3440657, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 6389767, 3506184, 3604489, 3670026, 3735563,
                    3801100, 3866637, 3932174, 3997711, 4063248, 3571733, 458752,
                    1343490, 1441797, 1540113, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 1671189, 1671189, 4259848, 4358153, 4423690,
                    4489227, 4554764, 4620301, 4685838, 4751375, 4816912, 4325397,
                    6553603, 4259848, 4358153, 4423690, 4489227, 4554764, 4620301,
                    4685838, 4751375, 4816912, 4325397, 294912, 1277954, 1474565,
                    1507345, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 2392085, 6651910, 2326536, 2424841, 2490378, 2555915,
                    2621452, 2686989, 2752526, 2818063, 2883600, 2392085, 3014656,
                    3244034, 3342341, 3440657, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 6717447, 3506184, 3604489, 3670026, 3735563,
                    3801100, 3866637, 3932174, 3997711, 4063248, 3571733, 196608,
                    1212418, 1245189, 1310737, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 4325397, 6815747, 4259848, 4358153, 4423690,
                    4489227, 4554764, 4620301, 4685838, 4751375, 4816912, 4325397,
                    196608, 1212418, 1245189, 1310737, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 4325397, 6914051, 4259848,
                    4358153, 4423690, 4489227, 4554764, 4620301, 4685838, 4751375,
                    4816912, 4325397, 1048576, 1179650, 3145733, 4947985, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 5079061, 7012356,
                    5079061, 1048576, 1179650, 3145733, 4947985, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 5079061, 688136,
                    819209, 884746, 950283, 7110668, 5767181, 5832718, 5898255, 5963792,
                    753685, 1048576, 1179650, 3145733, 4947985, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 7176196, 5079061,
                    1048576, 1179650, 3145733, 4947985, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 5079061, 7274500, 5079061,
                    458752, 1343490, 1441797, 1540113, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 1671189, 688136, 819209, 884746,
                    950283, 7372812, 5767181, 5832718, 5898255, 5963792, 753685, 1048576,
                    1179650, 3145733, 4947985, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 7438340, 5079061, 458752, 1343490, 1441797,
                    1540113, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 1671189, 688136, 819209, 884746, 950283, 7536652,
                    5767181, 5832718, 5898255, 5963792, 753685, 98304, 163842, 262149,
                    360465, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 753685, 688136, 819209, 884746, 950283, 7634956, 5767181,
                    5832718, 5898255, 5963792, 753685, 1048576, 1179650, 3145733,
                    4947985, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 7700484, 5079061, 294912, 1277954, 1474565, 1507345,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    2392085, 688136, 819209, 884746, 950283, 7798796, 5767181, 5832718,
                    5898255, 5963792, 753685, 1048576, 1179650, 3145733, 4947985, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 7864324,
                    5079061, 294912, 1277954, 1474565, 1507345, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 2392085, 7962630,
                    2326536, 2424841, 2490378, 2555915, 2621452, 2686989, 2752526,
                    2818063, 2883600, 2392085, 3014656, 3244034, 3342341, 3440657,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    8028167, 3506184, 3604489, 3670026, 3735563, 3801100, 3866637,
                    3932174, 3997711, 4063248, 3571733, 98304, 163842, 262149, 360465,
                    393234, 426003, 2148007958, 2148040727, 2148073496, 2148106265,
                    753685, 688136, 819209, 884746, 950283, 8126476, 5767181, 5832718,
                    5898255, 5963792, 753685, 1048576, 1179650, 3145733, 4947985, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 8192004,
                    5079061, 196608, 1212418, 1245189, 1310737, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 4325397, 688136,
                    819209, 884746, 950283, 8290316, 5767181, 5832718, 5898255, 5963792,
                    753685, 1048576, 1179650, 3145733, 4947985, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 8355844, 5079061,
                    196608, 1212418, 1245189, 1310737, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 4325397, 8454147, 4259848,
                    4358153, 4423690, 4489227, 4554764, 4620301, 4685838, 4751375,
                    4816912, 4325397, 98304, 163842, 262149, 360465, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 753685, 688136,
                    819209, 884746, 950283, 8552460, 5767181, 5832718, 5898255, 5963792,
                    753685, 1048576, 1179650, 3145733, 4947985, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 8617988, 5079061,
                    98304, 163842, 262149, 360465, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 753685, 688136, 819209, 884746,
                    950283, 8716300, 5767181, 5832718, 5898255, 5963792, 753685, 1048576,
                    1179650, 3145733, 4947985, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 8781828, 5079061, 98304, 163842, 262149,
                    360465, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 753685, 98304, 163842, 262149, 360465, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 688136, 819209,
                    884746, 950283, 8912908, 5767181, 5832718, 5898255, 5963792, 753685,
                    1048576, 1179650, 3145733, 4947985, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 8978436, 5079061, 65536, 9011202,
                    9109509, 9273361, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 196608, 1212418, 1245189, 1310737, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 9076739, 4259848,
                    4358153, 4423690, 4489227, 4554764, 4620301, 4685838, 4751375,
                    4816912, 4325397, 65536, 9011202, 9109509, 9273361, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 294912, 1277954,
                    1474565, 1507345, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 9175046, 2326536, 2424841, 2490378, 2555915, 2621452,
                    2686989, 2752526, 2818063, 2883600, 2392085, 3014656, 3244034,
                    3342341, 3440657, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 9240583, 3506184, 3604489, 3670026, 3735563, 3801100,
                    3866637, 3932174, 3997711, 4063248, 3571733, 65536, 9011202, 9109509,
                    9273361, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 65536, 9011202, 9109509, 9273361, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 9338888, 9437193,
                    9502730, 9568267, 9633804, 9699341, 9764878, 9830415, 9895952,
                    9404437, 65536, 9011202, 9109509, 9273361, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 9404437, 65536,
                    9011202, 9109509, 9273361, 393234, 426003, 2148007958, 2148040727,
                    2148073496, 2148106265, 65536, 9011202, 9109509, 9273361, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 9404437,
                    65536, 9011202, 9109509, 9273361, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 9404437, 65536, 9011202, 9109509,
                    9273361, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 9404437, 65536, 9011202, 9109509, 9273361, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 9404437,
                    65536, 9011202, 9109509, 9273361, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 9404437, 65536, 9011202, 9109509,
                    9273361, 393234, 426003, 2148007958, 2148040727, 2148073496,
                    2148106265, 9404437, 65536, 9011202, 9109509, 9273361, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 9404437,
                    65536, 9011202, 9109509, 9273361, 393234, 426003, 2148007958,
                    2148040727, 2148073496, 2148106265, 9404437, 9404437, 9404437,
                    9404437, 688136, 819209, 884746, 950283, 10092556, 5767181, 5832718,
                    5898255, 5963792, 753685, 1048576, 1179650, 3145733, 4947985, 393234,
                    426003, 2148007958, 2148040727, 2148073496, 2148106265, 10158084,
                    5079061, 65536, 9011202, 9109509, 9273361, 393234, 426003,
                    2148007958, 2148040727, 2148073496, 2148106265, 9404437, 9338888,
                    9437193, 9502730, 9568267, 9633804, 9699341, 9764878, 9830415,
                    9895952, 9404437, 2157740060,
                ];
                static SHIFT_TERM_OFFSETS: &[u32] = &[
                    0, 1, 11, 22, 33, 43, 53, 64, 74, 84, 95, 105, 115, 115, 125, 136,
                    146, 146, 146, 146, 146, 156, 166, 167, 177, 177, 187, 188, 198, 199,
                    209, 210, 220, 231, 241, 251, 261, 271, 281, 291, 301, 311, 321, 332,
                    342, 352, 362, 372, 382, 393, 403, 404, 414, 424, 425, 435, 436, 446,
                    447, 457, 458, 468, 469, 479, 480, 490, 491, 501, 502, 502, 512, 522,
                    532, 533, 543, 553, 554, 564, 565, 575, 576, 586, 587, 597, 598, 608,
                    609, 619, 620, 630, 631, 642, 652, 663, 673, 683, 693, 703, 714, 724,
                    734, 745, 755, 765, 776, 786, 796, 806, 816, 817, 827, 837, 838, 848,
                    849, 859, 860, 870, 871, 881, 882, 892, 893, 903, 904, 914, 915, 926,
                    936, 937, 938, 948, 949, 959, 969, 970, 980, 981, 991, 992, 1002,
                    1003, 1013, 1014, 1024, 1025, 1035, 1036, 1046, 1047, 1058, 1068,
                    1078, 1088, 1098, 1099, 1109, 1119, 1120, 1130, 1131, 1141, 1142,
                    1152, 1153, 1163, 1164, 1174, 1175, 1185, 1186, 1196, 1197, 1198,
                    1200, 1210, 1211, 1221, 1222, 1232, 1233, 1243, 1244, 1254, 1255,
                    1265, 1275, 1277, 1287, 1288, 1299, 1309, 1310, 1321, 1331, 1342,
                    1352, 1353, 1354, 1364, 1375, 1385, 1386, 1397, 1407, 1418, 1428,
                    1429, 1440, 1450, 1451, 1462, 1472, 1473, 1475, 1485, 1486, 1496,
                    1506, 1508, 1518, 1519, 1521, 1531, 1532, 1542, 1552, 1554, 1564,
                    1565, 1575, 1585, 1586, 1596, 1606, 1608, 1618, 1619, 1629, 1639,
                    1641, 1651, 1652, 1663, 1673, 1684, 1694, 1695, 1705, 1715, 1717,
                    1727, 1728, 1738, 1748, 1750, 1760, 1761, 1772, 1782, 1783, 1793,
                    1803, 1805, 1815, 1816, 1826, 1836, 1838, 1848, 1849, 1859, 1869,
                    1879, 1881, 1891, 1901, 1912, 1922, 1932, 1943, 1953, 1964, 1974,
                    1984, 1994, 2004, 2005, 2015, 2025, 2026, 2036, 2037, 2047, 2048,
                    2058, 2059, 2069, 2070, 2080, 2081, 2091, 2092, 2102, 2103, 2104,
                    2105, 2106, 2116, 2126, 2128, 2138, 2139, 2150, 2150,
                ];
                static SHIFT_NONTERM_DATA: &[u32] = &[
                    2157707264, 2157543424, 2156167168, 2156003328, 2155905024,
                    2155741184, 2155577344, 2155413504, 2155249664, 2155085824,
                    2154987520, 2149056512, 2154823680, 2148139008, 2148204544,
                    2148270080, 2148335616, 2148401152, 2148466688, 2154725376,
                    2154561536, 2148597760, 2154463232, 2154364928, 2154266624,
                    2154102784, 2154004480, 2153971712, 2148859904, 2153938944,
                    2153775104, 2150432768, 2149777408, 2149744640, 2149122048,
                    2148270080, 2149220352, 2149285888, 2149351424, 2149416960,
                    2149482496, 2149548032, 2149613568, 2149679104, 2149842944,
                    2148270080, 2149941248, 2150006784, 2150072320, 2150137856,
                    2150203392, 2150268928, 2150334464, 2150400000, 2153676800,
                    2153512960, 2150563840, 2153152512, 2150662144, 2152366080,
                    2150760448, 2151710720, 2150858752, 2151612416, 2150957056,
                    2151022592, 2148270080, 2151120896, 2151186432, 2151251968,
                    2151317504, 2151383040, 2151448576, 2151514112, 2151579648,
                    2151677952, 2151776256, 2148270080, 2151874560, 2151940096,
                    2152005632, 2152071168, 2152136704, 2152202240, 2152267776,
                    2152333312, 2153119744, 2152464384, 2152529920, 2148270080,
                    2152628224, 2152693760, 2152759296, 2152824832, 2152890368,
                    2152955904, 2153021440, 2153086976, 2153218048, 2153283584,
                    2153349120, 2153414656, 2153480192, 2153578496, 2153644032,
                    2153742336, 2153840640, 2153906176, 2154070016, 2154168320,
                    2154233856, 2154332160, 2154430464, 2154528768, 2154627072,
                    2154692608, 2154790912, 2154889216, 2154954752, 2155053056,
                    2155151360, 2155216896, 2155315200, 2155380736, 2155479040,
                    2155544576, 2155642880, 2155708416, 2155806720, 2155872256,
                    2155970560, 2156068864, 2156134400, 2156232704, 2156298240,
                    2156363776, 2156429312, 2157510656, 2156527616, 2157477888,
                    2156625920, 2156691456, 2157445120, 2156789760, 2156855296,
                    2148270080, 2156953600, 2157019136, 2157084672, 2157150208,
                    2157215744, 2157281280, 2157346816, 2157412352, 2157608960,
                    2157674496,
                ];
                static SHIFT_NONTERM_OFFSETS: &[u32] = &[
                    0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11, 12, 13, 14, 14, 14, 14,
                    14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19, 19, 20, 21, 22, 22, 23,
                    24, 25, 26, 27, 28, 29, 29, 30, 31, 32, 33, 34, 34, 35, 35, 36, 37,
                    37, 38, 38, 39, 39, 40, 40, 41, 41, 42, 42, 43, 43, 44, 44, 44, 44,
                    44, 45, 45, 46, 47, 47, 48, 48, 49, 49, 50, 50, 51, 51, 52, 52, 53,
                    53, 54, 54, 54, 55, 56, 57, 57, 58, 59, 59, 60, 61, 61, 62, 63, 63,
                    64, 65, 65, 66, 66, 67, 68, 68, 69, 69, 70, 70, 71, 71, 72, 72, 73,
                    73, 74, 74, 75, 75, 75, 76, 76, 76, 77, 77, 78, 79, 79, 80, 80, 81,
                    81, 82, 82, 83, 83, 84, 84, 85, 85, 86, 86, 86, 87, 88, 88, 89, 89,
                    90, 91, 91, 92, 92, 93, 93, 94, 94, 95, 95, 96, 96, 97, 97, 98, 98,
                    98, 98, 99, 99, 100, 100, 101, 101, 102, 102, 103, 103, 103, 104,
                    104, 105, 105, 105, 106, 106, 106, 107, 107, 108, 108, 108, 108, 108,
                    109, 109, 109, 110, 110, 111, 111, 111, 112, 112, 112, 113, 113, 113,
                    114, 114, 114, 115, 115, 116, 116, 116, 117, 117, 117, 118, 118, 119,
                    119, 119, 120, 120, 120, 121, 121, 122, 122, 122, 123, 123, 124, 124,
                    124, 125, 125, 126, 126, 126, 127, 127, 128, 128, 128, 129, 129, 130,
                    130, 130, 131, 131, 131, 132, 132, 133, 133, 133, 134, 134, 135, 135,
                    136, 136, 137, 137, 138, 139, 139, 140, 141, 141, 142, 142, 143, 144,
                    144, 145, 145, 146, 147, 147, 148, 148, 149, 149, 150, 150, 151, 151,
                    152, 152, 153, 153, 154, 154, 154, 154, 154, 154, 155, 155, 156, 156,
                    156, 156,
                ];
                static REDUCE_DATA: &[u32] = &[
                    3, 1, 4, 4, 1, 4, 6, 1, 4, 7, 1, 4, 8, 1, 4, 9, 1, 4, 10, 1, 4, 11,
                    1, 4, 12, 1, 4, 13, 1, 4, 14, 1, 4, 15, 1, 4, 16, 1, 4, 20, 1, 4, 21,
                    1, 4, 28, 1, 4, 3, 1, 2, 4, 1, 2, 6, 1, 2, 7, 1, 2, 8, 1, 2, 9, 1, 2,
                    10, 1, 2, 11, 1, 2, 12, 1, 2, 13, 1, 2, 14, 1, 2, 15, 1, 2, 16, 1, 2,
                    20, 1, 2, 21, 1, 2, 28, 1, 2, 3, 1, 0, 4, 1, 0, 6, 1, 0, 7, 1, 0, 8,
                    1, 0, 9, 1, 0, 10, 1, 0, 11, 1, 0, 12, 1, 0, 13, 1, 0, 14, 1, 0, 15,
                    1, 0, 16, 1, 0, 20, 1, 0, 21, 1, 0, 28, 1, 0, 3, 1, 1, 4, 1, 1, 6, 1,
                    1, 7, 1, 1, 8, 1, 1, 9, 1, 1, 10, 1, 1, 11, 1, 1, 12, 1, 1, 13, 1, 1,
                    14, 1, 1, 15, 1, 1, 16, 1, 1, 20, 1, 1, 21, 1, 1, 28, 1, 1, 3, 1, 3,
                    4, 1, 3, 6, 1, 3, 7, 1, 3, 8, 1, 3, 9, 1, 3, 10, 1, 3, 11, 1, 3, 12,
                    1, 3, 13, 1, 3, 14, 1, 3, 15, 1, 3, 16, 1, 3, 20, 1, 3, 21, 1, 3, 28,
                    1, 3, 8, 1, 8, 9, 1, 8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 13, 1, 8, 14,
                    1, 8, 15, 1, 8, 16, 1, 8, 3, 1, 20, 4, 1, 20, 6, 1, 20, 7, 1, 20, 8,
                    1, 20, 9, 1, 20, 10, 1, 20, 11, 1, 20, 12, 1, 20, 13, 1, 20, 14, 1,
                    20, 15, 1, 20, 16, 1, 20, 20, 1, 20, 21, 1, 20, 28, 1, 20, 8, 1, 9,
                    9, 1, 9, 10, 1, 9, 11, 1, 9, 12, 1, 9, 13, 1, 9, 14, 1, 9, 15, 1, 9,
                    16, 1, 9, 8, 1, 10, 9, 1, 10, 10, 1, 10, 11, 1, 10, 12, 1, 10, 13, 1,
                    10, 14, 1, 10, 15, 1, 10, 16, 1, 10, 8, 1, 11, 9, 1, 11, 10, 1, 11,
                    11, 1, 11, 12, 1, 11, 13, 1, 11, 14, 1, 11, 15, 1, 11, 16, 1, 11, 8,
                    1, 8, 9, 1, 8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 13, 1, 8, 14, 1, 8, 15,
                    1, 8, 16, 1, 8, 20, 1, 8, 8, 1, 9, 9, 1, 9, 10, 1, 9, 11, 1, 9, 12,
                    1, 9, 13, 1, 9, 14, 1, 9, 15, 1, 9, 16, 1, 9, 20, 1, 9, 8, 1, 10, 9,
                    1, 10, 10, 1, 10, 11, 1, 10, 12, 1, 10, 13, 1, 10, 14, 1, 10, 15, 1,
                    10, 16, 1, 10, 20, 1, 10, 8, 1, 11, 9, 1, 11, 10, 1, 11, 11, 1, 11,
                    12, 1, 11, 13, 1, 11, 14, 1, 11, 15, 1, 11, 16, 1, 11, 20, 1, 11, 8,
                    1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1,
                    12, 15, 1, 12, 16, 1, 12, 20, 1, 12, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11,
                    1, 6, 12, 1, 6, 13, 1, 6, 14, 1, 6, 15, 1, 6, 16, 1, 6, 20, 1, 6, 8,
                    1, 7, 9, 1, 7, 10, 1, 7, 11, 1, 7, 12, 1, 7, 13, 1, 7, 14, 1, 7, 15,
                    1, 7, 16, 1, 7, 20, 1, 7, 8, 1, 13, 9, 1, 13, 10, 1, 13, 11, 1, 13,
                    12, 1, 13, 13, 1, 13, 14, 1, 13, 15, 1, 13, 16, 1, 13, 20, 1, 13, 8,
                    1, 14, 9, 1, 14, 10, 1, 14, 11, 1, 14, 12, 1, 14, 13, 1, 14, 14, 1,
                    14, 15, 1, 14, 16, 1, 14, 20, 1, 14, 3, 1, 5, 4, 1, 5, 6, 1, 5, 7, 1,
                    5, 8, 1, 5, 9, 1, 5, 10, 1, 5, 11, 1, 5, 12, 1, 5, 13, 1, 5, 14, 1,
                    5, 15, 1, 5, 16, 1, 5, 20, 1, 5, 21, 1, 5, 28, 1, 5, 20, 1, 15, 6, 1,
                    15, 6, 1, 8, 8, 1, 8, 9, 1, 8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 13, 1,
                    8, 14, 1, 8, 15, 1, 8, 16, 1, 8, 6, 1, 9, 8, 1, 9, 9, 1, 9, 10, 1, 9,
                    11, 1, 9, 12, 1, 9, 13, 1, 9, 14, 1, 9, 15, 1, 9, 16, 1, 9, 6, 1, 10,
                    8, 1, 10, 9, 1, 10, 10, 1, 10, 11, 1, 10, 12, 1, 10, 13, 1, 10, 14,
                    1, 10, 15, 1, 10, 16, 1, 10, 6, 1, 11, 8, 1, 11, 9, 1, 11, 10, 1, 11,
                    11, 1, 11, 12, 1, 11, 13, 1, 11, 14, 1, 11, 15, 1, 11, 16, 1, 11, 6,
                    1, 12, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1,
                    12, 14, 1, 12, 15, 1, 12, 16, 1, 12, 6, 1, 6, 8, 1, 6, 9, 1, 6, 10,
                    1, 6, 11, 1, 6, 12, 1, 6, 13, 1, 6, 14, 1, 6, 15, 1, 6, 16, 1, 6, 6,
                    1, 7, 8, 1, 7, 9, 1, 7, 10, 1, 7, 11, 1, 7, 12, 1, 7, 13, 1, 7, 14,
                    1, 7, 15, 1, 7, 16, 1, 7, 6, 1, 13, 8, 1, 13, 9, 1, 13, 10, 1, 13,
                    11, 1, 13, 12, 1, 13, 13, 1, 13, 14, 1, 13, 15, 1, 13, 16, 1, 13, 6,
                    1, 14, 8, 1, 14, 9, 1, 14, 10, 1, 14, 11, 1, 14, 12, 1, 14, 13, 1,
                    14, 14, 1, 14, 15, 1, 14, 16, 1, 14, 7, 1, 15, 7, 1, 8, 8, 1, 8, 9,
                    1, 8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 13, 1, 8, 14, 1, 8, 15, 1, 8, 16,
                    1, 8, 7, 1, 9, 8, 1, 9, 9, 1, 9, 10, 1, 9, 11, 1, 9, 12, 1, 9, 13, 1,
                    9, 14, 1, 9, 15, 1, 9, 16, 1, 9, 7, 1, 10, 8, 1, 10, 9, 1, 10, 10, 1,
                    10, 11, 1, 10, 12, 1, 10, 13, 1, 10, 14, 1, 10, 15, 1, 10, 16, 1, 10,
                    7, 1, 11, 8, 1, 11, 9, 1, 11, 10, 1, 11, 11, 1, 11, 12, 1, 11, 13, 1,
                    11, 14, 1, 11, 15, 1, 11, 16, 1, 11, 7, 1, 12, 8, 1, 12, 9, 1, 12,
                    10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1, 12, 15, 1, 12, 16,
                    1, 12, 7, 1, 6, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11, 1, 6, 12, 1, 6, 13,
                    1, 6, 14, 1, 6, 15, 1, 6, 16, 1, 6, 7, 1, 7, 8, 1, 7, 9, 1, 7, 10, 1,
                    7, 11, 1, 7, 12, 1, 7, 13, 1, 7, 14, 1, 7, 15, 1, 7, 16, 1, 7, 7, 1,
                    13, 8, 1, 13, 9, 1, 13, 10, 1, 13, 11, 1, 13, 12, 1, 13, 13, 1, 13,
                    14, 1, 13, 15, 1, 13, 16, 1, 13, 7, 1, 14, 8, 1, 14, 9, 1, 14, 10, 1,
                    14, 11, 1, 14, 12, 1, 14, 13, 1, 14, 14, 1, 14, 15, 1, 14, 16, 1, 14,
                    7, 1, 19, 8, 1, 19, 9, 1, 19, 10, 1, 19, 11, 1, 19, 12, 1, 19, 13, 1,
                    19, 14, 1, 19, 15, 1, 19, 16, 1, 19, 7, 1, 18, 8, 1, 18, 9, 1, 18,
                    10, 1, 18, 11, 1, 18, 12, 1, 18, 13, 1, 18, 14, 1, 18, 15, 1, 18, 16,
                    1, 18, 3, 1, 8, 8, 1, 8, 9, 1, 8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 13,
                    1, 8, 14, 1, 8, 15, 1, 8, 16, 1, 8, 3, 1, 9, 8, 1, 9, 9, 1, 9, 10, 1,
                    9, 11, 1, 9, 12, 1, 9, 13, 1, 9, 14, 1, 9, 15, 1, 9, 16, 1, 9, 3, 1,
                    10, 8, 1, 10, 9, 1, 10, 10, 1, 10, 11, 1, 10, 12, 1, 10, 13, 1, 10,
                    14, 1, 10, 15, 1, 10, 16, 1, 10, 3, 1, 11, 8, 1, 11, 9, 1, 11, 10, 1,
                    11, 11, 1, 11, 12, 1, 11, 13, 1, 11, 14, 1, 11, 15, 1, 11, 16, 1, 11,
                    3, 1, 12, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1,
                    12, 14, 1, 12, 15, 1, 12, 16, 1, 12, 3, 1, 6, 8, 1, 6, 9, 1, 6, 10,
                    1, 6, 11, 1, 6, 12, 1, 6, 13, 1, 6, 14, 1, 6, 15, 1, 6, 16, 1, 6, 3,
                    1, 7, 8, 1, 7, 9, 1, 7, 10, 1, 7, 11, 1, 7, 12, 1, 7, 13, 1, 7, 14,
                    1, 7, 15, 1, 7, 16, 1, 7, 3, 1, 13, 8, 1, 13, 9, 1, 13, 10, 1, 13,
                    11, 1, 13, 12, 1, 13, 13, 1, 13, 14, 1, 13, 15, 1, 13, 16, 1, 13, 3,
                    1, 14, 8, 1, 14, 9, 1, 14, 10, 1, 14, 11, 1, 14, 12, 1, 14, 13, 1,
                    14, 14, 1, 14, 15, 1, 14, 16, 1, 14, 4, 1, 15, 4, 1, 8, 8, 1, 8, 9,
                    1, 8, 10, 1, 8, 11, 1, 8, 12, 1, 8, 13, 1, 8, 14, 1, 8, 15, 1, 8, 16,
                    1, 8, 4, 1, 9, 8, 1, 9, 9, 1, 9, 10, 1, 9, 11, 1, 9, 12, 1, 9, 13, 1,
                    9, 14, 1, 9, 15, 1, 9, 16, 1, 9, 4, 1, 10, 8, 1, 10, 9, 1, 10, 10, 1,
                    10, 11, 1, 10, 12, 1, 10, 13, 1, 10, 14, 1, 10, 15, 1, 10, 16, 1, 10,
                    4, 1, 11, 8, 1, 11, 9, 1, 11, 10, 1, 11, 11, 1, 11, 12, 1, 11, 13, 1,
                    11, 14, 1, 11, 15, 1, 11, 16, 1, 11, 4, 1, 12, 8, 1, 12, 9, 1, 12,
                    10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1, 12, 15, 1, 12, 16,
                    1, 12, 4, 1, 6, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11, 1, 6, 12, 1, 6, 13,
                    1, 6, 14, 1, 6, 15, 1, 6, 16, 1, 6, 4, 1, 7, 8, 1, 7, 9, 1, 7, 10, 1,
                    7, 11, 1, 7, 12, 1, 7, 13, 1, 7, 14, 1, 7, 15, 1, 7, 16, 1, 7, 4, 1,
                    13, 8, 1, 13, 9, 1, 13, 10, 1, 13, 11, 1, 13, 12, 1, 13, 13, 1, 13,
                    14, 1, 13, 15, 1, 13, 16, 1, 13, 4, 1, 14, 8, 1, 14, 9, 1, 14, 10, 1,
                    14, 11, 1, 14, 12, 1, 14, 13, 1, 14, 14, 1, 14, 15, 1, 14, 16, 1, 14,
                    4, 1, 19, 8, 1, 19, 9, 1, 19, 10, 1, 19, 11, 1, 19, 12, 1, 19, 13, 1,
                    19, 14, 1, 19, 15, 1, 19, 16, 1, 19, 8, 1, 12, 9, 1, 12, 10, 1, 12,
                    11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1, 12, 15, 1, 12, 16, 1, 12, 7,
                    1, 17, 8, 1, 17, 9, 1, 17, 10, 1, 17, 11, 1, 17, 12, 1, 17, 13, 1,
                    17, 14, 1, 17, 15, 1, 17, 16, 1, 17, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11,
                    1, 6, 12, 1, 6, 13, 1, 6, 14, 1, 6, 15, 1, 6, 16, 1, 6, 8, 1, 7, 9,
                    1, 7, 10, 1, 7, 11, 1, 7, 12, 1, 7, 13, 1, 7, 14, 1, 7, 15, 1, 7, 16,
                    1, 7, 8, 1, 13, 9, 1, 13, 10, 1, 13, 11, 1, 13, 12, 1, 13, 13, 1, 13,
                    14, 1, 13, 15, 1, 13, 16, 1, 13, 8, 1, 14, 9, 1, 14, 10, 1, 14, 11,
                    1, 14, 12, 1, 14, 13, 1, 14, 14, 1, 14, 15, 1, 14, 16, 1, 14, 8, 1,
                    12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1, 12,
                    15, 1, 12, 16, 1, 12, 7, 1, 16, 8, 1, 16, 9, 1, 16, 10, 1, 16, 11, 1,
                    16, 12, 1, 16, 13, 1, 16, 14, 1, 16, 15, 1, 16, 16, 1, 16, 6, 1, 19,
                    8, 1, 19, 9, 1, 19, 10, 1, 19, 11, 1, 19, 12, 1, 19, 13, 1, 19, 14,
                    1, 19, 15, 1, 19, 16, 1, 19, 8, 1, 19, 9, 1, 19, 10, 1, 19, 11, 1,
                    19, 12, 1, 19, 13, 1, 19, 14, 1, 19, 15, 1, 19, 16, 1, 19, 20, 1, 19,
                    8, 1, 18, 9, 1, 18, 10, 1, 18, 11, 1, 18, 12, 1, 18, 13, 1, 18, 14,
                    1, 18, 15, 1, 18, 16, 1, 18, 20, 1, 18, 3, 1, 15, 6, 1, 18, 8, 1, 18,
                    9, 1, 18, 10, 1, 18, 11, 1, 18, 12, 1, 18, 13, 1, 18, 14, 1, 18, 15,
                    1, 18, 16, 1, 18, 3, 1, 19, 8, 1, 19, 9, 1, 19, 10, 1, 19, 11, 1, 19,
                    12, 1, 19, 13, 1, 19, 14, 1, 19, 15, 1, 19, 16, 1, 19, 3, 1, 18, 8,
                    1, 18, 9, 1, 18, 10, 1, 18, 11, 1, 18, 12, 1, 18, 13, 1, 18, 14, 1,
                    18, 15, 1, 18, 16, 1, 18, 4, 1, 18, 8, 1, 18, 9, 1, 18, 10, 1, 18,
                    11, 1, 18, 12, 1, 18, 13, 1, 18, 14, 1, 18, 15, 1, 18, 16, 1, 18, 8,
                    1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1,
                    12, 15, 1, 12, 16, 1, 12, 4, 1, 17, 8, 1, 17, 9, 1, 17, 10, 1, 17,
                    11, 1, 17, 12, 1, 17, 13, 1, 17, 14, 1, 17, 15, 1, 17, 16, 1, 17, 8,
                    1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1,
                    12, 15, 1, 12, 16, 1, 12, 4, 1, 16, 8, 1, 16, 9, 1, 16, 10, 1, 16,
                    11, 1, 16, 12, 1, 16, 13, 1, 16, 14, 1, 16, 15, 1, 16, 16, 1, 16, 8,
                    1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1,
                    12, 15, 1, 12, 16, 1, 12, 8, 1, 17, 9, 1, 17, 10, 1, 17, 11, 1, 17,
                    12, 1, 17, 13, 1, 17, 14, 1, 17, 15, 1, 17, 16, 1, 17, 20, 1, 17, 8,
                    1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1,
                    12, 15, 1, 12, 16, 1, 12, 8, 1, 16, 9, 1, 16, 10, 1, 16, 11, 1, 16,
                    12, 1, 16, 13, 1, 16, 14, 1, 16, 15, 1, 16, 16, 1, 16, 20, 1, 16, 8,
                    1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1,
                    12, 15, 1, 12, 16, 1, 12, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12,
                    12, 1, 12, 13, 1, 12, 14, 1, 12, 15, 1, 12, 16, 1, 12, 6, 1, 17, 8,
                    1, 17, 9, 1, 17, 10, 1, 17, 11, 1, 17, 12, 1, 17, 13, 1, 17, 14, 1,
                    17, 15, 1, 17, 16, 1, 17, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12,
                    12, 1, 12, 13, 1, 12, 14, 1, 12, 15, 1, 12, 16, 1, 12, 6, 1, 16, 8,
                    1, 16, 9, 1, 16, 10, 1, 16, 11, 1, 16, 12, 1, 16, 13, 1, 16, 14, 1,
                    16, 15, 1, 16, 16, 1, 16, 8, 1, 19, 9, 1, 19, 10, 1, 19, 11, 1, 19,
                    12, 1, 19, 13, 1, 19, 14, 1, 19, 15, 1, 19, 16, 1, 19, 8, 1, 12, 9,
                    1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1, 12, 15, 1,
                    12, 16, 1, 12, 3, 1, 17, 8, 1, 17, 9, 1, 17, 10, 1, 17, 11, 1, 17,
                    12, 1, 17, 13, 1, 17, 14, 1, 17, 15, 1, 17, 16, 1, 17, 8, 1, 12, 9,
                    1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1, 12, 15, 1,
                    12, 16, 1, 12, 3, 1, 16, 8, 1, 16, 9, 1, 16, 10, 1, 16, 11, 1, 16,
                    12, 1, 16, 13, 1, 16, 14, 1, 16, 15, 1, 16, 16, 1, 16, 8, 1, 18, 9,
                    1, 18, 10, 1, 18, 11, 1, 18, 12, 1, 18, 13, 1, 18, 14, 1, 18, 15, 1,
                    18, 16, 1, 18, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12,
                    13, 1, 12, 14, 1, 12, 15, 1, 12, 16, 1, 12, 8, 1, 17, 9, 1, 17, 10,
                    1, 17, 11, 1, 17, 12, 1, 17, 13, 1, 17, 14, 1, 17, 15, 1, 17, 16, 1,
                    17, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12,
                    14, 1, 12, 15, 1, 12, 16, 1, 12, 8, 1, 16, 9, 1, 16, 10, 1, 16, 11,
                    1, 16, 12, 1, 16, 13, 1, 16, 14, 1, 16, 15, 1, 16, 16, 1, 16, 8, 1,
                    12, 9, 1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1, 12,
                    15, 1, 12, 16, 1, 12, 28, 1, 15, 8, 1, 8, 9, 1, 8, 10, 1, 8, 11, 1,
                    8, 12, 1, 8, 13, 1, 8, 14, 1, 8, 15, 1, 8, 16, 1, 8, 28, 1, 8, 8, 1,
                    9, 9, 1, 9, 10, 1, 9, 11, 1, 9, 12, 1, 9, 13, 1, 9, 14, 1, 9, 15, 1,
                    9, 16, 1, 9, 28, 1, 9, 8, 1, 10, 9, 1, 10, 10, 1, 10, 11, 1, 10, 12,
                    1, 10, 13, 1, 10, 14, 1, 10, 15, 1, 10, 16, 1, 10, 28, 1, 10, 8, 1,
                    11, 9, 1, 11, 10, 1, 11, 11, 1, 11, 12, 1, 11, 13, 1, 11, 14, 1, 11,
                    15, 1, 11, 16, 1, 11, 28, 1, 11, 8, 1, 12, 9, 1, 12, 10, 1, 12, 11,
                    1, 12, 12, 1, 12, 13, 1, 12, 14, 1, 12, 15, 1, 12, 16, 1, 12, 28, 1,
                    12, 8, 1, 6, 9, 1, 6, 10, 1, 6, 11, 1, 6, 12, 1, 6, 13, 1, 6, 14, 1,
                    6, 15, 1, 6, 16, 1, 6, 28, 1, 6, 8, 1, 7, 9, 1, 7, 10, 1, 7, 11, 1,
                    7, 12, 1, 7, 13, 1, 7, 14, 1, 7, 15, 1, 7, 16, 1, 7, 28, 1, 7, 8, 1,
                    13, 9, 1, 13, 10, 1, 13, 11, 1, 13, 12, 1, 13, 13, 1, 13, 14, 1, 13,
                    15, 1, 13, 16, 1, 13, 28, 1, 13, 8, 1, 14, 9, 1, 14, 10, 1, 14, 11,
                    1, 14, 12, 1, 14, 13, 1, 14, 14, 1, 14, 15, 1, 14, 16, 1, 14, 28, 1,
                    14, 8, 1, 19, 9, 1, 19, 10, 1, 19, 11, 1, 19, 12, 1, 19, 13, 1, 19,
                    14, 1, 19, 15, 1, 19, 16, 1, 19, 28, 1, 19, 8, 1, 18, 9, 1, 18, 10,
                    1, 18, 11, 1, 18, 12, 1, 18, 13, 1, 18, 14, 1, 18, 15, 1, 18, 16, 1,
                    18, 28, 1, 18, 8, 1, 17, 9, 1, 17, 10, 1, 17, 11, 1, 17, 12, 1, 17,
                    13, 1, 17, 14, 1, 17, 15, 1, 17, 16, 1, 17, 28, 1, 17, 8, 1, 12, 9,
                    1, 12, 10, 1, 12, 11, 1, 12, 12, 1, 12, 13, 1, 12, 14, 1, 12, 15, 1,
                    12, 16, 1, 12, 8, 1, 16, 9, 1, 16, 10, 1, 16, 11, 1, 16, 12, 1, 16,
                    13, 1, 16, 14, 1, 16, 15, 1, 16, 16, 1, 16, 28, 1, 16,
                ];
                static REDUCE_OFFSETS: &[u32] = &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 48, 48, 48, 96, 144, 192,
                    240, 240, 240, 267, 267, 315, 315, 342, 342, 369, 369, 396, 396, 396,
                    396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396, 396,
                    396, 396, 396, 426, 426, 426, 456, 456, 486, 486, 516, 516, 546, 546,
                    576, 576, 606, 606, 636, 636, 666, 714, 717, 720, 720, 750, 750, 750,
                    780, 780, 810, 810, 840, 840, 870, 870, 900, 900, 930, 930, 960, 960,
                    990, 990, 990, 990, 990, 990, 990, 990, 990, 990, 990, 990, 990, 990,
                    990, 990, 990, 993, 993, 1023, 1023, 1023, 1053, 1053, 1083, 1083,
                    1113, 1113, 1143, 1143, 1173, 1173, 1203, 1203, 1233, 1233, 1263,
                    1263, 1263, 1293, 1323, 1323, 1353, 1353, 1353, 1383, 1383, 1413,
                    1413, 1443, 1443, 1473, 1473, 1503, 1503, 1533, 1533, 1563, 1563,
                    1593, 1593, 1593, 1593, 1596, 1596, 1626, 1626, 1626, 1656, 1656,
                    1686, 1686, 1716, 1716, 1746, 1746, 1776, 1776, 1806, 1806, 1836,
                    1836, 1866, 1896, 1923, 1923, 1953, 1953, 1980, 1980, 2007, 2007,
                    2034, 2034, 2061, 2061, 2061, 2088, 2088, 2118, 2118, 2118, 2148,
                    2148, 2148, 2148, 2148, 2178, 2208, 2211, 2211, 2211, 2241, 2241,
                    2241, 2241, 2241, 2271, 2271, 2271, 2301, 2301, 2301, 2331, 2358,
                    2358, 2388, 2388, 2388, 2415, 2415, 2445, 2472, 2472, 2502, 2502,
                    2502, 2529, 2529, 2559, 2559, 2559, 2586, 2586, 2586, 2613, 2613,
                    2643, 2643, 2643, 2670, 2670, 2700, 2700, 2700, 2700, 2700, 2727,
                    2727, 2727, 2754, 2754, 2784, 2784, 2784, 2811, 2811, 2841, 2841,
                    2841, 2868, 2868, 2868, 2895, 2895, 2922, 2922, 2922, 2949, 2949,
                    2976, 2976, 2976, 2976, 3003, 3003, 3003, 3003, 3003, 3003, 3003,
                    3003, 3003, 3003, 3003, 3006, 3006, 3036, 3036, 3036, 3066, 3066,
                    3096, 3096, 3126, 3126, 3156, 3156, 3186, 3186, 3216, 3216, 3246,
                    3246, 3276, 3306, 3336, 3366, 3366, 3366, 3393, 3393, 3423, 3423,
                    3423,
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
                let num_states = 314usize;
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
        