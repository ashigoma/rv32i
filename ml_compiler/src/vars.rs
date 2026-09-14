use crate::enums::IRExpr;
use crate::enums::VarTree;
use crate::ir::FuncCode;
use crate::ir::IRCode;

pub fn ir_func_code_to_vartree(code: IRCode, label: String) -> Result<VarTree, String> {
    let Some((_, v, func_code)) = ir_lookup_fn(code.clone(), label.clone()) else {
        return Err(format!("undefined function {label}"));
    };

    let mut children = Vec::<VarTree>::new();

    let mut var_defined = Vec::new();
    let mut var_free = Vec::new();

    var_defined.push(v.clone());
    children.push(VarTree::VAR(v));

    for c in func_code {
        let mut var_defined_new = Vec::new();
        let mut var_free_new = Vec::new();
        match c {
            IRExpr::ADD(x, y, z) | IRExpr::SUB(x, y, z) | IRExpr::GEQ(x, y, z) | IRExpr::LEQ(x, y, z) | IRExpr::GT(x, y, z) | IRExpr::LT(x, y, z) | IRExpr::EQ(x, y, z) | IRExpr::NEQ(x, y, z) | IRExpr::AND(x, y, z) | IRExpr::OR(x, y, z) | IRExpr::APP(x, y, z) => {
                var_defined_new.push(x);
                var_free_new.push(y);
                var_free_new.push(z);
            }
            IRExpr::NOT(x, y) | IRExpr::LOAD(x, y) => {
                var_defined_new.push(x);
                var_free_new.push(y);
            }
            IRExpr::LOADINT(x, _) | IRExpr::LOADBOOL(x, _) | IRExpr::LOADSTR(x, _) | IRExpr::LOADUNIT(x) => {
                var_defined_new.push(x);
            }
            IRExpr::BRANCH(x, _) | IRExpr::RETURN(x) => {
                var_free_new.push(x);
            }
            IRExpr::LOADFUNC(label, f) => {
                var_defined_new.push(label);
                let subfunc_res = ir_func_code_to_vartree(code.clone(), f);
                if let Ok(tree) = subfunc_res {children.push(tree)} else {return subfunc_res};
            }
            _ => {}
        }
        var_free_new.retain(|x| !var_defined.contains(x));
        var_free_new.retain(|x| !var_free.contains(x));
        var_free_new.sort();
        var_free_new.dedup();
        var_free.extend(var_free_new);
        var_defined_new.retain(|x| !var_defined.contains(x));
        var_defined_new.sort();
        var_defined_new.dedup();
        var_defined.extend(var_defined_new.clone());
        for v in var_defined_new {
            children.push(VarTree::VAR(v));
        }
    }

    return Ok(VarTree::FUNC(label, Box::new(children), var_free));
}

fn ir_lookup_fn(code: IRCode, label: String) -> Option<FuncCode> {
    for (f, v, c) in code {
        if f == label {
            return Some((f, v, c));
        }
    }
    return None;
}
