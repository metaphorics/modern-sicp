let () =
  let x = amb 1 (amb 2 3) in
  require (x > 1);
  print_endline (string_of_int x)
