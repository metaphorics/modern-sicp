(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Core = Sicp_ch4.Sec_4_1

let budget = 200

(* The applicative-order evaluator with a fuel meter: every application
   node spends one unit, so a descent that never reaches its base case
   stops with a typed error instead of exhausting the host stack. *)
let fuel_bounded () : Core.eval_t =
  let spent = ref 0 in
  let rec eval e env =
    match Ast.view e with
    | Ast.Apply _ ->
      incr spent;
      if !spent > budget
      then
        Error
          (Eval_error.User_error
             (Printf.sprintf
                "the strict evaluation is still descending after %d applications"
                budget))
      else Core.open_eval ~self:eval e env
    | _ -> Core.open_eval ~self:eval e env
  in
  eval
;;

let run_strict source =
  match Core.expression source with
  | Error rejection -> "rejected: " ^ rejection
  | Ok e ->
    let out = Buffer.create 64 in
    let env = Core.the_global_environment ~emit:(Buffer.add_string out) () in
    (match fuel_bounded () e env with
     | Ok _ -> Buffer.contents out
     | Error err -> Buffer.contents out ^ "error: " ^ Eval_error.to_string err)
;;

let run_lazy source = Core.transcript ~experiment:Check.Lazy Sicp_ch4.Sec_4_2.run source

let unless_definition =
  "let unless condition usual_value exceptional_value =\n\
  \  if condition then exceptional_value else usual_value\n"
;;

let factorial_definition =
  "let rec factorial n = unless (n = 1) (n * factorial (n - 1)) 1\n"
;;

let ex_4_25 () =
  [ run_lazy
      (unless_definition
       ^ factorial_definition
       ^ "let () = print_int (factorial 5); print_newline ()")
  ; run_lazy
      (unless_definition
       ^ "let () = print_int (unless (1 = 1) (1 / 0) 42); print_newline ()")
  ; run_strict
      (unless_definition
       ^ "in\n"
       ^ factorial_definition
       ^ "in\nprint_int (factorial 5); print_newline ()")
  ; run_strict (unless_definition ^ "in\nprint_int (unless (1 = 1) (1 / 0) 42)")
  ]
;;
