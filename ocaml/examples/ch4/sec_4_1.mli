(* SPDX-License-Identifier: GPL-3.0-only *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error

(** The section 4.1 teaching evaluators over the checked syntax: the
    direct evaluator and the analyzed evaluator, which separate syntax
    analysis from execution and analyze each syntax node once per run.

    Both evaluators share one semantic core -- lexical environments,
    left-to-right operand evaluation, curried application, pattern
    matching, and the fixed primitive surface -- and differ only in how
    a closure body is executed: directly, or through its analysis.

    Neither evaluator ever calls the host language on guest programs;
    the only host computation is the primitive surface of [Prelude]. *)
module Value = Sicp_common.Value

(** [run ~emit program] evaluates [program] directly.  Printing
    primitives write their bytes through [emit]. *)
val run : emit:(string -> unit) -> Check.program -> (Value.t, Eval_error.t) result

(** [run_analyzed ~emit program] evaluates [program] through syntax
    analysis: each expression is analyzed once, then execution is
    environment application. *)
val run_analyzed
  :  emit:(string -> unit)
  -> Check.program
  -> (Value.t, Eval_error.t) result

(** [bind_pattern pattern value] is the bindings of [pattern] against
    [value], or [None] when the pattern does not match.  Constructor
    payloads align under the grammar's tuple-payload rule: a single
    variable payload binds the whole payload tuple and a multi-pattern
    payload destructures it. *)
val bind_pattern : Ast.pattern -> Value.t -> (string * Value.t) list option

(** [bind_pattern_with ~force pattern value] is [bind_pattern] for an
    evaluator whose values may be delayed: [force] runs on each subject
    whose shape the pattern tests (a scalar, tuple, constructor, [[]]
    or [::] pattern) before the test, while variables and wildcards
    bind their subject unforced.  An error from [force] stops the
    match. *)
val bind_pattern_with
  :  force:(Value.t -> (Value.t, Eval_error.t) result)
  -> Ast.pattern
  -> Value.t
  -> ((string * Value.t) list option, Eval_error.t) result

(** {1 Operator semantics}

    The scalar and operator meanings every teaching engine shares, so the
    search experiment, the explicit-control evaluator, and the compiled
    machine compute exactly what the evaluators here compute. *)

(** [scalar_value s] is the runtime value of the literal [s]. *)
val scalar_value : Ast.scalar -> Value.t

(** [arithmetic op left right] applies [op] under grammar section 6:
    target-width integer wrapping, truncating division, remainder with
    the sign of its left operand, and a division-by-zero failure. *)
val arithmetic : Ast.arith -> Value.t -> Value.t -> (Value.t, Eval_error.t) result

(** [negate v] is the integer or float negation of [v]. *)
val negate : Value.t -> (Value.t, Eval_error.t) result

(** [comparison op left right] applies [op] under the scalar comparison
    rules of grammar section 3. *)
val comparison : Ast.comparison -> Value.t -> Value.t -> (Value.t, Eval_error.t) result

(** {1 The evaluators over one expression}

    The entry points the exercises extend: one expression evaluated in
    an environment, rather than a whole checked unit. *)

(** An evaluator of one expression in an environment. *)
type eval_t = Ast.expr -> Sicp_common.Env.t -> (Value.t, Eval_error.t) result

(** [eval_expr] is the direct evaluator. *)
val eval_expr : eval_t

(** [analyze e] analyzes [e] once and answers its execution procedure. *)
val analyze : Ast.expr -> Sicp_common.Env.t -> (Value.t, Eval_error.t) result

(** [apply_procedure proc args] applies a closure or primitive to
    [args] under the direct evaluator. *)
val apply_procedure : Value.t -> Value.t list -> (Value.t, Eval_error.t) result

(** [the_global_environment ?emit ()] is the prelude environment;
    printing goes through [emit] (default: standard output). *)
val the_global_environment : ?emit:(string -> unit) -> unit -> Sicp_common.Env.t

(** [expression source] admits the single OCaml expression [source]
    (checked as the unit [let it = (source)]) and answers its syntax. *)
val expression : string -> (Ast.expr, string) result

(** [transcript ?experiment run source] admits [source] and runs it with
    [run]: the guest output, then [error: ...] when the run stops on a
    runtime error, or [rejected: kind] when admission fails. *)
val transcript
  :  ?experiment:Check.experiment
  -> (emit:(string -> unit) -> Check.program -> (Value.t, Eval_error.t) result)
  -> string
  -> string

(** [open_eval ~self] is one step of the direct evaluator's dispatch: it
    handles the node at hand and evaluates every subexpression and
    every closure body through [self].  An exercise's evaluator is the
    fixed point of its own clauses falling back on [open_eval]. *)
val open_eval : self:eval_t -> eval_t

(** [apply_with ~self proc args] applies [proc], running closure bodies
    through [self]. *)
val apply_with : self:eval_t -> Value.t -> Value.t list -> (Value.t, Eval_error.t) result
