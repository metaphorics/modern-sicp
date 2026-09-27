(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 exercise 4.4 *)

(** [and] and [or] as special forms and as derived expressions. The
    direct clauses short-circuit: an [and] stops at the first false
    operand and answers the false object, an [or] stops at the first
    true operand and answers its value. The rewrites produce ordinary
    forms: [and] nests [if] expressions over [true] and [false]
    leaves, and [or] binds each tested operand behind a one-parameter
    lambda, the temporary the base language has no [let] to name. *)

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

  let rec eval_and operands env =
    match operands with
    | [] -> Ok (Value.bool true)
    | [ last ] -> Ev.eval last env
    | next :: rest ->
      Ev.eval next env
      >>= fun value ->
      if SE.false_ value then Ok (Value.bool false) else eval_and rest env
  ;;

  let rec eval_or operands env =
    match operands with
    | [] -> Ok (Value.bool false)
    | next :: rest ->
      Ev.eval next env
      >>= fun value -> if SE.true_ value then Ok value else eval_or rest env
  ;;

  let eval exp env =
    match Ast.view exp with
    | Ast.And operands -> eval_and operands env
    | Ast.Or operands -> eval_or operands env
    | _ -> C.eval exp env
  ;;
end

let eval = Ev.eval

(** [and_to_if exp] is the derived-expression rewrite of one [and]. *)
let and_to_if exp =
  match Ast.view exp with
  | Ast.And operands ->
    let rec expand = function
      | [] -> Ok (Ast.bool true)
      | [ last ] -> Ok last
      | next :: rest ->
        expand rest >>= fun inner -> Ok (Ast.if_ next inner (Some (Ast.bool false)))
    in
    expand operands
  | _ -> Error (Eval_error.Invalid_form "and_to_if: not an and")
;;

(** [or_to_if exp] is the derived-expression rewrite of one [or]; every
    tested operand is evaluated once, behind the temporary binding. *)
let or_to_if exp =
  match Ast.view exp with
  | Ast.Or operands ->
    let temporary = "%or-value" in
    let rec expand = function
      | [] -> Ok (Ast.bool false)
      | [ last ] -> Ok last
      | next :: rest ->
        expand rest
        >>= fun inner ->
        let branch =
          Ast.if_ (Ast.variable temporary) (Ast.variable temporary) (Some inner)
        in
        Ast.lambda [ temporary ] [ branch ]
        >>= fun proc -> Ok (Ast.application proc [ next ])
    in
    expand operands
  | _ -> Error (Eval_error.Invalid_form "or_to_if: not an or")
;;

let run env text =
  Reader.read text
  |> Result.map_error (fun e -> Eval_error.Invalid_form (Reader.to_string e))
  >>= fun exp -> Ev.eval exp env
;;

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [ex_4_04 ()] evaluates one [and] directly and through [and_to_if],
      then one [or] whose later operand would fail if it were
      evaluated, directly and through [or_to_if], then the two empty
      forms at their identity values. *)
let ex_4_04 () =
  let env = SE.the_global_environment () in
  let direct_and = run env "(and 1 2 3)" in
  let derived_and =
    and_to_if (Ast.and_ [ Ast.int 1; Ast.int 2; Ast.int 3 ])
    >>= fun rewritten -> Ev.eval rewritten env
  in
  let direct_or = run env "(or 7 (car '()))" in
  let derived_or =
    or_to_if
      (Ast.or_ [ Ast.int 7; Ast.application (Ast.variable "car") [ Ast.quote Ast.DNil ] ])
    >>= fun rewritten -> Ev.eval rewritten env
  in
  let empty_and = run env "(and)" in
  let empty_or = run env "(or)" in
  List.map render [ direct_and; derived_and; direct_or; derived_or; empty_and; empty_or ]
;;
