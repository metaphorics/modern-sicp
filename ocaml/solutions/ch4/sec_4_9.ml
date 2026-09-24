(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.9 *)

(** A [while] iteration construct as a derived expression. The shape
    carries the test and the body; the lowering is the named-let
    expansion of exercise 4.8 with a nullary loop procedure:
    [(define %while-loop (lambda () (if test (begin body ... (%while-loop)) #f)))]
    followed by [(%while-loop)]. The nullary recursion answers the
    false object at exit and the body values are discarded, so the
    construct communicates through the environment it mutates. A second
    construct, [until], would differ only in the branch polarity: the
    same loop shape with the consequent and alternative swapped. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(** A while shape: the test and the body of one while form. *)
type shape = Ast.expr * Ast.expr list

(** [while_to_combination shape] lowers one while shape to a definition
    and a call. *)
let while_to_combination (test, body) =
  let name = "%while-loop" in
  let call = Ast.application (Ast.variable name) [] in
  Ast.sequence (body @ [ call ])
  >>= fun iteration ->
  Ast.define_function name [] [ Ast.if_ test iteration (Some (Ast.bool false)) ]
  >>= fun d -> Ast.sequence [ Ast.definition d; call ]
;;

(** [eval_while shape env] lowers and evaluates one while. *)
let eval_while shape env =
  while_to_combination shape >>= fun lowered -> SE.eval lowered env
;;

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [while_sum_shape] accumulates the sum 1 to 5 into the prebound
    [sum] while [i] advances toward 6. *)
let while_sum_shape =
  ( Ast.application (Ast.variable "<") [ Ast.variable "i"; Ast.int 6 ]
  , [ Ast.set
        "sum"
        (Ast.application (Ast.variable "+") [ Ast.variable "sum"; Ast.variable "i" ])
    ; Ast.set "i" (Ast.application (Ast.variable "+") [ Ast.variable "i"; Ast.int 1 ])
    ] )
;;

(** [ex_4_09 ()] binds the accumulator and the counter, runs the while,
      and reads the accumulator. *)
let ex_4_09 () =
  let env = SE.the_global_environment () in
  let define_sum = SE.run env "(define sum 0)" in
  let define_i = SE.run env "(define i 1)" in
  let loop = eval_while while_sum_shape env in
  let sum = SE.run env "sum" in
  List.map render [ define_sum; define_i; loop; sum ]
;;
