let make_pair a b = fun choose -> choose a b

let first pair = pair (fun x y -> x)

let second pair = pair (fun x y -> y)

let () =
  let pair =
    make_pair (let () = print_endline "left" in 1) (let () = print_endline "right" in 2)
  in
  print_int (first pair)

let () = print_newline ()
