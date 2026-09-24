(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.28: forcing the operator. In [(define (run-with op) (op
    2 3))] the variable [op] reaches the operator position bound to a
    thunk -- the delayed operand of a compound call -- so [eval] must
    force it before [apply] dispatches. The demonstration runs the
    program under the section evaluator and under a variant whose
    application clause evaluates the operator with [eval] alone: the
    variant hands the thunk itself to [apply], which can name it but not
    apply it. *)

let ( >>= ) = Result.bind

module Ast = Sicp_common.Ast
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Lazy_eval = Sicp_ch4.Sec_4_2
module Strict_eval = Sicp_ch4.Sec_4_1

(** The variant dispatch: the section's clauses with the application
    clause's operator forcing removed. *)
module rec Unforced : sig
  val eval : Lazy_eval.eval_t

  val apply_procedure
    :  Value.t
    -> Ast.expr list
    -> Value.env
    -> (Value.t, Eval_error.t) result
end = struct
  module C = Lazy_eval.Core (Unforced)

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
      >>= fun extended -> C.eval_sequence body extended
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
         Unforced.eval e env >>= fun value -> Strict_eval.define_variable_ name value env
       | Ast.Define_function { name; parameters; body } ->
         let proc = Value.compound ~name:(Some name) ~parameters ~body ~env in
         Strict_eval.define_variable_ name proc env)
    | Ast.Set (name, e) ->
      Unforced.eval e env
      >>= fun value ->
      Strict_eval.set_variable_value_ name value env >>= fun () -> Ok (Value.symbol "ok")
    | Ast.If _ -> C.eval_if exp env
    | Ast.Lambda (parameters, body) ->
      Ok (Value.compound ~name:None ~parameters ~body ~env)
    | Ast.Sequence body -> C.eval_sequence body env
    | Ast.Cond _ ->
      Strict_eval.cond_to_if exp >>= fun rewritten -> Unforced.eval rewritten env
    | Ast.Application (operator, operands) ->
      Unforced.eval operator env
      >>= fun proc -> Unforced.apply_procedure proc operands env
    | Ast.And _ | Ast.Or _ | Ast.Let _ ->
      Error (Eval_error.Invalid_form "unknown expression type: EVAL")
  ;;
end

let render = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let program = "(define (run-with op) (op 2 3)) (run-with +)"

(** [ex_4_28 ()] runs the same two-form program under the section
    evaluator, which forces the operator thunk and answers 5, and under
    the unforced variant, which hands the thunk to [apply]. *)
let ex_4_28 () =
  let forced = Lazy_eval.the_global_environment () in
  let unforced = Lazy_eval.the_global_environment () in
  match Sicp_common.Reader.read_program program with
  | Error e -> [ "Error: " ^ Sicp_common.Reader.to_string e ]
  | Ok exps ->
    List.concat_map
      (fun exp ->
         [ render (Lazy_eval.eval exp forced); render (Unforced.eval exp unforced) ])
      exps
;;
