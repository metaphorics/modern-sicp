let rec prime n = if n < 2 then false else prime_test n 2

and prime_test n d =
  if d * d > n then true else if n mod d = 0 then false else prime_test n (d + 1)

let () =
  let i = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let j = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  require (i < j);
  require (prime (i + j));
  print_endline (string_of_int i ^ "-" ^ string_of_int j)
