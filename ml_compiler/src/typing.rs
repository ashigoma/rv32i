use crate::enums::Expr;
use crate::enums::Type;
use std::cell::Cell;
use std::collections::HashMap;

fn get_type_and_constr(ast: Expr, env: Vec<(Type, Type)>) -> (Type, Vec<(Type, Type)>) {
    let (ast_, env_) = (ast.clone(), env.clone());
    println!("in1: {:?}", (ast_.clone(), env_.clone()));
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
    println!("in: {:?}", (ast_, env_));
    println!("out: {:?}", r);

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

// pub fn get_type_and_unified_constr(ast: Expr) -> (Type, HashMap<String, Type>) {
//     let ast_main = Expr::LET(Box::new(Expr::ID("_main".to_string())), Box::new(ast), Box::new(Expr::ID("_main".to_string())));

//     let (_, constr) = get_type_and_constr(ast_main, Vec::new());

//     println!("constr: {:?}", constr);

//     let mut u: Vec<(String, Type)> = Vec::new();
//     let mut ast_type = Type::ERROR;

//     for (x, y) in unify(constr) {
//         if let Type::ID(s) = x {
//             if s == "_main" {
//                 ast_type = y;
//             } else {
//                 u.push((s, y));
//             }
//         }
//     }

//     (ast_type, u.into_iter().collect())
// }

pub fn get_type_and_unified_constr(ast: Expr) -> (Type, HashMap<String, Type>) {
    let (ast_type, constr) = get_type_and_constr(ast, Vec::new());
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
    // println!("[subst] {:?} -> {:?}", t1, t2);
    let mut r = c.clone();
    for i in 0..r.len() {
        let (x, y) = &r[i];
        // println!("before subst: {:?}", (x, y));
        r[i] = (
            subst_once(x.clone(), t1.clone(), t2.clone()),
            subst_once(y.clone(), t1.clone(), t2.clone()),
        );
        // println!("after subst: {:?}", r[i]);
    }
    r
}
