(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.5 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module S = Sicp_ch4.Sec_4_1

type clause =
  | Test of Ast.expr * Ast.expr
  | Arrow of Ast.expr * Ast.expr
  | Else of Ast.expr

(* A space cannot occur in a source identifier, so the payload's name
   never captures a variable of the recipient. *)
let payload = "cond value"
let invalid detail = Error (Eval_error.Invalid_form detail)

let rec cond_to_expr = function
  | [] -> invalid "a conditional needs a final Else clause"
  | [ Else e ] -> Ok e
  | Else _ :: _ -> invalid "Else must be the last clause"
  | Test (test, result) :: rest ->
    let* rest = cond_to_expr rest in
    Ok (Ast.if_ test result rest)
  | Arrow (test, recipient) :: rest ->
    let* rest = cond_to_expr rest in
    Ok
      (Ast.match_
         test
         [ ( Ast.pconstruct "Some" [ Ast.pvar payload ]
           , Ast.apply recipient [ Ast.var payload ] )
         ; Ast.pconstruct "None" [], rest
         ])
;;

let eval_cond clauses env =
  let* e = cond_to_expr clauses in
  S.eval_expr e env
;;

let ex_4_05 () =
  let expr = Sec_4_1.open_expression [] in
  let lookup key =
    expr
      (Printf.sprintf
         "let rec assoc l = match l with [] -> None | (k, v) :: rest -> if k = \"%s\" \
          then Some v else assoc rest in assoc [ (\"a\", 1); (\"b\", 2) ]"
         key)
  in
  let run clauses =
    match eval_cond clauses (S.the_global_environment ()) with
    | Ok v -> Sicp_common.Value.to_string v
    | Error e -> "error: " ^ Eval_error.to_string e
  in
  let* identity = expr "fun x -> x" in
  let* double = expr "fun x -> x * 2" in
  let* zero = expr "0" in
  let* seven = expr "7" in
  let* b = lookup "b" in
  let* c = lookup "c" in
  let* less = expr "1 < 2" in
  let* greater = expr "1 > 2" in
  Ok
    [ run [ Arrow (b, identity); Else zero ]
    ; run [ Arrow (c, double); Test (less, seven); Else zero ]
    ; run [ Test (greater, seven); Arrow (b, double); Else zero ]
    ; run [ Arrow (c, double); Else zero ]
    ; run [ Else zero; Test (less, seven) ]
    ]
;;
