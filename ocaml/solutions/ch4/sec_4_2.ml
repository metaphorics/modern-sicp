(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.2 *)

(** Louis Reasoner's call-prefix language. Part (a) is codeless here:
    with the typed [Ast] a definition is a distinct [Definition] view,
    so reordering the dispatch cannot mistake [(define x 3)] for an
    application, and Louis's premise dissolves. Part (b) is the
    deliverable: every application is written [(call f x ...)], and an
    application whose operator is anything else is a syntax error. The
    rule holds at every nesting depth, because the application clause
    recurses through [Ev] itself, and a procedure body is only
    evaluated, and checked, when the procedure is called. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

module rec Ev : sig
  val eval : SE.eval_t
end = struct
  module C = SE.Core (Ev)

  let eval exp env =
    match Ast.view exp with
    | Ast.Application (operator, operands) ->
      (match Ast.view operator with
       | Ast.Variable "call" ->
         (match operands with
          | [] -> Error (Eval_error.Invalid_form "the call form needs an operator")
          | real_operator :: real_operands ->
            Ev.eval real_operator env
            >>= fun proc ->
            C.list_of_values real_operands env >>= fun args -> C.apply_procedure proc args)
       | _ -> Error (Eval_error.Invalid_form "the application does not start with call"))
    | _ -> C.eval exp env
  ;;
end

let eval = Ev.eval

let run env text =
  Reader.read text
  |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
  >>= fun exp -> Ev.eval exp env
;;

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [ex_4_02 ()] evaluates one [call] application of a primitive, then a
      two-parameter procedure defined in the call language and applied
      with [call], then a plain application, which under the new syntax
      is an error. *)
let ex_4_02 () =
  let env = SE.the_global_environment () in
  let call_primitive = run env "(call + 1 2)" in
  let define_f = run env "(define (f x y) (call * x y))" in
  let call_f = run env "(call f 6 7)" in
  let plain_application = run env "(+ 1 2)" in
  List.map render [ call_primitive; define_f; call_f; plain_application ]
;;
