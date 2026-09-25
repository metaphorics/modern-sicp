(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.2 *)

(** The lazy evaluator of section 4.2 against the 4.1 substrate:
    compound-procedure arguments are delayed into thunks, primitives
    stay strict, quoted data stays ordinary data, and the driver forces
    before printing. The section's contract adds the thunk machinery and
    the forcing step to the fixed shape of 4.1; every failure still
    travels through [Eval_error]. *)

(** One evaluator: from an expression and an environment to a value or a
    typed error, as in 4.1. *)
type eval_t = Sec_4_1.eval_t

(** [true_ v] holds for every value except the false object. *)
val true_ : Sicp_common.Value.t -> bool

(** [primitive_table] is the section's table: the 4.1 entries with the
    section's arithmetic in front of them -- [+], [-], and [*] over exact
    integers or inexact floats, and [/] with its typed zero-divisor
    error. Application resolves primitives here. *)
val primitive_table : (string * Sicp_common.Value.primitive) list

(** [setup_environment ()] is a fresh global environment with the
    section's primitive table and the bindings of [true] and [false]. *)
val setup_environment : unit -> Sicp_common.Value.env

(** {2 4.2.2: representing thunks} *)

(** [thunk_tag] is the reserved primitive name marking a thunk value; no
    primitive table installs it. *)
val thunk_tag : string

(** [is_thunk v] holds exactly for the thunk values this evaluator
    creates. *)
val is_thunk : Sicp_common.Value.t -> bool

(** [thunk_of force] is the thunk whose forcing runs [force ()];
    [force_value] keeps forcing until a non-thunk value comes back. *)
val thunk_of
  :  (unit -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result)
  -> Sicp_common.Value.t

(** [force_value v] is the book's [force-it]: a thunk answers its value,
    forcing recursively; any other value answers unchanged. A memoized
    thunk computes at most once, and a failure answered while the memo
    fills is cached and re-answered on every later forcing. *)
val force_value
  :  Sicp_common.Value.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [delay_it ~memo ~eval exp env] is the book's [delay-it], the thunk
    of [exp] in [env]. With [memo = true], the default, the thunk
    memoizes through a host [Lazy] cell; with [memo = false] every
    forcing re-evaluates [exp]. The thunk body runs under [~eval]. *)
val delay_it
  :  ?memo:bool
  -> eval:eval_t
  -> Sicp_common.Ast.expr
  -> Sicp_common.Value.env
  -> Sicp_common.Value.t

(** {2 4.2.2: the evaluator changes} *)

(** [Core (Eval)] is the lazy dispatch: the 4.1 clauses recursing through
    [Eval.eval], with the application clause forcing the operator and
    delaying the operands and the [if] clause forcing its predicate. *)
module Core (Eval : sig
    (** The recursive evaluator the standard clauses call back into. *)
    val eval : eval_t
  end) : sig
  (** [actual_value exp env] is the book's [actual-value]: [eval]
      followed by [force_value]. *)
  val actual_value
    :  Sicp_common.Ast.expr
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [list_of_arg_values exps env] forces the operands left to right. *)
  val list_of_arg_values
    :  Sicp_common.Ast.expr list
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t list, Sicp_common.Eval_error.t) result

  (** [list_of_delayed_args exps env] delays the operands left to
      right. *)
  val list_of_delayed_args
    :  Sicp_common.Ast.expr list
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t list, Sicp_common.Eval_error.t) result

  (** [eval_sequence exps env] evaluates a body or [begin] in order,
      answering the last value; sequence positions are evaluated, not
      forced (exercise 4.30). *)
  val eval_sequence
    :  Sicp_common.Ast.expr list
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [apply_procedure proc operands env] is the book's lazy [apply]:
      the operands arrive unevaluated, primitives force them, compound
      procedures delay them. *)
  val apply_procedure
    :  Sicp_common.Value.t
    -> Sicp_common.Ast.expr list
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [eval_if exp env] evaluates one [if] with its predicate forced. *)
  val eval_if
    :  Sicp_common.Ast.expr
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

  (** [eval exp env] is the lazy dispatch: every syntactic type of the
      section except [and], [or], and [let]. *)
  val eval : eval_t
end
[@@warning "-67"]

(** [eval exp env] evaluates one expression in one environment under the
    lazy language, the base instantiation of [Core]. *)
val eval : eval_t

(** [actual_value exp env] is the driver's evaluation step: the value of
    [exp], forced if it is a thunk. *)
val actual_value
  :  Sicp_common.Ast.expr
  -> Sicp_common.Value.env
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** {2 4.2.2: the driver} *)

(** [run env text] reads one object-language form from [text] and
    evaluates it with [actual_value] in [env]: a delayed value is forced
    before it reaches the surface. *)
val run
  :  Sicp_common.Value.env
  -> string
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [run_program env text] reads a whole program of forms and evaluates
    them in order, forcing the value of the last one. *)
val run_program
  :  Sicp_common.Value.env
  -> string
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [the_global_environment ()] is a fresh global environment with the
    section's primitive table and the bindings of [true] and [false]. *)
val the_global_environment : unit -> Sicp_common.Value.env

(** {2 4.2.1: the applicative-order contrast} *)

(** [Strict_eval] is the applicative-order evaluator of 4.1 applied
    over the section's primitive table: 4.1's [Core] dispatch with the
    application clause evaluating the operands before the call and
    resolving the operator against [primitive_table], the section's
    table. A division in an unchosen branch is a returned [Error] under
    [Strict_eval]
    where the lazy driver would answer. *)
module Strict_eval : sig
  (** [eval exp env] evaluates one expression in one environment under
      the applicative-order dispatch. *)
  val eval : eval_t

  (** [run env text] reads one object-language form from [text] and
      evaluates it in [env]. *)
  val run
    :  Sicp_common.Value.env
    -> string
    -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result
end
