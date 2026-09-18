let f = fun x -> (
  print_int x;
  print_string " is ";
  print_string (
    if x < 10 then "small" else "big"
  );
  print_string "\n"
) in f 12