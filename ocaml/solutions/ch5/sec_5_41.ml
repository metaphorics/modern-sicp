(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.41: [find-variable] -- the compiler's own
    [find_variable], answered here with the exercise's three cases and
    the edition's shape: a lexical address is the pair of the frame
    number and the displacement, and a miss is [None] (the book's
    [not-found]). *)

module C = Sicp_ch5.Sec_5_5

(** [render] is the address as the book prints it: [(frame
    displacement)], or [not-found]. *)
let render = function
  | Some (frame, displacement) -> Printf.sprintf "(%d %d)" frame displacement
  | None -> "not-found"
;;

let cenv_of_example = [ [ "y"; "z" ]; [ "a"; "b"; "c"; "d"; "e" ]; [ "x"; "y" ] ]

(** [ex_5_41 ()] runs the book's three lookups over the fragment's
    compile-time environment [( (y z) (a b c d e) (x y) )]. *)
let ex_5_41 () =
  let lookups =
    [ "c", C.find_variable "c" cenv_of_example
    ; "x", C.find_variable "x" cenv_of_example
    ; "w", C.find_variable "w" cenv_of_example
    ]
  in
  Ok (List.map (fun (name, addr) -> Printf.sprintf "%s: %s" name (render addr)) lookups)
;;
