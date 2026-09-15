use crate::vars::VarMap;

// rd = <var>
pub fn asm_load(rd: i32, var: &str, scope: &str, varmap: &VarMap) -> String {
    let mut res = format!("\t// x{} = <{}>\n", rd, var);

    let (map_local, map_free) = &varmap[scope];
    if let Some(var_index_local) = map_local.get(var) {
        res += &format!("\t< x{} = [sp + {}] >\n", rd, (var_index_local + 1) * 4);
    } else {
        let (n_derefence, scope_new) = &map_free[var];
        let var_index_local = &map_local[var];
        res += &format!("\t< x{} = [sp] >\n", rd);
        for i in 0..*n_derefence {
            res += &format!("\t< x{} = [x{} + 4]>\n", rd, rd);
            res += &format!("\t< x{} = [x{}]>\n", rd, rd);
        }
        res += &format!("\t< x{} = [x{} + {}] >\n", rd, rd, (var_index_local + 1) * 4);
    }
    res

}