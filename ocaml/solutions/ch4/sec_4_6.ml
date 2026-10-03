(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.6 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

(* A [_ = e] binding still runs [e]; its parameter gets a name no
   source identifier can spell, so the body cannot see it. *)
let parameter index (b : Ast.binding) =
  match b.name with
  | Some name -> name
  | None -> "unused " ^ string_of_int index
;;

let let_to_combination e =
  match Ast.view e with
  | Ast.Let (false, bindings, body) ->
    let parameters = List.mapi parameter bindings in
    let inits = List.map (fun (b : Ast.binding) -> b.rhs) bindings in
    Ok (Ast.apply ~at:(Ast.at e) (Ast.fun_ ~at:(Ast.at e) parameters body) inits)
  | Ast.Let (true, _, _) ->
    Error
      (Eval_error.Invalid_form "let_to_combination: a recursive let is not a combination")
  | _ -> Error (Eval_error.Invalid_form "let_to_combination: not a let")
;;

let rec eval e env =
  match Ast.view e with
  | Ast.Let (false, _, _) ->
    let* combination = let_to_combination e in
    eval combination env
  | _ -> S.open_eval ~self:eval e env
;;

let ex_4_06 () =
  let show = function
    | Ok v -> Value.to_string v
    | Error err -> "error: " ^ Eval_error.to_string err
  in
  let lowered_to_base =
    let* e = Sec_4_1.open_expression [] "let x = 3 and y = 4 in x + y" in
    let* combination = let_to_combination e in
    S.eval_expr combination (S.the_global_environment ())
  in
  [ Sec_4_1.run_source eval "let x = 3 and y = 4 in x + y"
  ; show lowered_to_base
  ; Sec_4_1.run_source eval "let x = 2 in let x = 3 and y = x in x + y"
  ; Sec_4_1.run_source eval "let _ = print_string \"effect \" in 1"
  ; Sec_4_1.run_source
      eval
      "let rec fact n = if n = 0 then 1 else n * fact (n - 1) in fact 5"
  ]
;;
