(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.7 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

type let_star =
  { bindings : (string * Ast.expr) list
  ; body : Ast.expr
  }

let rec let_star_to_nested_lets { bindings; body } =
  match bindings with
  | [] -> body
  | (name, init) :: rest ->
    Ast.let_
      false
      [ { Ast.name = Some name; rhs = init } ]
      (let_star_to_nested_lets { bindings = rest; body })
;;

let eval_let_star (eval : S.eval_t) shape env = eval (let_star_to_nested_lets shape) env

let example () =
  let open_in names source = Sec_4_1.open_expression names source in
  let* x = open_in [] "3" in
  let* y = open_in [ "x" ] "x + 2" in
  let* z = open_in [ "x"; "y" ] "x + y + 5" in
  let* body = open_in [ "x"; "y"; "z" ] "x * z" in
  Ok { bindings = [ "x", x; "y", y; "z", z ]; body }
;;

let ex_4_07 () =
  let show = function
    | Ok v -> Value.to_string v
    | Error err -> "error: " ^ Eval_error.to_string err
  in
  let* shape = example () in
  let* one = Sec_4_1.open_expression [] "1" in
  let* next = Sec_4_1.open_expression [ "x" ] "x + 1" in
  let* tenfold = Sec_4_1.open_expression [ "x" ] "x * 10" in
  let rebinding = { bindings = [ "x", one; "x", next ]; body = tenfold } in
  let env () = S.the_global_environment () in
  Ok
    [ show (eval_let_star Sec_4_6.eval shape (env ()))
    ; show (eval_let_star S.eval_expr shape (env ()))
    ; show (eval_let_star Sec_4_6.eval rebinding (env ()))
    ]
;;
