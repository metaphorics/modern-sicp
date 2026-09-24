(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.29 and the tailored addition 4.29a. The evaluator here is
    the section's lazy dispatch with one runtime toggle, [memoized],
    read when an operand is delayed, and three counters over the thunk
    machinery: [creations] counts [delay_it] calls, [forcings] counts
    forces that hit a thunk, and [computations] counts thunk bodies
    actually evaluated. 4.29 reads the object-language [count] the two
    modes produce; 4.29a reads the counters themselves. The counters
    live outside the recursive module, whose initialization must stay
    allocation-free. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2
module Strict_eval = Sicp_ch4.Sec_4_1

(** The memoization toggle: [true] delays with a host [Lazy] cell, the
    section's default; [false] delays without memoization. *)
let memoized = ref true

let creations = ref 0
let forcings = ref 0
let computations = ref 0

(** [reset_counters ()] zeroes the three counters; each demonstration
    starts from a fresh environment and a zeroed count. *)
let reset_counters () =
  creations := 0;
  forcings := 0;
  computations := 0
;;

(** The evaluator under the toggle: the section's clauses with the
    application and [if] clauses routed through the counting [actual_value]
    and the counting [delay]. *)
module rec Counted : sig
  val eval : Lazy_eval.eval_t
  val actual_value : Lazy_eval.eval_t
end = struct
  module C = Lazy_eval.Core (Counted)

  (** [counted_eval] is the thunk body's evaluator: it runs when a thunk
      computes, and that is exactly what [computations] counts. *)
  let counted_eval exp env =
    incr computations;
    Counted.eval exp env
  ;;

  (** [delay exp env] creates one counted thunk under the current
      toggle. *)
  let delay exp env =
    incr creations;
    Lazy_eval.delay_it ~memo:!memoized ~eval:counted_eval exp env
  ;;

  let rec list_of_delayed_args exps env =
    match exps with
    | [] -> Ok []
    | exp :: rest ->
      let first = delay exp env in
      list_of_delayed_args rest env >>= fun values -> Ok (first :: values)
  ;;

  let rec list_of_arg_values exps env =
    match exps with
    | [] -> Ok []
    | exp :: rest ->
      Counted.actual_value exp env
      >>= fun value -> list_of_arg_values rest env >>= fun values -> Ok (value :: values)
  ;;

  let actual_value exp env =
    Counted.eval exp env
    >>= fun v ->
    if Lazy_eval.is_thunk v then incr forcings;
    Lazy_eval.force_value v
  ;;

  let apply_procedure proc operands env =
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name Lazy_eval.primitive_table with
       | Some f -> list_of_arg_values operands env >>= fun args -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure { parameters; body; env = proc_env; _ } ->
      list_of_delayed_args operands env
      >>= fun args ->
      Strict_eval.extend_environment parameters args proc_env
      >>= fun extended -> C.eval_sequence body extended
    | _ -> Error (Eval_error.Not_applicable (Value.to_string proc))
  ;;

  let eval_if exp env =
    match Ast.view exp with
    | Ast.If (predicate, consequent, alternative) ->
      Counted.actual_value predicate env
      >>= fun tested ->
      if Lazy_eval.true_ tested
      then Counted.eval consequent env
      else (
        match alternative with
        | Some branch -> Counted.eval branch env
        | None -> Ok (Value.bool false))
    | _ -> Error (Eval_error.Invalid_form "eval_if: not an if")
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
         Counted.eval e env >>= fun value -> Strict_eval.define_variable_ name value env
       | Ast.Define_function { name; parameters; body } ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
         Strict_eval.define_variable_ name proc env)
    | Ast.Set (name, e) ->
      Counted.eval e env
      >>= fun value ->
      Strict_eval.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
    | Ast.If _ -> eval_if exp env
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> C.eval_sequence body env
    | Ast.Cond _ ->
      Strict_eval.cond_to_if exp >>= fun rewritten -> Counted.eval rewritten env
    | Ast.Application (operator, operands) ->
      Counted.actual_value operator env >>= fun proc -> apply_procedure proc operands env
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

(** [run env text] evaluates [text] with the counting evaluator, the
    driver's forced step. *)
let run env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (Counted.actual_value exp env)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

let counters () =
  Printf.sprintf
    "creations=%d forcings=%d computations=%d"
    !creations
    !forcings
    !computations
;;

let definitions =
  "(define count 0) (define (id x) (set! count (+ count 1)) x) (define (square x) (* x \
   x)) (define (cube x) (* x x x))"
;;

(** [ex_4_29 ()] exhibits the statement's interaction under both modes:
    [(square (id 10))] answers 100 either way, but [count] advances once
    with memoization and twice without; [(cube (id 10))] is the slower
    without memoization, recomputing the same operand three times. *)
let ex_4_29 () =
  let with_memo = Lazy_eval.the_global_environment () in
  let without_memo = Lazy_eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) =
    Lazy_eval.run_program with_memo definitions
  in
  let (_ : (Value.t, Eval_error.t) result) =
    Lazy_eval.run_program without_memo definitions
  in
  memoized := true;
  let f1 = run with_memo "(square (id 10))" in
  let f2 = run with_memo "count" in
  let f3 = run with_memo "(cube (id 10))" in
  let f4 = run with_memo "count" in
  memoized := false;
  let s1 = run without_memo "(square (id 10))" in
  let s2 = run without_memo "count" in
  let s3 = run without_memo "(cube (id 10))" in
  let s4 = run without_memo "count" in
  [ f1; f2; f3; f4; s1; s2; s3; s4 ]
;;

(** [ex_4_29a ()] asserts the counters themselves under each mode: one
    creation either way, two forcings of [square]'s single operand, and
    one computation with memoization against two without. *)
let ex_4_29a () =
  let with_memo = Lazy_eval.the_global_environment () in
  let without_memo = Lazy_eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) =
    Lazy_eval.run_program with_memo definitions
  in
  let (_ : (Value.t, Eval_error.t) result) =
    Lazy_eval.run_program without_memo definitions
  in
  memoized := true;
  reset_counters ();
  let fast_answer = run with_memo "(square (id 10))" in
  let fast_counts = counters () in
  memoized := false;
  reset_counters ();
  let slow_answer = run without_memo "(square (id 10))" in
  let slow_counts = counters () in
  [ fast_answer; fast_counts; slow_answer; slow_counts ]
;;
