let twice x = x + x

let () =
  let loud = let () = print_endline "computed" in 2 in
  print_int (twice loud)

let () = print_newline ()
