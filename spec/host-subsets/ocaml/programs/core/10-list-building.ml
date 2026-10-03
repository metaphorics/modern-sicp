let rec sum numbers =
  match numbers with
  | [] -> 0
  | head :: tail -> head + sum tail

let () = print_int (sum [ 1; 2; 3; 4 ])

let () = print_newline ()

let () = print_int (sum (5 :: 6 :: []))

let () = print_newline ()
