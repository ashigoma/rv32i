use crate::enums::IRExpr;
use crate::enums::VarTree;
use crate::ir::FuncCode;
use crate::ir::IRCode;
use std::collections::HashMap;

type LocalVarMapping = Vec<(String, i32)>;
type FreeVarMapping = HashMap<String, (i32, String)>;
type VarMapEntry = (LocalVarMapping, FreeVarMapping);
type VarMap = HashMap<String, VarMapEntry>;

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
            IRExpr::ADD(x, y, z)
            | IRExpr::SUB(x, y, z)
            | IRExpr::GEQ(x, y, z)
            | IRExpr::LEQ(x, y, z)
            | IRExpr::GT(x, y, z)
            | IRExpr::LT(x, y, z)
            | IRExpr::EQ(x, y, z)
            | IRExpr::NEQ(x, y, z)
            | IRExpr::AND(x, y, z)
            | IRExpr::OR(x, y, z)
            | IRExpr::APP(x, y, z) => {
                var_defined_new.push(x);
                var_free_new.push(y);
                var_free_new.push(z);
            }
            IRExpr::NOT(x, y) | IRExpr::LOAD(x, y) => {
                var_defined_new.push(x);
                var_free_new.push(y);
            }
            IRExpr::LOADINT(x, _)
            | IRExpr::LOADBOOL(x, _)
            | IRExpr::LOADSTR(x, _)
            | IRExpr::LOADUNIT(x) => {
                var_defined_new.push(x);
            }
            IRExpr::BRANCH(x, _) | IRExpr::RETURN(x) => {
                var_free_new.push(x);
            }
            IRExpr::LOADFUNC(label, f) => {
                var_defined_new.push(label);
                let subfunc_res = ir_func_code_to_vartree(code.clone(), f);
                if let Ok(tree) = subfunc_res {
                    children.push(tree)
                } else {
                    return subfunc_res;
                };
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

fn write_to_map(map: &mut VarMap, func: &VarTree, scope: Vec<(String, i32, String)>) {
    // println!("map:{:?}\nfunc:{:?}\nscope:{:?}", map, func, scope);
    let mut map_local = LocalVarMapping::new();
    let mut map_free = FreeVarMapping::new();
    let mut scope_ = scope.clone();

    if let VarTree::FUNC(func_name, children, freevals) = func {
        // 束縛変数
        let mut i = 0;
        for c in children.iter() {
            if let VarTree::VAR(s) = c {
                map_local.push((s.to_string(), i));
                scope_.push((s.to_string(), 0, func_name.to_string()));
                i += 1;
            } else if let VarTree::FUNC(_, _, _) = c {
                let mut scope_2 = scope_.clone();
                for j in 0..scope_2.len() {
                    let (v, n, f) = &scope_2[j];
                    scope_2[j] = (v.to_string(), n + 1, f.to_string());
                }
                write_to_map(map, &c, scope_2);
            }
        }

        // 自由変数
        for s in freevals {
            let mut found = false;
            for (v, n, f) in scope.iter().rev() {
                if v == s {
                    map_free.insert(v.to_string(), (*n, f.to_string()));
                    found = true;
                    break;
                }
            }
            if !found {
                println!("write_to_map: undefined variable: {}", s);
            }
        }

        let entry: VarMapEntry = (map_local, map_free);
        map.insert(func_name.to_string(), entry);
    } else {
        println!("write_to_map: not a function: {:?}", func);
    }
}

pub fn vartree_to_varmap(tree: VarTree) -> VarMap {
    let mut map = VarMap::new();
    write_to_map(&mut map, &tree, Vec::new());
    map
}
