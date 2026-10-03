let choose_first x y = x

let () =
  let result =
    choose_first 1 (let () = print_endline "not-evaluated" in 2)
  in
  print_int result

let () = print_newline ()
