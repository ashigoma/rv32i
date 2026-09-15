use crate::enums::IRExpr;
use crate::ir::IRCode;
use crate::vars::VarMap;

const REG_RA: i32 = 1; // return address
const REG_SP: i32 = 2; // stack pointer
const REG_T0: i32 = 5; // tmp 0
const REG_T1: i32 = 6; // tmp 1
const REG_T2: i32 = 7; // tmp 2
const REG_A0: i32 = 10; // arg 1 (return val)

// rd = <var>
fn asm_load_from_var(rd: i32, var: &str, scope: &str, varmap: &VarMap, toplevel: bool) -> String {
    let mut res = if toplevel {
        format!("\t// x{} = <{}>\n", rd, var)
    } else {
        "".to_string()
    };

    let (map_local, map_free) = &varmap[scope];
    if let Some(var_index_local) = map_local.get(var) {
        // いまいるstack frame内にある
        res += &format!("\tlw x{}, {}(x{})\n", rd, (var_index_local + 1) * 4, REG_SP);
    } else {
        let (n_derefence, scope_new) = &map_free[var];
        // closure pointer
        if toplevel {
            res += &format!("\tlw x{}, 0(x{})\n", rd, REG_SP);
        } else {
            res += &format!("\tlw x{}, 0(x{})\n", rd, rd);
        }
        // 1つ前のstack frame
        res += &format!("\tlw x{}, 4(x{})\n", rd, rd);
        // 1つ前のstack frameに戻る
        res += &asm_load_from_var(rd, var, scope_new, varmap, false);
    }
    res
}

// <var> = rd
fn asm_store_to_var(var: &str, rd: i32, scope: &str, varmap: &VarMap) -> String {
    let mut res = format!("\t// <{}> = x{}\n", var, rd);
    let (map_local, map_free) = &varmap[scope];
    if let Some(var_index_local) = map_local.get(var) {
        res += &format!("\tsw x{}, {}(x{})\n", rd, (var_index_local + 1) * 4, REG_SP);
    } else {
        panic!()
    }
    res
}

pub fn ir_code_to_asm(code: IRCode, varmap: &VarMap) -> String {
    let mut res = "".to_string();
    for (f, var, fn_code) in code {
        res += &format!("{}:\n", f);
        for c in fn_code {
            match c {
                IRExpr::LOADINT(x, n) => {
                    res += &format!("\tli x{}, {}\n", REG_T0, n);
                    res += &asm_store_to_var(&x, REG_T0, &f, varmap);
                }
                IRExpr::ADD(x1, x2, x3) => {
                    res += &asm_load_from_var(REG_T1, &x2, &f, varmap, true);
                    res += &asm_load_from_var(REG_T2, &x3, &f, varmap, true);
                    res += &format!("\tadd x{}, x{}, x{}\n", REG_T0, REG_T1, REG_T2);
                    res += &asm_store_to_var(&x1, REG_T0, &f, varmap);
                }
                IRExpr::LABEL(s) => {
                    res += &format!("{}:\n", s);
                }
                IRExpr::RETURN(x) => {
                    res += &asm_load_from_var(REG_A0, &x, &f, varmap, true);
                    res += "\tret\n";
                }
                _ => {}
            }
        }
    }
    res.to_string()
}
