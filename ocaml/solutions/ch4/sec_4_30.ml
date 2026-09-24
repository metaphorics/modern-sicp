(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.30: forcing in [eval-sequence]. The text's original
    sequence evaluates every expression and forces none of them; Cy
    D. Fect's proposal forces all but the last. The solution runs the
    statement's programs under both. Ben is right about [for-each]:
    the procedure call's effects run because the body of the called
    procedure is a sequence whose [set!] evaluates on the spot and
    because the [display] story flows through primitive application,
    which forces operands. Cy is right about [p2]: under the original
    sequence the [set!] hides in an unforced, unused thunk and never
    runs, so [x] stays 1; under his sequence the thunk is forced and
    [x] becomes [(1 2)]. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2
module Strict_eval = Sicp_ch4.Sec_4_1

(** Cy's evaluator: the section's clauses with his [eval_sequence] --
    every expression but the last is forced. *)
module rec Cy : sig
  val eval : Lazy_eval.eval_t
  val actual_value : Lazy_eval.eval_t
end = struct
  module C = Lazy_eval.Core (Cy)

  let actual_value exp env = Cy.eval exp env >>= Lazy_eval.force_value

  (** [eval_sequence exps env] is Cy's proposal. *)
  let rec eval_sequence exps env =
    match exps with
    | [] -> Error (Eval_error.Invalid_form "the body of the sequence is empty")
    | [ exp ] -> Cy.eval exp env
    | exp :: rest -> Cy.actual_value exp env >>= fun _ -> eval_sequence rest env
  ;;

  let apply_procedure proc operands env =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name Lazy_eval.primitive_table with
       | Some f -> C.list_of_arg_values operands env >>= fun args -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure { parameters; body; env = proc_env; _ } ->
      C.list_of_delayed_args operands env
      >>= fun args ->
      Strict_eval.extend_environment parameters args proc_env
      >>= fun extended -> eval_sequence body extended
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  let eval exp env =
    match Ast.view exp with
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> Strict_eval.lookup_variable_value name env
    | Ast.Quote datum -> Ok (Strict_eval.datum_to_value datum)
    | Ast.Definition d ->
      (match Ast.view_definition d with
       | Ast.Define_variable (name, e) ->
         Cy.eval e env >>= fun value -> Strict_eval.define_variable_ name value env
       | Ast.Define_function { name; parameters; body } ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
         Strict_eval.define_variable_ name proc env)
    | Ast.Set (name, e) ->
      Cy.eval e env
      >>= fun value ->
      Strict_eval.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
    | Ast.If _ -> C.eval_if exp env
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> eval_sequence body env
    | Ast.Cond _ -> Strict_eval.cond_to_if exp >>= fun rewritten -> Cy.eval rewritten env
    | Ast.Application (operator, operands) ->
      Cy.actual_value operator env >>= fun proc -> apply_procedure proc operands env
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [force_with eval exp env] is the driver step: the value, forced. *)
let force_with eval exp env = eval exp env >>= Lazy_eval.force_value

let run eval env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (force_with eval exp env)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

(** [load eval env text] reads a whole program of definitions and
    evaluates them in order under [eval]. *)
let load eval env text =
  match Sicp_common.Reader.read_program text with
  | Ok exps ->
    let (_ : (Value.t, Eval_error.t) result) =
      List.fold_left (fun _ exp -> eval exp env) (Ok (Value.symbol "ok")) exps
    in
    "ok"
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

let for_each =
  "(define (for-each proc items) (if (null? items) 'done (begin (proc (car items)) \
   (for-each proc (cdr items)))))"
;;

let count_up = "(define count 0) (define (bump x) (set! count (+ count 1)) x)"
let p1 = "(define (p1 x) (set! x (cons x '(2))) x)"
let p2 = "(define (p2 x) (define (p e) e x) (p (set! x (cons x '(2)))))"

(** [ex_4_30 ()] answers the statement's parts under both sequences:
    part a, [for-each] runs [bump] three times either way and answers
    [done]; part b, [(p1 1)] is [(1 2)] either way while [(p2 1)] is 1
    under the original and [(1 2)] under Cy's. *)
let ex_4_30 () =
  let original = Lazy_eval.the_global_environment () in
  let cy = Lazy_eval.the_global_environment () in
  let original_eval exp env = Lazy_eval.eval exp env in
  let a1 = load Lazy_eval.eval original (for_each ^ " " ^ count_up) in
  let a2 =
    run original_eval original "(for-each (lambda (x) (bump x)) (list 57 321 88))"
  in
  let a3 = run original_eval original "count" in
  let b1 = load Cy.eval cy (for_each ^ " " ^ count_up) in
  let b2 = run Cy.eval cy "(for-each (lambda (x) (bump x)) (list 57 321 88))" in
  let b3 = run Cy.eval cy "count" in
  let c1 = run original_eval original p1 in
  let c2 = run original_eval original "(p1 1)" in
  let c3 = run original_eval original p2 in
  let c4 = run original_eval original "(p2 1)" in
  let d1 = run Cy.eval cy p1 in
  let d2 = run Cy.eval cy "(p1 1)" in
  let d3 = run Cy.eval cy p2 in
  let d4 = run Cy.eval cy "(p2 1)" in
  [ a1; a2; a3; b1; b2; b3; c1; c2; c3; c4; d1; d2; d3; d4 ]
;;
