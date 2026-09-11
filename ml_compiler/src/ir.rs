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
        Expr::ADD(a, b) => {
            let xa = new_tmp_var();
            let xb = new_tmp_var();
            let (ca, fa) = ast_block_to_ir(*a, xa.clone());
            let (cb, fb) = ast_block_to_ir(*b, xb.clone());
            code.extend(ca);
            code.extend(cb);
            func_code.extend(fa);
            func_code.extend(fb);
            code.push(IRExpr::ADD(var, xa.clone(), xb.clone()));
        }

        _ => {

        }
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


fn print_ir_expr(ir_expr: IRExpr) {
    match ir_expr {
        IRExpr::LABEL(s) => println!("{}:", s),
        IRExpr::JUMP(s) => println!("\tj {}", s),
        IRExpr::BRANCH(x, s) => println!("\tb {}, {}", x, s),
        IRExpr::LOADINT(x, n) => println!("\t{} = {}", x, n),
        IRExpr::LOADBOOL(x, b) => println!("\t{} = {}", x, if b { "true" } else { "false" }),
        IRExpr::LOADSTR(x, s) => println!("\t{} = {}", x, s),
        IRExpr::LOADFUNC(f, s) => println!("\t{} = &{}", f, s),
        IRExpr::ADD(x, a, b) => println!("\t{} = {} + {}", x, a, b),
        IRExpr::RETURN(x) => println!("\tret {}", x),
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
