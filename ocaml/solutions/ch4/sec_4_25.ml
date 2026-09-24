(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.25: [unless] breaks under applicative order. Under the
    lazy evaluator the [unless] factorial computes 120 for [n >= 1]
    because the recursive operand stays a thunk until the [if] in
    [unless] chooses; under the applicative-order evaluator the operand
    is evaluated before the call, so the descent never reaches the base
    case. The strict side runs under a fuel-bounded instantiation of the
    4.1 dispatch applied over the section's primitive table, so the
    divergence is observed instead of hung on. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2
module Strict_eval = Sicp_ch4.Sec_4_1

(** The fuel budget of the applicative-order demonstration: every
    application spends one unit, and an exhausted budget answers the
    typed error the demonstration renders. It lives outside the
    recursive module, whose initialization must stay allocation-free. *)
let fuel = ref 0

module rec Strict_fuel : sig
  val eval : Strict_eval.eval_t
end = struct
  module C = Strict_eval.Core (Strict_fuel)

  let spend () =
    incr fuel;
    if !fuel > 200
    then
      Error
        (Eval_error.User_error
           "the strict evaluation is still descending after 200 applications")
    else Ok ()
  ;;

  let apply_procedure proc args =
    spend ()
    >>= fun () ->
    match Value.view proc with
    | Value.Primitive_procedure name ->
      (match List.assoc_opt name Lazy_eval.primitive_table with
       | Some f -> f args
       | None ->
         Error (Eval_error.Invalid_form ("the primitive " ^ name ^ " is not installed")))
    | Value.Compound_procedure cv ->
      Strict_eval.extend_environment cv.parameters args cv.env
      >>= fun extended -> C.eval_sequence cv.body extended
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
         Strict_fuel.eval e env
         >>= fun value -> Strict_eval.define_variable_ name value env
       | Ast.Define_function { name; parameters; body } ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
         Strict_eval.define_variable_ name proc env)
    | Ast.Set (name, e) ->
      Strict_fuel.eval e env
      >>= fun value ->
      Strict_eval.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
    | Ast.If (predicate, consequent, alternative) ->
      Strict_fuel.eval predicate env
      >>= fun tested ->
      if Lazy_eval.true_ tested
      then Strict_fuel.eval consequent env
      else (
        match alternative with
        | Some branch -> Strict_fuel.eval branch env
        | None -> Ok (Value.bool false))
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> C.eval_sequence body env
    | Ast.Cond _ ->
      Strict_eval.cond_to_if exp >>= fun rewritten -> Strict_fuel.eval rewritten env
    | Ast.Application (operator, operands) ->
      Strict_fuel.eval operator env
      >>= fun proc ->
      C.list_of_values operands env >>= fun args -> apply_procedure proc args
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

let strict_env () =
  let env = Strict_eval.setup_environment () in
  List.iter
    (fun (name, f) -> Value.env_define env name (Value.primitive ~name f))
    [ "+", List.assoc "+" Lazy_eval.primitive_table
    ; "-", List.assoc "-" Lazy_eval.primitive_table
    ; "*", List.assoc "*" Lazy_eval.primitive_table
    ; "/", List.assoc "/" Lazy_eval.primitive_table
    ];
  env
;;

let lazy_env () = Lazy_eval.the_global_environment ()

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let run_lazy env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (Lazy_eval.actual_value exp env)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

let run_strict env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (Strict_fuel.eval exp env)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

let unless_definition =
  "(define (unless condition usual-value exceptional-value) (if condition \
   exceptional-value usual-value))"
;;

let factorial_definition =
  "(define (factorial n) (unless (= n 1) (* n (factorial (- n 1))) 1))"
;;

(** [ex_4_25 ()] runs the statement's story: under the lazy evaluator
    the unless factorial answers 120, and the armed [unless] call skips
    its unchosen division; under the applicative-order evaluator the
    same armed call dies in the division and the factorial never
    reaches its base case. *)
let ex_4_25 () =
  let lazy_env = lazy_env () in
  let armed = strict_env () in
  let strict = strict_env () in
  fuel := 0;
  let r1 = run_lazy lazy_env unless_definition in
  let r2 = run_lazy lazy_env factorial_definition in
  let r3 = run_lazy lazy_env "(factorial 5)" in
  let r4 = run_lazy lazy_env "(unless (= 1 1) (/ 1 0) 42)" in
  let r5 = run_strict strict unless_definition in
  let r6 = run_strict strict "(unless (= 1 1) (/ 1 0) 42)" in
  let r7 = run_strict armed unless_definition in
  let r8 = run_strict armed factorial_definition in
  let r9 = run_strict armed "(factorial 5)" in
  [ r1; r2; r3; r4; r5; r6; r7; r8; r9 ]
;;
