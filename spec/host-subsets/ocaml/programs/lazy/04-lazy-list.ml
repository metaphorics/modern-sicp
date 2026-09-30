type 'a lazy_list =
  | LNil
  | LCons of 'a * (unit -> 'a lazy_list)

let rec ones = LCons (1, function () -> ones)

let first_item items = match items with LNil -> 0 | LCons (item, _) -> item

let rest items = match items with LNil -> LNil | LCons (_, tail) -> tail ()

let () =
  let stream = LCons (1, function () -> LCons (2, function () -> LNil)) in
  print_int (first_item (rest stream))

let () = print_newline ()

let () = print_int (first_item (rest (rest ones)) + 0)

let () = print_newline ()
