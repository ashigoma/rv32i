use crate::enums::IRExpr;
use crate::enums::VarTree;
use crate::ir::FuncCode;
use crate::ir::IRCode;
use std::collections::HashMap;

type LocalVarMapping = HashMap<String, usize>;
type FreeVarMapping = HashMap<String, (bool, usize)>;    // 1つ前のscopeのclosureからとってくるならtrue 1つ前のscopeの束縛変数ならfalse あとclosure上でのindex
type VarMapEntry = (LocalVarMapping, FreeVarMapping);
pub type VarMap = HashMap<String, VarMapEntry>;

pub fn ir_func_code_to_vartree(code: IRCode, label: String) -> Result<VarTree, String> {
    let Some((_, v, func_code)) = ir_lookup_fn(code.clone(), label.clone()) else {
        return Err(format!("undefined function {label}"));
    };

    let mut children = Vec::<VarTree>::new();

    let mut var_defined = Vec::new();
    let mut var_free = Vec::new();

    if label == "_main" {
        var_defined.push("print_string".to_string());
        var_defined.push("print_int".to_string());
        children.push(VarTree::VAR("print_string".to_string()));
        children.push(VarTree::VAR("print_int".to_string()));
    } else {
        var_defined.push(v.clone());
        children.push(VarTree::VAR(v));
    }

    for c in func_code.clone() {
        let mut skip_push_children = false;
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
                children.push(VarTree::VAR(label.clone()));
                skip_push_children = true;
                var_defined_new.push(label.clone());
                let subfunc_res = ir_func_code_to_vartree(code.clone(), f);
                if let Ok(tree) = subfunc_res {
                    children.push(tree);
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
        if !skip_push_children {
            for v in var_defined_new {
                children.push(VarTree::VAR(v.to_string()));
            }
        }
    }

    // println!("{:?}: {:?} -> {:?}", label, &func_code, var_free);
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

fn write_to_map(map: &mut VarMap, func: &VarTree, outer_local: &mut LocalVarMapping, outer_free: &mut FreeVarMapping) -> Result<Vec<String>, String> {
    let mut map_local = LocalVarMapping::new();
    let mut map_free = FreeVarMapping::new();
    let mut new_outer_free = Vec::new();

    if let VarTree::FUNC(func_name, children, freevals) = func {
        // 明らかに自由変数なものたち
        for s in freevals {
            if let Some(_) = outer_local.get(s) {
                // 外側関数の束縛変数
                map_free.insert(s.to_string(), (false, map_free.len()));
            } else if let Some((_, _)) = outer_free.get(s) {
                // 他の関数によって、外側関数の自由変数にすでに登録されている
                map_free.insert(s.to_string(), (true, map_free.len()));
            } else {
                // 外側関数の自由変数かつ未登録なら仮登録
                outer_free.insert(s.to_string(), (true, outer_free.len())); // trueは仮 idxはここで確定
                map_free.insert(s.to_string(), (true, map_free.len()));
                new_outer_free.push(s.to_string());
            }
        }

        // 束縛変数
        for c in children.iter() {
            if let VarTree::VAR(s) = c {
                map_local.insert(s.to_string(), map_local.len());
            } else if let VarTree::FUNC(_, _, _) = c {
                // 内側関数の自由変数を解析
                let r = write_to_map(map, c, &mut map_local, &mut map_free);
                if let Ok(new_free) = r {
                    // その過程で仮登録された自身の各自由変数が、外側関数の自由変数なのか束縛変数なのかを登録
                    for fv in new_free {
                        let (_, idx) = map_free[&fv];
                        map_free.insert(fv.clone(), (!outer_local.contains_key(&fv), idx));
                    }
                } else if let Err(e) = r {
                    return Err(e);
                }
            }
        }

        let entry: VarMapEntry = (map_local, map_free);
        map.insert(func_name.to_string(), entry);
        Ok(new_outer_free)
    } else {
        Err(format!("not a function: {:?}", func))
    }
} 


// fn write_to_map(map: &mut VarMap, func: &VarTree, scope: Vec<(String, i32, String)>) {
//     let mut map_local = LocalVarMapping::new();
//     let mut map_free = FreeVarMapping::new();
//     let mut scope_ = scope.clone();

//     if let VarTree::FUNC(func_name, children, freevals) = func {
//         // 束縛変数
//         let mut i = 0;
//         for c in children.iter() {
//             if let VarTree::VAR(s) = c {
//                 // println!("{} {} {}", func_name, i, s);
//                 map_local.insert(s.to_string(), i);
//                 scope_.push((s.to_string(), 0, func_name.to_string()));
//                 i += 1;
//             } else if let VarTree::FUNC(func_name_child, _, _) = c {
//                 let mut scope_2 = scope_.clone();
//                 for j in 0..scope_2.len() {
//                     let (v, n, f) = &scope_2[j];
//                     scope_2[j] = (v.to_string(), n + 1, f.to_string());
//                 }
//                 write_to_map(map, &c, scope_2);
//                 // 内側関数の自由変数 \ 外側関数の束縛変数 は 外側関数の自由変数として扱う
//                 let mut u = map[func_name_child].1; // 内側の自由変数
//                 u.retain(|x| map_local.contains(x));
//                 for val_inner_free_carry in u {
//                     let (n_plus_1, scope_new, closure_idx) = map[&c].1[val_inner_free_carry];
//                     map_free.insert(val_inner_free_carry, (n_plus_1 - 1, scope_new, closure_idx));
//                 }
//             }
//         }

//         // 自由変数
//         // println!("map:{:?}\nfunc:{:?}\nscope:{:?}", map, func, scope);
//         // println!("freevals: {:?}", freevals);
//         // println!("scope: {:?}", scope);
//         let mut j = 0;
//         for s in freevals {
//             let mut found = false;
//             for (v, n, f) in scope.iter().rev() {
//                 if v == s {
//                     map_free.insert(v.to_string(), (*n, f.to_string(), j));
//                     found = true;
//                     j += 1;
//                     break;
//                 }
//             }
//             if !found {
//                 println!("write_to_map: undefined variable: {}", s);
//             }
//         }

//         let entry: VarMapEntry = (map_local, map_free);
//         map.insert(func_name.to_string(), entry);
//     } else {
//         println!("write_to_map: not a function: {:?}", func);
//     }
// }

pub fn vartree_to_varmap(tree: VarTree) -> Result<VarMap, String> {
    let mut map_local_main = LocalVarMapping::new();
    let mut map_free_main = FreeVarMapping::new();
    let mut map = VarMap::new();

    let r = write_to_map(&mut map, &tree, &mut map_local_main, &mut map_free_main);
    if let Ok(v) = r {
        if !v.is_empty() {
            return Err(format!("_main has freevals: {:?}", v));
        }
    } else if let Err(e) = r {
        return Err(format!("failed to construct varmap: {}", e));
    }

    Ok(map)
}
