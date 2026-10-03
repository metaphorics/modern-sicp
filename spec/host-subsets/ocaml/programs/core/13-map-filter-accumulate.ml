let () =
  print_int
    (List.fold_left
       (fun acc x -> acc + x)
       0
       (List.map (fun x -> x * x) (List.filter (fun x -> x mod 2 = 0) [ 1; 2; 3; 4; 5; 6 ])))

let () = print_newline ()
