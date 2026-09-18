let f = fun x -> (
  print_int x;
  print_string " is ";
  print_string (
    if x = 42 then "nice" else (
      if x < 42 then "small" else "big"
    )
  );
  print_string "\n"
) in
let g = fun x -> fun y -> (
  let x1 = x < 42 in let x2 = y < 42 in (
    print_string "(";
    print_int x;
    print_string ", ";
    print_int y;
    print_string ") is ";
    print_string (
      if x1 && x2 then "both small" else (
        if not x1 && not x2 then "both big" else "big and small"
      )
    );
    print_string "\n"
  )
)

in f 3; f 256; f 42;
g 52 78; g 3 5; g 1 100