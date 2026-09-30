(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2

(* The lifted list clause: a list cell written in the program's list
   notation delays its element and its tail exactly as the program's
   own [cons] does, so the literal is the same lazy structure. *)
let lifted st ~self e env =
  match Ast.view e with
  | Ast.Cons (head, tail) ->
    Ok (Value.cons (Lazy_eval.delay st head env) (Lazy_eval.delay st tail env))
  | _ -> Lazy_eval.open_eval ~self st e env
;;

let section st ~self e env = Lazy_eval.open_eval ~self st e env

let run open_ source =
  let st = Lazy_eval.state () in
  Sicp_ch4.Sec_4_1.transcript
    ~experiment:Check.Lazy
    (Lazy_eval.run_with ~self:(Lazy_eval.fix (open_ st)) st)
    source
;;

let lazy_lists =
  "let cons x y = x :: y\n\
   let head_or default l = match l with x :: _ -> x | [] -> default\n\
   let tail l = match l with _ :: rest -> rest | [] -> []\n\
   let rec list_ref l n =\n\
  \  match l with\n\
  \  | [] -> 0\n\
  \  | x :: rest -> if n = 0 then x else list_ref rest (n - 1)\n"
;;

let second_of_literal =
  "let () = print_int (head_or 0 (tail [ 1 / 0; 42 ])); print_newline ()\n"
;;

let second_of_cons =
  "let () = print_int (head_or 0 (tail (cons (1 / 0) (cons 42 [])))); print_newline ()\n"
;;

let walk_literal =
  "let () = print_int (list_ref [ 1 / 0; 42; 1 / 0; 7 ] 3); print_newline ()\n"
;;

let ex_4_33 () =
  [ run section (lazy_lists ^ second_of_literal)
  ; run section (lazy_lists ^ second_of_cons)
  ; run lifted (lazy_lists ^ second_of_literal)
  ; run lifted (lazy_lists ^ walk_literal)
  ]
;;
