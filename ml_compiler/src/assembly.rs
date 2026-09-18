use crate::enums::IRExpr;
use crate::ir::IRCode;
use crate::ir::ir_to_string;
use crate::vars::VarMap;

const REG_RA: i32 = 1; // return address
const REG_SP: i32 = 2; // stack pointer
const REG_HP: i32 = 3; // heap pointer (なお規約違反)
const REG_T0: i32 = 5; // tmp 0
const REG_T1: i32 = 6; // tmp 1
const REG_T2: i32 = 7; // tmp 2
const REG_T3: i32 = 28; // tmp 3
const REG_A0: i32 = 10; // arg 1 (return val)

// rd = <var>
// 関数内で、束縛変数 or lambda closure内のキャプチャされた自由変数の中身をとってきてレジスタにいれる
fn asm_load_from_var(rd: i32, var: &str, scope: &str, varmap: &VarMap) -> String {
    let mut res = "".to_string();

    let (map_local, map_free) = &varmap[scope];
    if let Some(var_index_local) = map_local.get(var) {
        // いまいるstack frame内にある
        // rd = [sp + (idx + 3) * 4]
        res += &format!("\tlw x{}, {}(x{})\n", rd, (var_index_local + 3) * 4, REG_SP);
    } else {
        // いまいるstack frameから飛べるlambda closureにおいてある
        let (_, closure_idx) = &map_free[var];
        // rd = [sp + 8]                        // *closure
        // rd = [rd + (closure idx + 4) * 4]
        res += &format!("\tlw x{}, {}(x{})\n", rd, 8, REG_SP);
        res += &format!("\tlw x{}, {}(x{})\n", rd, (closure_idx + 4) * 4, rd);
    }
    res
}

// キャプチャのために束縛変数をとってきてレジスタにいれる
// scopeにおいて実体化が行われる
// varがfuncの自由変数
fn asm_load_for_capture(rd: i32, var: &str, scope: &str, func: &str, varmap: &VarMap) -> String {
    let mut res = "".to_string();

    println!(
        "asm_load_for_capture: var={} scope={} func={}",
        var, scope, func
    );
    let (_, map_free_inner) = &varmap[func];
    let (map_local, map_free) = &varmap[scope];
    let (from_closure, _) = &map_free_inner[var];

    if *from_closure {
        let (_, closure_idx) = map_free[var];
        // rd = [sp + 8]    // *closure
        res += &format!("\tlw x{}, {}(x{})\n", rd, 8, REG_SP);
        // rd = [rd + (idx + 4) * 4]    // closureからとってくる
        res += &format!("\tlw x{}, {}(x{})\n", rd, (closure_idx + 4) * 4, rd);
    } else {
        let local_idx = map_local[var];
        // rd = [sp + (idx + 3) * 4]    // stack frameからとってくる
        res += &format!("\tlw x{}, {}(x{})\n", rd, (local_idx + 3) * 4, REG_SP);
    }
    res
}

// <var> = rd
fn asm_store_to_var(var: &str, rd: i32, scope: &str, varmap: &VarMap) -> String {
    let mut res = "".to_string();
    let (map_local, _) = &varmap[scope];
    if let Some(var_index_local) = map_local.get(var) {
        // [sp + (idx + 3) * 4] = rd
        res += &format!("\tsw x{}, {}(x{})\n", rd, (var_index_local + 3) * 4, REG_SP);
    } else {
        panic!()
    }
    res.to_string()
}

