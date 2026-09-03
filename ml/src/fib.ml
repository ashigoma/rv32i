let rec mod_ = fun x r -> if x >= r then mod_ (x - r) r else x;;

let fizzbuzz = fun x ->
    let is_fizz = mod_ x 3 = 0 in
    let is_buzz = mod_ x 5 = 0 in
    if is_fizz then print_string "Fizz" else ();
    if is_buzz then print_string "Buzz" else ();
    if not is_fizz && not is_buzz then print_int x else ();
    print_string "\n";;

let rec main = fun x ->
  if x < 15 then (fizzbuzz x; main (x+1)) else ();;

main 0;;