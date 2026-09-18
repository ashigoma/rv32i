print_int (42 = 42); print_string("\n");
let print_bool = fun x -> (print_string (if x = 0 then "true " else "false ")) in
print_bool (314159 > 42);
print_bool (42 > 42);
print_bool (42 > 314159);
print_string "\n";
print_bool (314159 >= 42);
print_bool (42 >= 42);
print_bool (42 >= 314159);
print_string "\n";
print_bool (314159 < 42);
print_bool (42 < 42);
print_bool (42 < 314159);
print_string "\n";
print_bool (314159 <= 42);
print_bool (42 <= 42);
print_bool (42 <= 314159);
print_string "\n";
print_bool (314159 = 42);
print_bool (42 = 42);
print_bool (42 = 314159);
print_string "\n";
print_bool (314159 != 42);
print_bool (42 != 42);
print_bool (42 != 314159);
print_string "\n"