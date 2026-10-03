let is_even n = n mod 2 = 0

let () = print_endline (if is_even 4 then "even" else "odd")

let () = print_endline (if 1.5 < 2.5 then "less" else "more")

let () = print_endline (if "a" = "a" then "same" else "different")

let () = print_endline (if () = () then "unit" else "other")
