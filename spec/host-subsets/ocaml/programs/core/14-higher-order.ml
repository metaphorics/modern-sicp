let compose f g = fun x -> f (g x)

let twice f = compose f f

let inc x = x + 1

let () = print_int ((twice inc) 5)

let () = print_newline ()

let () = print_int ((compose (fun x -> x * 2) (fun x -> x + 1)) 5)

let () = print_newline ()
