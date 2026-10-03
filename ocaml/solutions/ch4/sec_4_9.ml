(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.9 *)

let ( let* ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module S = Sicp_ch4.Sec_4_1

type loop =
  | While of Ast.expr * Ast.expr
  | For of string * Ast.expr * Ast.expr * Ast.expr

(* The helper names hold a space, which no source identifier can, so a
   loop body can neither see nor shadow them. *)
let loop_name = "loop procedure"
let limit_name = "loop limit"
let unit_name = "loop unit"
let unit_value = Ast.scalar Ast.Unit

let recursive name parameter body in_body =
  Ast.let_ true [ { Ast.name = Some name; rhs = Ast.fun_ [ parameter ] body } ] in_body
;;

let loop_to_expr = function
  | While (test, body) ->
    let again = Ast.apply (Ast.var loop_name) [ unit_value ] in
    recursive
      loop_name
      unit_name
      (Ast.if_ test (Ast.sequence body again) unit_value)
      (Ast.apply (Ast.var loop_name) [ unit_value ])
  | For (var, from, upto, body) ->
    let next = Ast.arith Ast.Add (Ast.var var) (Ast.scalar (Ast.Int 1)) in
    Ast.let_
      false
      [ { Ast.name = Some limit_name; rhs = upto } ]
      (recursive
         loop_name
         var
         (Ast.if_
            (Ast.compare_ Ast.Le (Ast.var var) (Ast.var limit_name))
            (Ast.sequence body (Ast.apply (Ast.var loop_name) [ next ]))
            unit_value)
         (Ast.apply (Ast.var loop_name) [ from ]))
;;

let eval_loop (eval : S.eval_t) loop env = eval (loop_to_expr loop) env

let show out = function
  | Ok v -> Buffer.contents out ^ Value.to_string v
  | Error err -> Buffer.contents out ^ "error: " ^ Eval_error.to_string err
;;

let summation () =
  let* test = Sec_4_1.open_expression [ "i"; "total" ] "!i <= 10" in
  let* step =
    Sec_4_1.open_expression [ "i"; "total" ] "total := !total + !i; i := !i + 1"
  in
  let int n = Ast.scalar (Ast.Int n) in
  Ok
    (Ast.let_
       false
       [ { Ast.name = Some "i"; rhs = Ast.make_ref (int 1) }
       ; { Ast.name = Some "total"; rhs = Ast.make_ref (int 0) }
       ]
       (Ast.sequence (loop_to_expr (While (test, step))) (Ast.deref (Ast.var "total"))))
;;

let ex_4_09 () =
  let run build =
    let out = Buffer.create 32 in
    let env = S.the_global_environment ~emit:(Buffer.add_string out) () in
    show out (Result.bind (build ()) (fun e -> S.eval_expr e env))
  in
  let squares () =
    let* body = Sec_4_1.open_expression [ "k" ] "print_int (k * k); print_string \" \"" in
    let* one = Sec_4_1.open_expression [] "1" in
    let* five = Sec_4_1.open_expression [] "2 + 3" in
    Ok (loop_to_expr (For ("k", one, five, body)))
  in
  let never () =
    let* test = Sec_4_1.open_expression [] "1 > 2" in
    let* body = Sec_4_1.open_expression [] "print_string \"never\"" in
    Ok (loop_to_expr (While (test, body)))
  in
  [ run summation
  ; run squares
  ; run never
  ; Sec_4_1.run_source S.eval_expr "let i = ref 0 in while !i < 3 do i := !i + 1 done; !i"
  ; Sec_4_1.run_source S.eval_expr "for k = 1 to 3 do print_int k done"
  ]
;;
