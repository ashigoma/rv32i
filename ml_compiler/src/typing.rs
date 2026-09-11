use crate::enums::Expr;
use crate::enums::Type;
use std::cell::Cell;
use std::collections::HashMap;

fn get_type_and_constr(ast: Expr, env: Vec<(Type, Type)>) -> (Type, Vec<(Type, Type)>) {
    // let (ast_, env_) = (ast.clone(), env.clone());
    let r = match ast {
        Expr::INT(_) => (Type::INT, env),
        Expr::STRING(_) => (Type::STRING, env),
        Expr::BOOL(_) => (Type::BOOL, env),
        Expr::UNIT => (Type::UNIT, env),
        Expr::ID(x) => match lookup_env(&env, Type::ID(x.clone())) {
            Some(t) => (t, env),
            None => {
                let alpha = new_type_id();
                let mut c = env.clone();
                c.push((Type::ID(x.clone()), alpha.clone()));
                (alpha.clone(), c)
            }
        },
        Expr::ADD(x, y) | Expr::SUB(x, y) => {
            let (t1, c1) = get_type_and_constr(*x, env.clone());
            let (t2, c2) = get_type_and_constr(*y, env.clone());

            let mut c = Vec::new();
            c.push((t1, Type::INT));
            c.push((t2, Type::INT));
            c.extend(c1);
            c.extend(c2);
            (Type::INT, c)
        }
        Expr::AND(x, y) | Expr::OR(x, y) => {
            let (t1, c1) = get_type_and_constr(*x, env.clone());
            let (t2, c2) = get_type_and_constr(*y, env.clone());
            let mut c = env.clone();
            c.push((t1, Type::BOOL));
            c.push((t2, Type::BOOL));
            c.extend(c1);
            c.extend(c2);
            (Type::BOOL, c)
        }
        Expr::NOT(x) => {
            let (t1, c1) = get_type_and_constr(*x, env.clone());
            let mut c = env.clone();
            c.push((t1, Type::BOOL));
            c.extend(c1);
            (Type::BOOL, c)
        }
        Expr::GEQ(x, y)
        | Expr::LEQ(x, y)
        | Expr::GT(x, y)
        | Expr::LT(x, y)
        | Expr::EQ(x, y)
        | Expr::NEQ(x, y) => {
            let (t1, c1) = get_type_and_constr(*x, env.clone());
            let (t2, c2) = get_type_and_constr(*y, env.clone());
            let mut env2 = env.clone();
            env2.push((t1, Type::INT));
            env2.push((t2, Type::INT));
            env2.extend(c1);
            env2.extend(c2);
            (Type::BOOL, env2)
        }
        Expr::LET(x, e1, e2) => {
            let (t1, c1) = get_type_and_constr(*e1, env.clone());
            let mut env2 = env.clone();
            match *x {
                Expr::ID(s) => env2.push((Type::ID(s), t1)),
                _ => {
                    println!("let x = e1 in e2: x is not an identifier");
                    return (Type::ERROR, Vec::new());
                }
            };
            let (t2, c2) = get_type_and_constr(*e2, env2.clone());
            let mut c = c1.clone();
            c.extend(c2);
            (t2, c)
        }
        Expr::IF(e1, e2, e3) => {
            let (t1, c1) = get_type_and_constr(*e1, env.clone());
            let (t2, c2) = get_type_and_constr(*e2, env.clone());
            let (t3, c3) = get_type_and_constr(*e3, env.clone());
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
                    println!("fun x -> e: x is not an identifier");
                    return (Type::ERROR, Vec::new());
                }
            };

            let (t, c) = get_type_and_constr(*e, env2.clone());

            (Type::FUN(Box::new(alpha), Box::new(t)), c)
        }
        Expr::APP(e1, e2) => {
            let (t1, c1) = get_type_and_constr(*e1, env.clone());
            let (t2, c2) = get_type_and_constr(*e2, env.clone());
            let alpha = new_type_id();
            let mut c = env.clone();
            c.push((
                t1.clone(),
                Type::FUN(Box::new(t2.clone()), Box::new(alpha.clone())),
            ));
            c.extend(c1);
            c.extend(c2);
            (alpha, c)
        }
        Expr::SEMI(e1, e2) => {
            let (t1, c1) = get_type_and_constr(*e1, env.clone());
            let (t2, c2) = get_type_and_constr(*e2, env.clone());
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
                    println!("let rec f = e1 in e2: f is not an identifier");
                    return (Type::ERROR, Vec::new());
                }
            };
            let (t1, c1) = get_type_and_constr(*e1, env2.clone());
            let (t2, c2) = get_type_and_constr(*e2, env2.clone());
            let mut c = vec![(alpha.clone(), t1.clone())];
            c.extend(c1);
            c.extend(c2);
            (t2, c)
        }
    };
    // println!("in: {:?}", (ast_, env_));
    // println!("out: {:?}", r);

    let (t, c) = r;
    (t, c)
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

