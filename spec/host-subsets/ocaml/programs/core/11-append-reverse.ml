let rec append left right =
  match left with
  | [] -> right
  | head :: tail -> head :: append tail right

let rec reverse items =
  match items with
  | [] -> []
  | head :: tail -> append (reverse tail) [ head ]

let rec show_ints items =
  match items with
  | [] -> ""
  | first :: [] -> string_of_int first
  | first :: rest -> string_of_int first ^ "," ^ show_ints rest

let () = print_endline (show_ints (append [ 1; 2 ] [ 3; 4 ]))

let () = print_endline (show_ints (reverse [ 1; 2; 3 ]))
