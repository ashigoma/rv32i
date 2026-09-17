mod assembly;
mod enums;
mod grammer;
mod ir;
mod lexer;
mod parser;
mod typing;
mod vars;

use clap::Parser;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(author, version, about = "OCaml subset to RV32I compiler", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "")]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn exit(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    process::exit(1);
}

fn main() {
    let cli = Cli::parse();

    let code = fs::read_to_string(&cli.input)
        .unwrap_or_else(|e| exit(&format!("failed to open code ({}): {}", cli.input, e)));

    let tokens =
        lexer::tokenize(&code).unwrap_or_else(|e| exit(&format!("failed to tokenize: {}", e)));

    let ast = parser::parse(tokens).unwrap_or_else(|e| exit(&format!("failed to parse: {}", e)));

    let (_ast_type, _constr) = typing::get_type_and_unified_constr(ast.clone());

    let ir_code = ir::ast_to_ir(ast);

    let var_tree = vars::ir_func_code_to_vartree(ir_code.clone(), "_main".to_string())
        .unwrap_or_else(|e| exit(&format!("failed to construct vartree: {}", e)));

    let var_map = vars::vartree_to_varmap(var_tree);
    let asm_code = assembly::ir_code_to_asm(ir_code, &var_map);

    if let Some(out_path) = cli.output {
        fs::write(&out_path, &asm_code)
            .unwrap_or_else(|e| exit(&format!("failed to write assembly to {}: {}", out_path, e)));
    } else {
        println!("{}", asm_code);
    }
}
