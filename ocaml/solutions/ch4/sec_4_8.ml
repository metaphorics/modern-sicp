(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.8 *)

(** Named let. The lowering of exercise 4.6 extends to the named form:
    [(let name ((v init) ...) body)] becomes the sequence of a
    definition of [name] as the procedure over the binding variables,
    then the call of [name] on the inits. The call finds the definition
    in the defining environment, so the body can recurse through
    [name]. The shared reader has no named let, so the shape arrives
    through the typed interface. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(** [named_let_to_combination name bindings body] lowers one named let
    to a definition and a call. *)
let named_let_to_combination name bindings body =
  let parameters = List.map fst bindings in
  let inits = List.map snd bindings in
  Ast.define_function name parameters body
  >>= fun d ->
  Ast.sequence [ Ast.definition d; Ast.application (Ast.variable name) inits ]
;;

(** [eval_named name bindings body env] lowers and evaluates one named
    let. *)
let eval_named name bindings body env =
  named_let_to_combination name bindings body >>= fun lowered -> SE.eval lowered env
;;

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [fib_iter_shape] is the statement's iterative Fibonacci as a named
    let over a prebound [n]. *)
let fib_iter_shape =
  ( "fib-iter"
  , [ "a", Ast.int 1; "b", Ast.int 0; "count", Ast.variable "n" ]
  , [ Ast.if_
        (Ast.application (Ast.variable "=") [ Ast.variable "count"; Ast.int 0 ])
        (Ast.variable "b")
        (Some
           (Ast.application
              (Ast.variable "fib-iter")
              [ Ast.application (Ast.variable "+") [ Ast.variable "a"; Ast.variable "b" ]
              ; Ast.variable "a"
              ; Ast.application (Ast.variable "-") [ Ast.variable "count"; Ast.int 1 ]
              ]))
    ] )
;;

(** [sum_loop_shape] sums [0 + 1 + 2 + 3 + 4] by recursion on the named
    procedure, the second use the statement asks for. *)
let sum_loop_shape =
  ( "sum-loop"
  , [ "i", Ast.int 0; "acc", Ast.int 0 ]
  , [ Ast.if_
        (Ast.application (Ast.variable "=") [ Ast.variable "i"; Ast.int 5 ])
        (Ast.variable "acc")
        (Some
           (Ast.application
              (Ast.variable "sum-loop")
              [ Ast.application (Ast.variable "+") [ Ast.variable "i"; Ast.int 1 ]
              ; Ast.application
                  (Ast.variable "+")
                  [ Ast.variable "acc"; Ast.variable "i" ]
              ]))
    ] )
;;

(** [ex_4_08 ()] binds [n] to 10, evaluates the fib-iter named let, and
      evaluates the summing named let. *)
let ex_4_08 () =
  let env = SE.the_global_environment () in
  let define_n = SE.run env "(define n 10)" in
  let name, bindings, body = fib_iter_shape in
  let fib = eval_named name bindings body env in
  let sum_name, sum_bindings, sum_body = sum_loop_shape in
  let sum = eval_named sum_name sum_bindings sum_body env in
  List.map render [ define_n; fib; sum ]
;;
