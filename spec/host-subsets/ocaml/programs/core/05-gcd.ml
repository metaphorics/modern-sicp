let rec gcd a b = if b = 0 then a else gcd b (a mod b)

let () = print_int (gcd 206 40)

let () = print_newline ()
