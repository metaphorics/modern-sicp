(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.26: [unless] as a special form versus as a procedure. The
    solution fills in both sides of the argument. Ben's side: [unless]
    as a derived expression, rewritten to [if] before evaluation, works
    in the applicative-order evaluator; the rewrite runs through the
    section's [cond]-style lowering in a dispatch of its own. Alyssa's
    side: under the lazy evaluator [unless] stays an ordinary procedure,
    so it can be passed as a value to a higher-order procedure -- the
    demonstration maps it over a list of triples, and the same use goes
    unbound under the special-form evaluator, where [unless] is only
    syntax. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2
module Strict_eval = Sicp_ch4.Sec_4_1

(** [unless_to_if exp] is the derived-expression rewrite: [(unless c u
    e)] becomes [(if c e u)]. *)
let unless_to_if exp =
  match Ast.view exp with
  | Ast.Application (operator, [ condition; usual; exceptional ])
    when Ast.view operator = Ast.Variable "unless" ->
    Ok (Ast.if_ condition exceptional (Some usual))
  | Ast.Application (operator, _) when Ast.view operator = Ast.Variable "unless" ->
    Error (Eval_error.Invalid_form "unless: expects a condition and two values")
  | _ -> Error (Eval_error.Invalid_form "unless_to_if: not an unless")
;;

(** The special-form evaluator: the 4.1 dispatch over the section's
    primitive table, with [unless] lowered to [if] ahead of the
    application clause, so an [unless] written in operand position never
    reaches [apply]. *)
module rec Special : sig
  val eval : Strict_eval.eval_t
end = struct
  module C = Strict_eval.Core (Special)

  let apply_procedure proc args =
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
    | Ast.Application (operator, _) when Ast.view operator = Ast.Variable "unless" ->
      unless_to_if exp >>= fun rewritten -> Special.eval rewritten env
    | Ast.Int n -> Ok (Value.int n)
    | Ast.Float f -> Ok (Value.float f)
    | Ast.Bool b -> Ok (Value.bool b)
    | Ast.String s -> Ok (Value.string s)
    | Ast.Variable name -> Strict_eval.lookup_variable_value name env
    | Ast.Quote datum -> Ok (Strict_eval.datum_to_value datum)
    | Ast.Definition d ->
      (match Ast.view_definition d with
       | Ast.Define_variable (name, e) ->
         Special.eval e env >>= fun value -> Strict_eval.define_variable_ name value env
       | Ast.Define_function { name; parameters; body } ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
         Strict_eval.define_variable_ name proc env)
    | Ast.Set (name, e) ->
      Special.eval e env
      >>= fun value ->
      Strict_eval.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
    | Ast.If (predicate, consequent, alternative) ->
      Special.eval predicate env
      >>= fun tested ->
      if Lazy_eval.true_ tested
      then Special.eval consequent env
      else (
        match alternative with
        | Some branch -> Special.eval branch env
        | None -> Ok (Value.bool false))
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> C.eval_sequence body env
    | Ast.Cond _ ->
      Strict_eval.cond_to_if exp >>= fun rewritten -> Special.eval rewritten env
    | Ast.Application (operator, operands) ->
      Special.eval operator env
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

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let run_special env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (Special.eval exp env)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

let run_lazy env text =
  match Sicp_common.Reader.read text with
  | Ok exp -> render (Lazy_eval.actual_value exp env)
  | Error e -> "Error: " ^ Sicp_common.Reader.to_string e
;;

let unless_procedure =
  "(define (unless condition usual-value exceptional-value) (if condition \
   exceptional-value usual-value))"
;;

let object_map =
  "(define (map f l) (if (null? l) '() (cons (f (car l)) (map f (cdr l)))))"
;;

let triples = "((#t (/ 1 0) 42) (#f 7 (/ 1 0)))"

let mapped_unless =
  "(map (lambda (t) (unless (car t) (car (cdr t)) (car (cdr (cdr t))))) '" ^ triples ^ ")"
;;

let unless_as_value = "((lambda (p) (p #f 1 2)) unless)"

(** [ex_4_26 ()] runs both sides of the debate: the special form works
    under applicative order and delays the unchosen arm; the lazy
    procedure does the same while staying a value that [map] accepts;
    the value use goes unbound where [unless] is only syntax. *)
let ex_4_26 () =
  let special = strict_env () in
  let lazy_env = Lazy_eval.the_global_environment () in
  let lazy_values = Lazy_eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) =
    Lazy_eval.run_program lazy_values (unless_procedure ^ " " ^ object_map)
  in
  let r1 = run_special special "(unless (= 1 1) (/ 1 0) 42)" in
  let r2 = run_lazy lazy_env unless_procedure in
  let r3 = run_lazy lazy_env "(unless (= 1 1) (/ 1 0) 42)" in
  let r4 = run_lazy lazy_values mapped_unless in
  let r5 = run_special special unless_as_value in
  [ r1; r2; r3; r4; r5 ]
;;
