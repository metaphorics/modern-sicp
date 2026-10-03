let rec factorial n = if n = 0 then 1 else n * factorial (n - 1)

let () = print_int (factorial 5)

let () = print_newline ()
