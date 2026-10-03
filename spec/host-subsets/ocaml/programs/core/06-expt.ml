let rec expt b n = if n = 0 then 1 else b * expt b (n - 1)

let rec fast_expt b n =
  if n = 0
  then 1
  else if n mod 2 = 0
  then (
    let half = fast_expt b (n / 2) in
    half * half)
  else b * fast_expt b (n - 1)

let () = print_int (expt 2 10)

let () = print_newline ()

let () = print_int (fast_expt 2 10)

let () = print_newline ()
