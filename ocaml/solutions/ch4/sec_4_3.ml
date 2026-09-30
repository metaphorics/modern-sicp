(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.3 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

type handler = S.eval_t -> S.eval_t
type table = (string, handler) Hashtbl.t

let kind e =
  match Ast.view e with
  | Ast.Scalar _ -> "scalar"
  | Ast.Var _ -> "variable"
  | Ast.Let _ -> "let"
  | Ast.Fun _ -> "fun"
  | Ast.Apply _ -> "application"
  | Ast.If _ -> "if"
  | Ast.Match _ -> "match"
  | Ast.Tuple _ -> "tuple"
  | Ast.Construct _ -> "construct"
  | Ast.Record _ -> "record"
  | Ast.Field _ -> "field"
  | Ast.Sequence _ -> "sequence"
  | Ast.And _ -> "and"
  | Ast.Or _ -> "or"
  | Ast.Arith _ -> "arith"
  | Ast.Compare _ -> "compare"
  | Ast.Nil -> "nil"
  | Ast.Cons _ -> "cons"
  | Ast.Concat _ -> "concat"
  | Ast.Not _ -> "not"
  | Ast.Neg _ -> "neg"
  | Ast.Deref _ -> "deref"
  | Ast.Assign _ -> "assign"
  | Ast.Make_ref _ -> "make-ref"
;;

let put table name handler = Hashtbl.replace table name handler
let find table name = Hashtbl.find_opt table name

let wrong_kind name =
  Error (Eval_error.Invalid_form ("the " ^ name ^ " handler got another kind"))
;;

let truth what v =
  match Value.view v with
  | Value.Bool b -> Ok b
  | _ -> Error (Eval_error.Type_error (what ^ ": operand is not a bool"))
;;

let eval_if (self : S.eval_t) e env =
  match Ast.view e with
  | Ast.If (condition, consequent, alternative) ->
    let* test = self condition env in
    let* b = truth "if" test in
    if b then self consequent env else self alternative env
  | _ -> wrong_kind "if"
;;

let eval_and (self : S.eval_t) e env =
  match Ast.view e with
  | Ast.And (left, right) ->
    let* l = self left env in
    let* b = truth "&&" l in
    if b then self right env else Ok (Value.bool false)
  | _ -> wrong_kind "and"
;;

let eval_or (self : S.eval_t) e env =
  match Ast.view e with
  | Ast.Or (left, right) ->
    let* l = self left env in
    let* b = truth "||" l in
    if b then Ok (Value.bool true) else self right env
  | _ -> wrong_kind "or"
;;

let eval_sequence (self : S.eval_t) e env =
  match Ast.view e with
  | Ast.Sequence (first, second) ->
    let* _ = self first env in
    self second env
  | _ -> wrong_kind "sequence"
;;

let eval_fun (_ : S.eval_t) e env =
  match Ast.view e with
  | Ast.Fun (parameters, body) -> Ok (Value.closure ~name:None ~parameters ~body ~env)
  | _ -> wrong_kind "fun"
;;

let standard () =
  let table = Hashtbl.create 16 in
  put table "if" eval_if;
  put table "and" eval_and;
  put table "or" eval_or;
  put table "sequence" eval_sequence;
  put table "fun" eval_fun;
  table
;;

let eval table =
  let rec self e env =
    match find table (kind e) with
    | Some handler -> handler self e env
    | None -> S.open_eval ~self e env
  in
  self
;;

let ex_4_03 () =
  let table = standard () in
  let count = ref 0 in
  put table "if" (fun self e env ->
    incr count;
    eval_if self e env);
  let fact = "let rec fact n = if n = 0 then 1 else n * fact (n - 1) in fact 5" in
  let value = Sec_4_1.run_source (eval table) fact in
  [ value
  ; Printf.sprintf "if handled %d times" !count
  ; Sec_4_1.run_source (eval (standard ())) "true && (false || 1 = 1)"
  ]
;;
