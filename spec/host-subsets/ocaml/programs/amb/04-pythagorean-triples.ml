let () =
  let i = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let j = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  let k = amb 1 (amb 2 (amb 3 (amb 4 5))) in
  require (i <= j);
  require (i * i + j * j = k * k);
  print_endline (string_of_int i ^ "," ^ string_of_int j ^ "," ^ string_of_int k)
