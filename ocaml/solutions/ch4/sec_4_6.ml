(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.6 *)

(** [let] as a derived expression. [let_to_combination] rewrites one
    [let] into the application of a lambda over the init expressions;
    the clause fires at every depth because the dispatch recurses
    through [Ev] itself. The inits are operands of the application, so
    they are evaluated in the outer environment, which is the scoping
    the statement's equivalence demands. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Reader = Sicp_common.Reader
module Value = Sicp_common.Value
module SE = Sicp_ch4.Sec_4_1

(** [let_to_combination exp] is the equivalent application of a
    lambda. *)
let let_to_combination exp =
  match Ast.view exp with
  | Ast.Let (bindings, body) ->
    let parameters = List.map fst bindings in
    let inits = List.map snd bindings in
    Ast.lambda parameters body >>= fun proc -> Ok (Ast.application proc inits)
  | _ -> Error (Eval_error.Invalid_form "let_to_combination: not a let")
;;

module rec Ev : sig
  val eval : SE.eval_t
end = struct
  module C = SE.Core (Ev)

  let eval exp env =
    match Ast.view exp with
    | Ast.Let _ -> let_to_combination exp >>= fun lowered -> Ev.eval lowered env
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

(** [ex_4_06 ()] evaluates one [let] through the clause, the same
      lowered combination through the plain base evaluator, and the
      shadowing program whose inner inits must still see the outer
      bindings. *)
let ex_4_06 () =
  let env = SE.the_global_environment () in
  let through_clause = run env "(let ((x 3) (y 4)) (+ x y))" in
  let lowered_to_base =
    Reader.read "(let ((x 3) (y 4)) (+ x y))"
    |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
    >>= let_to_combination
    >>= fun lowered -> SE.eval lowered env
  in
  let shadowing = run env "(let ((x 2)) (let ((x 3) (y x)) (+ x y)))" in
  List.map render [ through_clause; lowered_to_base; shadowing ]
;;
