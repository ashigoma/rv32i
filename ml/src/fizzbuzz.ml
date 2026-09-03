let rec mod_ = fun x -> fun r ->
  if x >= r then mod_ (x - r) r else x;;

let fizzbuzz = fun x ->
  if mod_ x 3 = 0 then
    if mod_ x 5 = 0 then print_string "FizzBuzz"
    else print_string "Fizz"
  else
    if mod_ x 5 = 0 then print_string "Buzz"
    else print_int x;;

let rec main = fun x ->
  if x <= 15 then (
    fizzbuzz x;
    print_string "\n";
    main (x + 1)
  ) else ();;

main 1;;