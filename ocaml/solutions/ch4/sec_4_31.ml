(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Ast = Sicp_common.Ast
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Core = Sicp_ch4.Sec_4_1

let ( let* ) = Result.bind

type mode =
  | Strict
  | Lazy
  | Lazy_memo

(* The declaration rides on the parameter's name: a [_lazy_memo]
   suffix delays through a memoized thunk, a [_lazy] suffix through an
   unmemoized one, and every other name stays strict. *)
let mode_of name =
  if String.ends_with ~suffix:"_lazy_memo" name
  then Lazy_memo
  else if String.ends_with ~suffix:"_lazy" name
  then Lazy
  else Strict
;;

(* [annotated] is the direct evaluator with two clauses replaced.  A
   variable bound to a thunk is forced where it is read, recording the
   result only for a [_lazy_memo] name.  An application of a closure
   binds each parameter per its declaration; every other operator
   receives strict operands. *)
let rec annotated e env =
  match Ast.view e with
  | Ast.Var name -> read name env
  | Ast.Apply (operator, operands) ->
    let* operator = annotated operator env in
    apply operator operands env
  | _ -> Core.open_eval ~self:annotated e env

and read name env =
  let* v = Core.open_eval ~self:annotated (Ast.var name) env in
  match Value.thunk_state_of v with
  | None -> Ok v
  | Some cell ->
    (match !cell with
     | Value.Forced memo -> Ok memo
     | Value.Delayed (expr, thunk_env) ->
       let* forced = annotated expr thunk_env in
       if mode_of name = Lazy_memo then Value.set_thunk_state cell (Value.Forced forced);
       Ok forced)

and apply operator operands env =
  match Value.view operator, operands with
  | _, [] -> Ok operator
  | ( Value.Closure { parameters = parameter :: parameters; body; env = closure_env; _ }
    , operand :: rest ) ->
    let* argument = bind parameter operand env in
    let closure_env = Env.bind parameter argument closure_env in
    if parameters = []
    then
      let* result = annotated body closure_env in
      apply result rest env
    else apply (Value.closure ~name:None ~parameters ~body ~env:closure_env) rest env
  | _ ->
    let* arguments = strict_all operands env in
    Core.apply_with ~self:annotated operator arguments

and bind parameter operand env =
  match mode_of parameter with
  | Strict -> annotated operand env
  | Lazy | Lazy_memo -> Ok (Value.thunk ~expr:operand ~env)

and strict_all operands env =
  List.fold_left
    (fun acc operand ->
       let* values = acc in
       let* v = annotated operand env in
       Ok (v :: values))
    (Ok [])
    operands
  |> Result.map List.rev
;;

let run source =
  match Core.expression source with
  | Error rejection -> "rejected: " ^ rejection
  | Ok e ->
    let out = Buffer.create 64 in
    let env = Core.the_global_environment ~emit:(Buffer.add_string out) () in
    (match annotated e env with
     | Ok _ -> Buffer.contents out
     | Error err -> Buffer.contents out ^ "error: " ^ Eval_error.to_string err)
;;

let counted =
  "let count = ref 0 in\n\
   let id x = (count := !count + 1; x) in\n\
   let rec show l =\n\
  \  match l with\n\
  \  | [] -> print_newline ()\n\
  \  | x :: rest -> print_int x; print_string \" \"; show rest\n\
   in\n"
;;

let ex_4_31 () =
  [ run
      "let pick b_lazy = if true then \"taken\" else b_lazy in\n\
       print_endline (pick (string_of_int (1 / 0)))"
  ; run
      "let pick b = if true then \"taken\" else b in\n\
       print_endline (pick (string_of_int (1 / 0)))"
  ; run
      (counted
       ^ "let f a b_lazy c d_lazy_memo = [ a; b_lazy; b_lazy; c; d_lazy_memo; \
          d_lazy_memo ] in\n\
          show (f (id 1) (id (2 + 3)) (id 4) (id (5 * 6)));\n\
          print_int !count;\n\
          print_newline ()")
  ; run
      (counted
       ^ "let twice b_lazy = [ b_lazy; b_lazy ] in\n\
          let twice_m d_lazy_memo = [ d_lazy_memo; d_lazy_memo ] in\n\
          show (twice (id 10));\n\
          print_int !count;\n\
          print_newline ();\n\
          show (twice_m (id 10));\n\
          print_int !count;\n\
          print_newline ()")
  ]
;;
