(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.20 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error

let letrec_to_lets e =
  match Ast.view e with
  | Ast.Let (true, _, _) -> Ok (Sec_4_16.scan_out_let_rec e)
  | _ -> Error (Eval_error.Invalid_form "letrec_to_lets: not a let rec")
;;

let rec eval e env =
  match Ast.view e with
  | Ast.Let (true, _, _) ->
    let* lets = letrec_to_lets e in
    eval lets env
  | _ -> Sec_4_16.scanning Fun.id ~self:eval e env
;;

let parity binder =
  "let f x = let "
  ^ binder
  ^ " even n = if n = 0 then true else odd (n - 1) and odd n = if n = 0 then false else \
     even (n - 1) in even x in f 5"
;;

let ex_4_20 () =
  [ Sec_4_1.run_source eval (parity "rec")
  ; Sec_4_1.run_source
      eval
      "let rec fact n = if n = 1 then 1 else n * fact (n - 1) in fact 10"
  ; Sec_4_1.run_source eval (parity "")
  ]
;;
