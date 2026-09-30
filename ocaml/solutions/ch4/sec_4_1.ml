(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.1 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

let rec list_of_values_left_to_right (eval : S.eval_t) exps env =
  match exps with
  | [] -> Ok []
  | exp :: rest ->
    let* value = eval exp env in
    let* values = list_of_values_left_to_right eval rest env in
    Ok (value :: values)
;;

let rec list_of_values_right_to_left (eval : S.eval_t) exps env =
  match exps with
  | [] -> Ok []
  | exp :: rest ->
    let* values = list_of_values_right_to_left eval rest env in
    let* value = eval exp env in
    Ok (value :: values)
;;

(* The application clause is the only one that changes; every other
   node falls back on the standard dispatch, which evaluates its
   subexpressions and closure bodies through the same fixed point. *)
let with_operand_order list_of_values : S.eval_t =
  let rec eval e env =
    match Ast.view e with
    | Ast.Apply (operator, operands) ->
      let* procedure = eval operator env in
      let* arguments = list_of_values eval operands env in
      S.apply_with ~self:eval procedure arguments
    | _ -> S.open_eval ~self:eval e env
  in
  eval
;;

let eval_left_to_right = with_operand_order list_of_values_left_to_right
let eval_right_to_left = with_operand_order list_of_values_right_to_left

let run_source (eval : S.eval_t) source =
  match S.expression source with
  | Error d -> "rejected: " ^ d
  | Ok e ->
    let out = Buffer.create 64 in
    let env = S.the_global_environment ~emit:(Buffer.add_string out) () in
    (match eval e env with
     | Ok v -> Buffer.contents out ^ Value.to_string v
     | Error err -> Buffer.contents out ^ "error: " ^ Eval_error.to_string err)
;;

let open_expression names source =
  let rec strip count e =
    if count = 0
    then Ok e
    else (
      match Ast.view e with
      | Ast.Fun (parameters, body) when List.length parameters <= count ->
        strip (count - List.length parameters) body
      | _ -> Error (Eval_error.Invalid_form "open_expression: no enclosing function"))
  in
  let wrapped =
    match names with
    | [] -> source
    | _ -> "fun " ^ String.concat " " names ^ " -> (" ^ source ^ ")"
  in
  match S.expression wrapped with
  | Error d -> Error (Eval_error.Invalid_form d)
  | Ok e -> strip (List.length names) e
;;

let ex_4_01 () =
  let difference =
    "(fun x y -> x - y) (print_string \"left \"; 10) (print_string \"right \"; 3)"
  in
  let failing =
    "(fun x y -> x - y) (print_string \"left \"; 1 / 0) (print_string \"right \"; 3)"
  in
  [ run_source eval_left_to_right difference
  ; run_source eval_right_to_left difference
  ; run_source eval_left_to_right failing
  ; run_source eval_right_to_left failing
  ]
;;
