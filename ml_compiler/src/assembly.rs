use crate::vars::VarMap;

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
        res += &format!("\t< x{} = [sp + {}] >\n", rd, (var_index_local + 1) * 4);
    } else {
        let (n_derefence, scope_new) = &map_free[var];
        // closure pointer
        if toplevel {
            res += &format!("\t< x{} = [sp] >\n", rd);
        } else {
            res += &format!("\t< x{} = [x{}]>\n", rd, rd);
        }
        // 1つ前のstack frame
        res += &format!("\t< x{} = [x{} + 4]>\n", rd, rd);
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
        res += &format!("\t< [sp + {}] = x{} >\n", (var_index_local + 1) * 4, rd);
    } else {
        panic!()
    }
    res
}
