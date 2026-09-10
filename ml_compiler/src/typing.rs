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

        _ => (Type::UNIT, env),
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
