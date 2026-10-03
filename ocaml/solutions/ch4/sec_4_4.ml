(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.4 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

let truth what v =
  match Value.view v with
  | Value.Bool b -> Ok b
  | _ -> Error (Eval_error.Type_error (what ^ ": operand is not a bool"))
;;

let eval_and (eval : S.eval_t) left right env =
  let* l = eval left env in
  let* b = truth "&&" l in
  if b then eval right env else Ok (Value.bool false)
;;

let eval_or (eval : S.eval_t) left right env =
  let* l = eval left env in
  let* b = truth "||" l in
  if b then Ok (Value.bool true) else eval right env
;;

let rec eval_special e env =
  match Ast.view e with
  | Ast.And (left, right) -> eval_and eval_special left right env
  | Ast.Or (left, right) -> eval_or eval_special left right env
  | _ -> S.open_eval ~self:eval_special e env
;;

let bool b = Ast.scalar (Ast.Bool b)
let and_to_if a b = Ast.if_ a b (bool false)
let or_to_if a b = Ast.if_ a (bool true) b

let rec derive e =
  match Ast.view e with
  | Ast.And (a, b) -> and_to_if (derive a) (derive b)
  | Ast.Or (a, b) -> or_to_if (derive a) (derive b)
  | _ -> Ast.map_children derive e
;;

let eval_derived : S.eval_t = fun e env -> S.eval_expr (derive e) env

let rec conjunction = function
  | [] -> bool true
  | [ e ] -> e
  | e :: rest -> and_to_if e (conjunction rest)
;;

let rec disjunction = function
  | [] -> bool false
  | [ e ] -> e
  | e :: rest -> or_to_if e (disjunction rest)
;;

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "error: " ^ Eval_error.to_string e
;;

let ex_4_04 () =
  let pair source =
    Sec_4_1.run_source eval_special source
    ^ " / "
    ^ Sec_4_1.run_source eval_derived source
  in
  let fold build sources =
    let* operands =
      List.fold_right
        (fun source acc ->
           let* acc = acc in
           let* e = Sec_4_1.open_expression [] source in
           Ok (e :: acc))
        sources
        (Ok [])
    in
    S.eval_expr (build operands) (S.the_global_environment ())
  in
  [ pair "false && 1 / 0 = 0"
  ; pair "true || 1 / 0 = 0"
  ; pair "1 < 2 && 3 < 4"
  ; pair "1 > 2 || 3 > 4"
  ; show (fold conjunction [])
  ; show (fold disjunction [])
  ; show (fold conjunction [ "1 < 2"; "2 < 3"; "3 < 1"; "1 / 0 = 0" ])
  ; show (fold disjunction [ "1 > 2"; "2 < 3"; "1 / 0 = 0" ])
  ]
;;
