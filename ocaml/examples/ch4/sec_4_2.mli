(* SPDX-License-Identifier: GPL-3.0-only *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error

(** The section 4.2 named lazy experiment over the checked core syntax
    (grammar sections 10 and 13).  Compound-procedure arguments and
    [let] right-hand sides become explicit thunk values carrying an
    expression, an environment, and an optional memoized result; the
    experiment also admits the names [delay] and [force].  Primitive
    arguments remain strict.  Forcing memoizes in place; these forcing
    rules are experiment behavior and never change the strict core
    evaluators of [Sec_4_1].

    The transcript ends with four named sub-observations derived from
    the run's explicit thunk state:

    - [thunk-allocations: n] delayed cells created
    - [thunk-forces: n] forcing demands made
    - [thunk-recomputations: n] forces that ran the delayed computation
    - [thunk-memo-hits: n] forces that read a memoized result *)
module Value = Sicp_common.Value

(** The counting state and memo policy of one run. *)
type state

(** The four sub-observation counts of a run so far. *)
type counts =
  { allocations : int
  ; forces : int
  ; recomputations : int
  ; memo_hits : int
  }

(** [state ?memoize ()] is a fresh state with zero counts.  With
    [~memoize:false] a force never records its result, so every force
    of a delayed thunk recomputes it (default [true]). *)
val state : ?memoize:bool -> unit -> state

(** [counts st] reads the counts of [st]. *)
val counts : state -> counts

(** An unforced evaluator: its answer may be a thunk. *)
type eval_t = Ast.expr -> Env.t -> (Value.t, Eval_error.t) result

(** [open_eval ~self st] is one dispatch step of the lazy evaluator:
    subexpressions evaluate through [self], and every strict site
    (conditions, primitive operands, operators, record, reference and
    match scrutinees) forces through [actual_value ~self st]. *)
val open_eval : self:eval_t -> state -> eval_t

(** [actual_value ~self st v] forces [v] under the policy of [st],
    counting the force and its memo hit or recomputation. *)
val actual_value : self:eval_t -> state -> Value.t -> (Value.t, Eval_error.t) result

(** [delay st expr env] is a new thunk of [expr] in [env], counted as
    one allocation. *)
val delay : state -> Ast.expr -> Env.t -> Value.t

(** [apply ~self st env fn args] applies the already-forced operator
    [fn] to the operand expressions [args] of environment [env]:
    closure operands are delayed, primitive operands forced. *)
val apply
  :  self:eval_t
  -> state
  -> Env.t
  -> Value.t
  -> Ast.expr list
  -> (Value.t, Eval_error.t) result

(** [fix open_] ties the knot of an open evaluator. *)
val fix : (self:eval_t -> eval_t) -> eval_t

(** [run_with ~self st ~emit program] runs [program]'s items through
    [self], forcing each binding's value, then appends the four count
    lines of [st]. *)
val run_with
  :  self:eval_t
  -> state
  -> emit:(string -> unit)
  -> Check.program
  -> (Value.t, Eval_error.t) result

(** [run ~emit program] evaluates [program] under the lazy experiment.
    Guest printing writes through [emit]; the thunk sub-observations are
    appended after the guest output.  It is
    [run_with ~self:(fix (fun ~self -> open_eval ~self st)) st] over [st = state ()]. *)
val run : emit:(string -> unit) -> Check.program -> (Value.t, Eval_error.t) result
