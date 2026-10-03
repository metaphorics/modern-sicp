let factorial n =
  let total = ref 1 in
  let counter = ref 0 in
  let rec loop step =
    if !counter = n then !total
    else (
      counter := !counter + 1;
      total := !total * !counter;
      loop step)
  in
  loop 0

let () = print_int (factorial 6)

let () = print_newline ()
