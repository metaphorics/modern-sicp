(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.8 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

type named_let =
  { name : string
  ; bindings : (string * Ast.expr) list
  ; body : Ast.expr
  }

type let_form =
  | Plain of Ast.expr
  | Named of named_let

(* The procedure is bound by a [let rec] whose own body is only the
   name, and the inits are operands outside it: they see the enclosing
   scope, never the procedure's name. *)
let let_to_combination = function
  | Plain e -> Sec_4_6.let_to_combination e
  | Named { name; bindings; body } ->
    let parameters, arguments =
      match bindings with
      | [] -> [ "unit argument" ], [ Ast.scalar Ast.Unit ]
      | _ -> List.map fst bindings, List.map snd bindings
    in
    let procedure =
      Ast.let_
        true
        [ { Ast.name = Some name; rhs = Ast.fun_ parameters body } ]
        (Ast.var name)
    in
    Ok (Ast.apply procedure arguments)
;;

let eval_let_form form env =
  let* combination = let_to_combination form in
  Sec_4_6.eval combination env
;;

let fib_body () =
  Sec_4_1.open_expression
    [ "fib_iter"; "a"; "b"; "count" ]
    "if count = 0 then b else fib_iter (a + b) a (count - 1)"
;;

let fib () =
  let* body = fib_body () in
  let* one = Sec_4_1.open_expression [] "1" in
  let* zero = Sec_4_1.open_expression [] "0" in
  let loop =
    Named
      { name = "fib_iter"
      ; bindings = [ "a", one; "b", zero; "count", Ast.var "n" ]
      ; body
      }
  in
  let* combination = let_to_combination loop in
  Ok (Ast.fun_ [ "n" ] combination)
;;

let ex_4_08 () =
  let show = function
    | Ok v -> Value.to_string v
    | Error err -> "error: " ^ Eval_error.to_string err
  in
  let* fib = fib () in
  let* ten = Sec_4_1.open_expression [] "10" in
  let* plain = Sec_4_1.open_expression [] "let x = 3 and y = 4 in x * y" in
  let* count = Sec_4_1.open_expression [] "10" in
  let* shadowed =
    (* The init [count] names the outer binding even though the loop
       procedure has a parameter of the same name. *)
    Sec_4_1.open_expression [ "loop"; "count" ] "if count = 0 then 0 else count"
  in
  let env () = S.the_global_environment () in
  let outer = Sicp_common.Env.bind "count" (Value.int 7) (env ()) in
  Ok
    [ show (Sec_4_6.eval (Ast.apply fib [ ten ]) (env ()))
    ; show (eval_let_form (Plain plain) (env ()))
    ; show
        (eval_let_form
           (Named
              { name = "loop"; bindings = [ "count", Ast.var "count" ]; body = shadowed })
           outer)
    ; show (eval_let_form (Named { name = "loop"; bindings = []; body = count }) (env ()))
    ]
;;
