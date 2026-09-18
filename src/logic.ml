let print_bool = fun x -> (print_string (if x = 0 then "true " else "false ")) in
print_bool (true || true);
print_bool (true || false);
print_bool (false || false);
print_string "\n";
print_bool (true && true);
print_bool (true && false);
print_bool (false && false);
print_string "\n";
print_bool (not false);
print_bool (not true);
print_string "\n"