let rec fib = fun n -> if n <= 1 then 1 else fib (n-1) + fib (n-2) in
print_int (fib 10); print_string "\n"