pub fn ir_code_to_asm(code: IRCode, varmap: &VarMap) -> String {
    let mut res = "".to_string();
    res += ".global _main\n";
    for (f, _, fn_code) in code {
        res += &format!("{}:\n", f);
        if f == "_main" {
            let (map_local, _) = &varmap[&f];
            let stack_frame_size = ((map_local.len() + 3) * 4) as i32;
            // stack frameの確保
            // sp = sp - stack_frame_size
            res += &format!("\taddi x{}, x{}, {}\n", REG_SP, REG_SP, -stack_frame_size);

            // _mainのclosure作成
            // [hp] =

            // 組み込み関数のclosure作成
            // [hp + 4] = sp
            // [hp + 8] = 16
            // [hp + 12] = print_string (local idx: 0)
            // [sp + 12] = hp
            // hp += 16
            // [hp + 4] = sp
            // [hp + 8] = 16
            // [hp + 12] = print_int (local idx: 1)
            // [sp + 16] = hp
            // hp += 16
            res += &format!("\tsw x{}, {}(x{})\n", REG_SP, 4, REG_HP);
            res += &format!("\tli x{}, {}\n", REG_T0, 16);
            res += &format!("\tsw x{}, {}(x{})\n", REG_T0, 8, REG_HP);
            res += &format!("\tla x{}, {}\n", REG_T0, "print_string");
            res += &format!("\tsw x{}, {}(x{})\n", REG_T0, 12, REG_HP);
            res += &format!("\tsw x{}, {}(x{})\n", REG_HP, 12, REG_SP);
            res += &format!("\taddi x{}, x{}, {}\n", REG_HP, REG_HP, 16);
            res += &format!("\tsw x{}, {}(x{})\n", REG_SP, 4, REG_HP);
            res += &format!("\tli x{}, {}\n", REG_T0, 16);
            res += &format!("\tsw x{}, {}(x{})\n", REG_T0, 8, REG_HP);
            res += &format!("\tla x{}, {}\n", REG_T0, "print_int");
            res += &format!("\tsw x{}, {}(x{})\n", REG_T0, 12, REG_HP);
            res += &format!("\tsw x{}, {}(x{})\n", REG_HP, 16, REG_SP);
            res += &format!("\taddi x{}, x{}, {}\n", REG_HP, REG_HP, 16);
        }
        for c in fn_code {
            res += "# ";
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
                IRExpr::LOADSTR(x, s) => {
                    // 文字列をheapに確保
                    let mut i = 0;
                    for c in s.chars().take_while(|&c| c != '\0') {
                        let code = c as u8;
                        res += &format!("\tli x{}, {}\n", REG_T0, code);
                        res += &format!("\tsb x{}, {}(x{})\n", REG_T0, i, REG_HP);
                        i = i + 1;
                    }
                    res += &format!("\tsb x{}, {}(x{})\n", 0, i, REG_HP);
                    res += &asm_store_to_var(&x, REG_HP, &f, varmap);
                    res += &format!("\taddi x{}, x{}, {}\n", REG_HP, REG_HP, s.len() + 1);
                }
                IRExpr::LOADFUNC(x, flabel) => {
                    let (map_local, map_free) = &varmap[&flabel];
                    let stack_frame_size = (map_local.len() + 3) * 4;

                    // lambda closureをheapに確保
                    // t0 = [sp + 8]            // *親closure
                    // [hp] = t0                // closure[0] = *親closure
                    // [hp + 4] = sp            // closure[1] = caller stack frame
                    // t0 = stack frame size
                    // [hp + 8] = t0            // closure[2] = stack frame size
                    // t0 = flabel
                    // [hp + 12] = t0           // closure[3] = 開始アドレス

                    // t0 = <v_i>
                    // [hp + 16 + i*4] = t0     // 各自由変数をcapture

                    // <x> = hp                 // x = *closure
                    // hp = hp + closure size   // heap確保
                    res += &format!("\tlw x{}, {}(x{})\n", REG_T0, 8, REG_SP);
                    res += &format!("\tsw x{}, {}(x{})\n", REG_T0, 0, REG_HP);
                    res += &format!("\tsw x{}, {}(x{})\n", REG_SP, 4, REG_HP);
                    res += &format!("\tli x{}, {}\n", REG_T0, stack_frame_size);
                    res += &format!("\tsw x{}, {}(x{})\n", REG_T0, 8, REG_HP);
                    res += &format!("\tla x{}, {}\n", REG_T0, flabel);
                    res += &format!("\tsw x{}, {}(x{})\n", REG_T0, 12, REG_HP);

                    for (fvar, entry) in map_free {
                        let (_, closure_idx) = entry;
                        res += &asm_load_for_capture(REG_T0, fvar, &f, &flabel, varmap);
                        res +=
                            &format!("\tsw x{}, {}(x{})\n", REG_T0, 4 * (closure_idx + 4), REG_HP);
                    }

                    res += &asm_store_to_var(&x, REG_HP, &f, varmap);
                    res += &format!(
                        "\taddi x{}, x{}, {}\n",
                        REG_HP,
                        REG_HP,
                        4 * (map_free.len() + 4)
                    );
                }
                IRExpr::RETURN(x) => {
                    res += &asm_load_from_var(REG_A0, &x, &f, varmap);
                    res += "\tret\n";
                }
                IRExpr::APP(x, func, a) => {
                    // stackをつくる
                    // t0 = <func>      // *closure
                    // t1 = [t0 + 8]    // stack frame size
                    // t2 = sp          // old sp
                    // t3 = <a>         // arg
                    // sp = sp - t1     // stack frame確保
                    // [sp] = t2        // stack frame[0] = old sp
                    // [sp + 4] = ra    // stack frame[1] = old return address
                    // [sp + 8] = t0    // stack frame[2] = *closure
                    // [sp + 12] = t3   // stack frame[3] = arg
                    res += &asm_load_from_var(REG_T0, &func, &f, varmap);
                    res += &format!("\tlw x{}, {}(x{})\n", REG_T1, 8, REG_T0);
                    res += &format!("\tmv x{}, x{}\n", REG_T2, REG_SP);
                    res += &asm_load_from_var(REG_T3, &a, &f, varmap);
                    res += &format!("\tsub x{}, x{}, x{}\n", REG_SP, REG_SP, REG_T1);
                    res += &format!("\tsw x{}, {}(x{})\n", REG_T2, 0, REG_SP);
                    res += &format!("\tsw x{}, {}(x{})\n", REG_RA, 4, REG_SP);
                    res += &format!("\tsw x{}, {}(x{})\n", REG_T0, 8, REG_SP);
                    res += &format!("\tsw x{}, {}(x{})\n", REG_T3, 12, REG_SP);

                    // t0 = [t0 + 12]   // entry point
                    // call t0
                    res += &format!("\tlw x{}, {}(x{})\n", REG_T0, 12, REG_T0);
                    res += &format!("\tjalr x{}\n", REG_T0);

                    // ra = [sp + 4]    // return addressをpop
                    // sp = [sp]        // spをpop
                    res += &format!("\tlw x{}, {}(x{})\n", REG_RA, 4, REG_SP);
                    res += &format!("\tlw x{}, {}(x{})\n", REG_SP, 0, REG_SP);

                    // <x> = a0         // x = func(a)
                    res += &asm_store_to_var(&x, REG_A0, &f, varmap);
                }
                _ => {}
            }
        }
    }
    res.to_string()
}
