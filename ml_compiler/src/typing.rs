use crate::enums::Expr;
use crate::enums::Type;
use std::cell::Cell;

pub fn type_check(ast: Expr, env: Vec<(Type, Type)>) -> (Type, Vec<(Type, Type)>) {
    match ast {
        Expr::INT(_) => (Type::INT, env),
        Expr::STRING(_) => (Type::STRING, env),
        Expr::BOOL(_) => (Type::BOOL, env),
        Expr::UNIT => (Type::UNIT, env),
        Expr::ID(x) => match lookup_env(&env, Type::ID(x.clone())) {
            Some(t) => (t, env),
            None => {
                let alpha = new_type_id();
                let mut env2 = env.clone();
                env2.push((Type::ID(x.clone()), alpha.clone()));
                (alpha.clone(), env2)
            }
        },
        Expr::ADD(x, y) | Expr::SUB(x, y) => {
            let (t1, c1) = type_check(*x, env.clone());
            let (t2, c2) = type_check(*y, env.clone());
            let mut env2 = env.clone();
            env2.push((t1, Type::INT));
            env2.push((t2, Type::INT));
            env2.extend(c1);
            env2.extend(c2);
            (Type::INT, env2)
        }
        Expr::AND(x, y) | Expr::OR(x, y) => {
            let (t1, c1) = type_check(*x, env.clone());
            let (t2, c2) = type_check(*y, env.clone());
            let mut env2 = env.clone();
            env2.push((t1, Type::BOOL));
            env2.push((t2, Type::BOOL));
            env2.extend(c1);
            env2.extend(c2);
            (Type::BOOL, env2)
        }
        Expr::NOT(x) => {
            let (t, c) = type_check(*x, env.clone());
            let mut env2 = env.clone();
            env2.push((t, Type::BOOL));
            env2.extend(c);
            (Type::BOOL, env2)
        }
        Expr::GEQ(x, y)
        | Expr::LEQ(x, y)
        | Expr::GT(x, y)
        | Expr::LT(x, y)
        | Expr::EQ(x, y)
        | Expr::NEQ(x, y) => {
            let (t1, c1) = type_check(*x, env.clone());
            let (t2, c2) = type_check(*y, env.clone());
            let mut env2 = env.clone();
            env2.push((t1, Type::INT));
            env2.push((t2, Type::INT));
            env2.extend(c1);
            env2.extend(c2);
            (Type::BOOL, env2)
        }
        Expr::LET(x, e1, e2) => {
            let (t1, c1) = type_check(*e1, env.clone());
            let mut env2 = env.clone();
            match *x {
                Expr::ID(s) => env2.push((Type::ID(s), t1)),
                _ => {
                    return (
                        Type::ERROR("let x = e1 in e2 : x is not an identifier".to_string()),
                        Vec::new(),
                    );
                }
            };
            let (t2, c2) = type_check(*e2, env2.clone());
            let mut c = c1.clone();
            c.extend(c2);
            (t2, c)
        }
        Expr::IF(e1, e2, e3) => {
            let (t1, c1) = type_check(*e1, env.clone());
            let (t2, c2) = type_check(*e2, env.clone());
            let (t3, c3) = type_check(*e3, env.clone());
            let mut c = vec![(t1.clone(), Type::BOOL), (t2.clone(), t3.clone())];
            c.extend(c1);
            c.extend(c2);
            c.extend(c3);
            (t3, c)
        }
        Expr::FUN(x, e) => {
            let mut env2 = env.clone();
            let alpha = new_type_id();
            match *x {
                Expr::ID(s) => env2.push((Type::ID(s), alpha.clone())),
                _ => {
                    return (
                        Type::ERROR("fun x -> e : x is not an identifier".to_string()),
                        Vec::new(),
                    );
                }
            };
            let (t, c) = type_check(*e, env2);
            (Type::FUN(Box::new(alpha), Box::new(t)), c)
        }
        Expr::APP(e1, e2) => {
            let (t1, c1) = type_check(*e1, env.clone());
            let (t2, c2) = type_check(*e2, env.clone());
            let alpha = new_type_id();
            let mut c = vec![(
                t1.clone(),
                Type::FUN(Box::new(t2.clone()), Box::new(alpha.clone())),
            )];
            c.extend(c1);
            c.extend(c2);
            (alpha, c)
        }
        Expr::SEMI(e1, e2) => {
            let (t1, c1) = type_check(*e1, env.clone());
            let (t2, c2) = type_check(*e2, env.clone());
            let mut c = vec![(t1.clone(), Type::UNIT)];
            c.extend(c1);
            c.extend(c2);
            (t2, c)
        }
        Expr::LETREC(f, e1, e2) => {
            let mut env2 = env.clone();
            let alpha = new_type_id();
            match *f {
                Expr::ID(s) => env2.push((Type::ID(s), alpha.clone())),
                _ => {
                    return (
                        Type::ERROR("let rec f = e1 in e2 : f is not an identifier".to_string()),
                        Vec::new(),
                    );
                }
            };
            let (t1, c1) = type_check(*e1, env2.clone());
            let (t2, c2) = type_check(*e2, env2.clone());
            let mut c = vec![(alpha.clone(), t2.clone())];
            c.extend(c1);
            c.extend(c2);
            (t2, c)
        }

        _ => (Type::ERROR("undefined expr".to_string()), Vec::new()),
    }
}

fn lookup_env(env: &Vec<(Type, Type)>, t: Type) -> Option<Type> {
    for e in env {
        match e {
            (t1, t2) if t == *t1 => return Some(t2.clone()),
            _ => continue,
        }
    }
    None
}

fn new_type_id() -> Type {
    thread_local! {
        static TYPE_ID_COUNTER: Cell<i32> = Cell::new(0);
    }

    Type::TYPEID(TYPE_ID_COUNTER.with(|c| {
        c.set(c.get() + 1);
        c.get()
    }))
}
