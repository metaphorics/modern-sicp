let rec count n = if n = 0 then 0 else 1 + count (n - 1)

let () = print_int (count 5000)

let () = print_newline ()
