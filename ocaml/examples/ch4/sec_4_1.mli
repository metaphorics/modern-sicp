(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.1 *)

(** The metacircular evaluator of section 4.1 against the shared
    substrate: the typed [Ast] is the syntax, [Value] the runtime data,
    [Env] the environments, and every failure travels through
    [Eval_error]. [Core] is the reusable standard dispatch; [Analyze] is
    the analyzed evaluator of 4.1.7. *)

(** One evaluator: from an expression and an environment to a value or a
    typed error. *)
type eval_t =
  Sicp_common.Ast.expr
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [true_ v] holds for every value except the false object. *)
val true_ : Sicp_common.Value.t -> bool

(** [false_ v] holds exactly for the false object. *)
val false_ : Sicp_common.Value.t -> bool

(** [datum_to_value d] is the runtime value of the quoted datum [d]. *)
val datum_to_value : Sicp_common.Ast.datum -> Sicp_common.Value.t

(** The marker value of a scanned-out internal definition that has not
    been assigned yet (4.16). *)
val unassigned : Sicp_common.Value.t

(** [is_unassigned v] holds exactly for [unassigned]. *)
val is_unassigned : Sicp_common.Value.t -> bool

(** {2 4.1.3: the environment operations} *)

(** [lookup_variable_value name env] is the value of the nearest binding
    of [name], or [Error (Unbound_variable name)]. *)
val lookup_variable_value
  :  string
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [extend_environment names values base_env] is a fresh frame binding
    each name to the value at the same position, in front of
    [base_env]. *)
val extend_environment
  :  string list
  -> Sicp_common.Value.t list
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.env, Sicp_common.Eval_error.t) result

(** [set_variable_value_ name value env] rebinds the nearest binding of
    [name]. *)
val set_variable_value_
  :  string
  -> Sicp_common.Value.t
  -> Sicp_common.Value.env
  -> (unit, Sicp_common.Eval_error.t) result

(** [define_variable_ name value env] binds [name] in the newest frame
    and answers the symbol [ok]. *)
val define_variable_
  :  string
  -> Sicp_common.Value.t
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [primitive_table env] holds the section's primitives under their
    object-language names. *)
val primitive_table : (string * Sicp_common.Value.primitive) list

(** [setup_environment ()] is a fresh global environment with the
    primitives and the bindings of [true] and [false]. *)
val setup_environment : unit -> Sicp_common.Value.env

(** [the_global_environment ()] is a fresh global environment, the
    book's [the-global-environment]. *)
val the_global_environment : unit -> Sicp_common.Value.env

(** [sequence_to_exp exps] packs a clause body into one expression. *)
val sequence_to_exp
  :  Sicp_common.Ast.expr list
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** [cond_to_if exp] is the derived-expression rewrite of one [cond]. *)
val cond_to_if
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** {2 4.1.1: the core of the evaluator} *)

(** [Core (Eval)] is the standard dispatch of the metacircular
    evaluator, recursing through [Eval.eval]. An exercise that adds a
    clause to [eval] instantiates [Core] with its own recursive module;
    the base evaluator is [Core] instantiated with itself. The result
    signature cannot mention the parameter, so the functor-parameter
    warning is silenced here; the implementation uses it throughout. *)
module Core (Eval : sig
    (** The recursive evaluator the standard clauses call back into. *)
    val eval : eval_t
  end) : sig
  (** [list_of_values exps env] evaluates the operands left to right. *)
  val list_of_values
    :  Sicp_common.Ast.expr list
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t list, Sicp_common.Eval_error.t) result

  (** [list_of_values_right_to_left exps env] evaluates the same
      operands right to left (exercise 4.1). *)
  val list_of_values_right_to_left
    :  Sicp_common.Ast.expr list
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t list, Sicp_common.Eval_error.t) result

  (** [eval_sequence exps env] evaluates a body or [begin] in order,
      answering the last value. *)
  val eval_sequence
    :  Sicp_common.Ast.expr list
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [apply_procedure proc args] is the book's [apply]. *)
  val apply_procedure
    :  Sicp_common.Value.t
    -> Sicp_common.Value.t list
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [eval_if exp env] evaluates one [if] under the object language's
      truth. *)
  val eval_if
    :  Sicp_common.Ast.expr
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [eval_assignment name exp env] rebinds [name] and answers [ok]. *)
  val eval_assignment
    :  string
    -> Sicp_common.Ast.expr
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [eval_definition d env] installs one definition and answers [ok]. *)
  val eval_definition
    :  Sicp_common.Ast.definition
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [eval exp env] is the standard dispatch: every syntactic type of
      the section except [and], [or], and [let], which the exercises
      add. *)
  val eval : eval_t
end
[@@warning "-67"]

(** [eval exp env] evaluates one expression, the base instantiation of
    [Core]. *)
val eval : eval_t

(** {2 4.1.4: the driver} *)

(** [run env text] reads one object-language form from [text] and
    evaluates it in [env]. *)
val run
  :  Sicp_common.Value.env
  -> string
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [run_program env text] reads a program of forms and evaluates them
    in order, answering the last value. *)
val run_program
  :  Sicp_common.Value.env
  -> string
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** The analyzed evaluator of 4.1.7. *)
module Analyze : sig
  (** One execution procedure: the analyzed form of one expression, an
      environment to result closure with the dispatch already decided. *)
  type execution =
    Sicp_common.Value.env -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [analyze exp] compiles [exp] once into its execution procedure. *)
  val analyze : Sicp_common.Ast.expr -> (execution, Sicp_common.Eval_error.t) result

  (** [eval exp env] analyzes [exp] and runs the result in [env]. *)
  val eval : eval_t
end
