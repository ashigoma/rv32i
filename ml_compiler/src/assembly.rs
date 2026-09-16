use crate::enums::IRExpr;
use crate::ir::IRCode;
use crate::ir::ir_to_string;
use crate::vars::VarMap;

const REG_RA: i32 = 1; // return address
const REG_SP: i32 = 2; // stack pointer
const REG_T0: i32 = 5; // tmp 0
const REG_T1: i32 = 6; // tmp 1
const REG_T2: i32 = 7; // tmp 2
const REG_A0: i32 = 10; // arg 1 (return val)

// rd = <var>
// 関数内で、束縛変数 or lambda closure内のキャプチャされた自由変数の中身をとってきてレジスタにいれる
fn asm_load_from_var(rd: i32, var: &str, scope: &str, varmap: &VarMap) -> String {
    let mut res = "".to_string();

    let (map_local, map_free) = &varmap[scope];
    if let Some(var_index_local) = map_local.get(var) {
        // いまいるstack frame内にある
        res += &format!("\tlw x{}, {}(x{})\n", rd, (var_index_local + 2) * 4, REG_SP);
    } else {
        // いまいるstack frameから飛べるlambda closureにおいてある
        let (n, scope_new, closure_idx) = &map_free[var];
        assert!(*n == 0);
        res += &format!("\tlw x{}, 4(x{})\n", rd, REG_SP);
        res += &format!("\tlw x{}, {}(x{})\n", rd, 4 * (closure_idx + 4), rd);
    }
    res
}

// 外側の束縛変数をとってきてレジスタにいれる
fn asm_load_from_var_closure(rd: i32, var: &str, scope: &str, varmap: &VarMap) -> String {
    let mut res = "".to_string();

    let (_, map_free) = &varmap[scope];
    let (n, scope_new, closure_idx) = &map_free[var];

    res += &format!("\tmov x{}, x{}\n", rd, REG_SP);

    for _ in 0..*n {
        res += &format!("\tlw x{}, 0(x{})\n", rd, rd);
    }

    res += &format!("\tlw x{}, {}(x{})\n", rd, 4 * (closure_idx + 2), rd);
    res
}

// <var> = rd
fn asm_store_to_var(var: &str, rd: i32, scope: &str, varmap: &VarMap) -> String {
    let mut res = "".to_string();
    let (map_local, map_free) = &varmap[scope];
    if let Some(var_index_local) = map_local.get(var) {
        res += &format!("\tsw x{}, {}(x{})\n", rd, (var_index_local + 2) * 4, REG_SP);
    } else {
        panic!()
    }
    res.to_string()
}

pub fn ir_code_to_asm(code: IRCode, varmap: &VarMap) -> String {
    let mut res = "".to_string();
    for (f, var, fn_code) in code {
        res += &format!("{}:\n", f);
        for c in fn_code {
            res += "// ";
            res += &ir_to_string(c.clone(), false);
            res += "\n";
            match c {
                IRExpr::ADD(x1, x2, x3) => {
                    res += &asm_load_from_var(REG_T1, &x2, &f, varmap);
                    res += &asm_load_from_var(REG_T2, &x3, &f, varmap);
                    res += &format!("\tadd x{}, x{}, x{}\n", REG_T0, REG_T1, REG_T2);
                    res += &asm_store_to_var(&x1, REG_T0, &f, varmap);
                }
                IRExpr::LABEL(s) => {
                    res += &format!("{}:\n", s);
                }
                IRExpr::JUMP(s) => {
                    res += &format!("\tj {}\n", s);
                }
                IRExpr::BRANCH(x, s) => {
                    res += &asm_load_from_var(REG_T0, &x, &f, varmap);
                    res += &format!("\tbne x{}, x0, {}\n", REG_T0, s);
                }
                IRExpr::LOAD(x1, x2) => {
                    res += &asm_load_from_var(REG_T0, &x2, &f, varmap);
                    res += &asm_store_to_var(&x1, REG_T0, &f, varmap);
                }
                IRExpr::LOADUNIT(x1) => {
                    res += &asm_store_to_var(&x1, 0, &f, varmap);
                }
                IRExpr::LOADINT(x, n) => {
                    res += &format!("\tli x{}, {}\n", REG_T0, n);
                    res += &asm_store_to_var(&x, REG_T0, &f, varmap);
                }
                IRExpr::LOADBOOL(x, b) => {
                    res += &format!("\tli x{}, {}\n", REG_T0, if b { 0 } else { 1 });
                    res += &asm_store_to_var(&x, REG_T0, &f, varmap);
                }
                IRExpr::LOADSTR(x, b) => {}
                IRExpr::LOADFUNC(x, flabel) => {
                    res += &format!("\tla x{}, {}\n", REG_T0, flabel);
                    res += &asm_store_to_var(&x, REG_T0, &f, varmap);
                }
                IRExpr::RETURN(x) => {
                    res += &asm_load_from_var(REG_A0, &x, &f, varmap);
                    res += "\tret\n";
                }
                IRExpr::APP(func, x1, x2) => {}
                _ => {}
            }
        }
    }
    res.to_string()
}
