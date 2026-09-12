use crate::enums::Expr;
use crate::enums::IRExpr;
use std::cell::Cell;

type IRCode = Vec<(String, Vec<String>, Vec<IRExpr>)>;

pub fn ast_to_ir(ast: Expr) -> IRCode {
    let mut r = IRCode::new();
    let (mut main_code, fn_code) = ast_block_to_ir(ast, "_res".to_string());
    main_code.push(IRExpr::RETURN("_res".to_string()));
    r.push(("_main".to_string(), vec!["_dummy".to_string()], main_code));
    r.extend(fn_code);
    r
}

fn ast_block_to_ir(ast: Expr, var: String) -> (Vec<IRExpr>, IRCode) {
    let mut code = Vec::<IRExpr>::new();
    let mut func_code = IRCode::new();

    match ast {
        Expr::INT(n) => {
            code.push(IRExpr::LOADINT(var, n));
        }
        Expr::ID(s) => {
            code.push(IRExpr::LOAD(var, s));
        }
        Expr::ADD(e1, e2) => {
            let x1 = new_tmp_var();
            let x2 = new_tmp_var();
            let (c1, f1) = ast_block_to_ir(*e1, x1.clone());
            let (c2, f2) = ast_block_to_ir(*e2, x2.clone());
            code.extend(c1);
            code.extend(c2);
            func_code.extend(f1);
            func_code.extend(f2);
            code.push(IRExpr::ADD(var, x1.clone(), x2.clone()));
        }
        Expr::LET(x, e1, e2) => {
            let Expr::ID(s) = *x else {todo!()};
            let (c1, f1) = ast_block_to_ir(*e1, s.to_string());
            let (c2, f2) = ast_block_to_ir(*e2, var);
            code.extend(c1);
            code.extend(c2);
            func_code.extend(f1);
            func_code.extend(f2);
        }
        Expr::FUN(x, e) => {
            let Expr::ID(s) = *x else {todo!()};
            let f = new_tmp_fn();

            let x1 = new_tmp_var();
            let (c1, f1) = ast_block_to_ir(*e, x1.clone());
            let mut f_code = Vec::<IRExpr>::new();
            f_code.extend(c1);
            f_code.push(IRExpr::RETURN(x1));

            func_code.push((f.clone(), vec![s], f_code));

            code.push(IRExpr::LOADFUNC(var, f));
        }
        Expr::APP(f, a) => {
            let Expr::ID(s) = *f else {todo!()};
            let x1 = new_tmp_var();
            let (c1, f1) = ast_block_to_ir(*a, x1.clone());
            code.extend(c1);
            func_code.extend(f1);
            code.push(IRExpr::APP(var, s, x1));
        }

        _ => {}
    }

    (code, func_code)
}

fn new_tmp_var() -> String {
    thread_local! {
        static TMP_VAR_COUNTER: Cell<i32> = Cell::new(0);
    }

    let id = TMP_VAR_COUNTER.with(|c| c.replace(c.get() + 1) + 1);
    format!("_x{id}")
}

fn new_tmp_fn() -> String {
    thread_local! {
        static FN_ID_COUNTER: Cell<i32> = Cell::new(0);
    }

    let id = FN_ID_COUNTER.with(|c| c.replace(c.get() + 1) + 1);
    format!("_f{id}")
}

fn print_ir_expr(ir_expr: IRExpr) {
    match ir_expr {
        IRExpr::LABEL(s) => println!("{}:", s),
        IRExpr::JUMP(s) => println!("\tj {}", s),
        IRExpr::BRANCH(x, s) => println!("\tb {}, {}", x, s),
        IRExpr::LOAD(x, y) => println!("\t{} = {}", x, y),
        IRExpr::LOADINT(x, n) => println!("\t{} = {}", x, n),
        IRExpr::LOADBOOL(x, b) => println!("\t{} = {}", x, if b { "true" } else { "false" }),
        IRExpr::LOADSTR(x, s) => println!("\t{} = {}", x, s),
        IRExpr::LOADFUNC(f, s) => println!("\t{} = &{}", f, s),
        IRExpr::ADD(x, a, b) => println!("\t{} = {} + {}", x, a, b),
        IRExpr::RETURN(x) => println!("\tret {}", x),
        IRExpr::APP(x, f, a) => println!("\t{} = {} {}", x, f, a),
    }
}

pub fn print_ir(ir: IRCode) {
    for (fn_name, vars, code) in ir {
        print!("{} ", fn_name);
        for var in vars {
            print!("{} ", var);
        }
        println!(":");
        for ln in code {
            print_ir_expr(ln);
        }
    }
}