fn unify(t: Type, mut constr: Vec<(Type, Type)>) -> (Type, Vec<(Type, Type)>) {
    if constr == Vec::new() {
        return (t, Vec::new());
    } else {
        let c = constr.remove(0);
        match c {
            (t1, t2) if t1 == t2 => unify(t, constr),
            (Type::FUN(s1, t1), Type::FUN(s2, t2)) => {
                let mut c2 = vec![(*s1, *s2), (*t1, *t2)];
                c2.extend(constr);
                unify(t, c2)
            }
            (Type::TYPEID(x), t1) | (t1, Type::TYPEID(x)) if !matches!(t1, Type::ID(_)) => {
                let u = subst(constr, Type::TYPEID(x), t1.clone());
                let (t2, c2) = unify(t.clone(), u.clone());
                (
                    subst_once(t2, Type::TYPEID(x), t1.clone()),
                    subst(c2, Type::TYPEID(x), t1.clone()),
                )
            }
            _ => {
                let mut u = vec![c];
                let (t2, c2) = unify(t.clone(), constr.clone());
                u.extend(c2);
                (t2, u)
            }
        }
    }
}

pub fn get_type_and_unified_constr(ast: Expr) -> (Type, HashMap<String, Type>) {
    let constr_init = vec![
        (
            Type::ID("print_string".to_string()),
            Type::FUN(Box::new(Type::STRING), Box::new(Type::UNIT)),
        ),
        (
            Type::ID("print_int".to_string()),
            Type::FUN(Box::new(Type::INT), Box::new(Type::UNIT)),
        ),
        (
            Type::ID("print_bool".to_string()),
            Type::FUN(Box::new(Type::BOOL), Box::new(Type::UNIT)),
        ),
    ];
    let (ast_type, constr) = get_type_and_constr(ast, constr_init);
    let (ast_type_unified, constr_unified) = unify(ast_type.clone(), constr.clone());
    // println!("(ast_type, constr) = {:?}", (ast_type.clone(), constr.clone()));
    // println!("(ast_type_unified, constr_unified) = {:?}", (ast_type_unified.clone(), constr_unified.clone()));

    let mut u: Vec<(String, Type)> = Vec::new();

    for (x, y) in constr_unified {
        if let Type::ID(s) = x {
            u.push((s, y));
        }
    }

    (ast_type_unified, u.into_iter().collect())
}

// tのt1をt2に
fn subst_once(t: Type, t1: Type, t2: Type) -> Type {
    match t {
        Type::INT => Type::INT,
        Type::BOOL => Type::BOOL,
        Type::STRING => Type::STRING,
        Type::UNIT => Type::UNIT,
        Type::FUN(x, y) => Type::FUN(
            Box::new(subst_once(*x, t1.clone(), t2.clone())),
            Box::new(subst_once(*y, t1.clone(), t2.clone())),
        ),
        Type::ID(s) => Type::ID(s),
        Type::TYPEID(_) => {
            if t == t1 {
                t2
            } else {
                t
            }
        }
        Type::ERROR => Type::ERROR,
    }
}

// cのt1をt2に
fn subst(c: Vec<(Type, Type)>, t1: Type, t2: Type) -> Vec<(Type, Type)> {
    let mut r = c.clone();
    for i in 0..r.len() {
        let (x, y) = &r[i];
        r[i] = (
            subst_once(x.clone(), t1.clone(), t2.clone()),
            subst_once(y.clone(), t1.clone(), t2.clone()),
        );
    }
    r
}
