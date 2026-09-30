(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.2 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module S = Sicp_ch4.Sec_4_1

let subexpressions e =
  match Ast.view e with
  | Ast.Scalar _ | Ast.Var _ | Ast.Nil -> []
  | Ast.Let (_, bindings, body) ->
    List.map (fun (b : Ast.binding) -> b.rhs) bindings @ [ body ]
  | Ast.Fun (_, body) -> [ body ]
  | Ast.Apply (operator, operands) -> operator :: operands
  | Ast.If (c, t, f) -> [ c; t; f ]
  | Ast.Match (scrutinee, cases) -> scrutinee :: List.map snd cases
  | Ast.Tuple parts | Ast.Construct (_, parts) -> parts
  | Ast.Record fields -> List.map snd fields
  | Ast.Field (record, _) -> [ record ]
  | Ast.Sequence (a, b)
  | Ast.And (a, b)
  | Ast.Or (a, b)
  | Ast.Arith (_, a, b)
  | Ast.Compare (_, a, b)
  | Ast.Cons (a, b)
  | Ast.Concat (a, b)
  | Ast.Assign (a, b) -> [ a; b ]
  | Ast.Not a | Ast.Neg a | Ast.Deref a | Ast.Make_ref a -> [ a ]
;;

(* Louis's order: the application test comes first, and it asks only
   whether the node is compound, the way [application?] asks [pair?]. *)
let rec louis_eval e env =
  match subexpressions e with
  | operator :: operands ->
    let* procedure = louis_eval operator env in
    let* arguments = Sec_4_1.list_of_values_left_to_right louis_eval operands env in
    S.apply_with ~self:louis_eval procedure arguments
  | [] -> S.open_eval ~self:louis_eval e env
;;

let rec eval e env =
  match Ast.view e with
  | Ast.Apply (operator, operands) ->
    (match Ast.view operator, operands with
     | Ast.Var "call", procedure :: operands ->
       let* procedure = eval procedure env in
       let* arguments = Sec_4_1.list_of_values_left_to_right eval operands env in
       S.apply_with ~self:eval procedure arguments
     | Ast.Var "call", [] -> Error (Eval_error.Invalid_form "call needs an operator")
     | _ -> Error (Eval_error.Invalid_form "an application must start with call"))
  | _ -> S.open_eval ~self:eval e env
;;

let ex_4_02 () =
  [ Sec_4_1.run_source louis_eval "let x = 3 in x"
  ; Sec_4_1.run_source louis_eval "1 + 2"
  ; Sec_4_1.run_source louis_eval "string_of_int 42"
  ; Sec_4_1.run_source eval "let call f = f in call string_of_int 42"
  ; Sec_4_1.run_source eval "let call f = f in let mul x y = x * y in call mul 6 7"
  ; Sec_4_1.run_source
      eval
      "let call f = f in let rec fact n = if n = 0 then 1 else n * call fact (n - 1) in \
       call fact 5"
  ; Sec_4_1.run_source eval "let twice x = x + x in twice 4"
  ]
;;
