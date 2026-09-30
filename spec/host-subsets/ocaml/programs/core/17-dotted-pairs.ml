type 'a box = Box of 'a * 'a

let car box = match box with Box (a, _) -> a

let cdr box = match box with Box (_, b) -> b

let () =
  let pair = Box (Box (1, 2), Box (3, 4)) in
  print_int (car (car pair));
  print_int (cdr (car pair));
  print_int (car (cdr pair));
  print_int (cdr (cdr pair))

let () = print_newline ()
