mod assembly;
mod enums;
mod grammer;
mod ir;
mod lexer;
mod parser;
mod typing;
mod vars;

use std::fs;
use std::process;

fn main() {
    let code = match fs::read_to_string("tests/add.ml") {
        Ok(r) => r,
        Err(_) => {
            eprintln!("failed to open code");
            process::exit(1)
        }
    };

    println!("{}", code);

    let tokens = match lexer::tokenize(&code) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to tokenize: {}", e);
            process::exit(1)
        }
    };

    println!("{:?}", tokens);

    let ast = match parser::parse(tokens) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to parse: {}", e);
            process::exit(1)
        }
    };

    println!("{:?}", ast);

    let (ast_type, constr) = typing::get_type_and_unified_constr(ast.clone());
    println!("{:?}", ast_type);
    println!("{:?}", constr);

    let ir_code = ir::ast_to_ir(ast);
    ir::print_ir(ir_code.clone());

    let var_tree = match vars::ir_func_code_to_vartree(ir_code, "_main".to_string()) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("failed to construct vartree: {}", e);
            process::exit(1)
        }
    };
    println!("{:?}", var_tree);

    let var_map = vars::vartree_to_varmap(var_tree);
    println!("{:?}", var_map);
}
