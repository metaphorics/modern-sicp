type term =
  | Number of int
  | Sum of term * term
  | Product of term * term

let rec evaluate term =
  match term with
  | Number n -> n
  | Sum (left, right) -> evaluate left + evaluate right
  | Product (left, right) -> evaluate left * evaluate right

let rec render term =
  match term with
  | Number n -> string_of_int n
  | Sum (left, right) -> "(" ^ render left ^ " + " ^ render right ^ ")"
  | Product (left, right) -> "(" ^ render left ^ " * " ^ render right ^ ")"

let document = Sum (Number 1, Product (Number 2, Number 3))

let () = print_endline (render document)

let () = print_int (evaluate document)

let () = print_newline ()
