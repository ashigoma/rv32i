use crate::enums::Expr;
use crate::enums::IRExpr;

type IRCode = Vec<(String, Vec<String>, Vec<IRExpr>)>;

pub fn ast_to_ir(ast: Expr) -> IRCode {
    let mut r = IRCode::new();
    let (mut main_code, mut fn_code) = ast_block_to_ir(ast, "_res".to_string());
    main_code.push(IRExpr::RETURN("_res".to_string()));
    r.push(("_main".to_string(), vec!["_dummy".to_string()], main_code));
    r.extend(fn_code);
    r
}

fn ast_block_to_ir(ast: Expr, var: String) -> (Vec<IRExpr>, IRCode) {
    (Vec::<IRExpr>::new(), IRCode::new())
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
