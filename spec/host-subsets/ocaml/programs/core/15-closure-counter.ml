let make_counter start =
  let cell = ref start in
  function () ->
  cell := !cell + 1;
  !cell

let () =
  let counter = make_counter 0 in
  let _ = counter () in
  print_int (counter ())

let () = print_newline ()
