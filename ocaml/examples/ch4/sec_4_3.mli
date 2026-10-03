(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 4.3 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error

(** The section 4.3 named search experiment over the checked core syntax
    plus the names [amb] and [require] (grammar sections 10 and 13).

    Search order: depth-first over choice points, left alternative
    first.  A branch's observable effects are reversible: the transcript
    carries exactly the printing of successful branches, in answer
    order, and a failed branch leaves nothing behind (its assignments
    and printing are rolled back with it).  Core reference assignment
    keeps its plain semantics; this reversibility is experiment
    behavior only.

    The search restarts: each attempt runs the whole program again
    against a list of decisions, one per choice point, and an attempt
    that reaches a choice point past its decisions extends the search
    with one path per alternative.

    The transcript ends with three named sub-observations:

    - [answers: n] successful branch completions
    - [choices: n] choice points expanded
    - [failures: n] branches that failed *)
module Value = Sicp_common.Value

(** [run ~emit program] evaluates [program] under the search
    experiment.  Each successful branch prints through [emit] in answer
    order; the search sub-observations are appended after the branch
    output. *)
val run : emit:(string -> unit) -> Check.program -> (Value.t, Eval_error.t) result

(** {1 The open search evaluator}

    An exercise adding a special form to the search evaluator is the
    fixed point of its own clauses falling back on [open_eval], run
    through [run_with]. *)

(** One attempt of the search: its remaining decisions, the decisions
    taken so far, and the permanent store it shares with every other
    attempt. *)
type search

(** Why an attempt stopped: its branch failed, it reached a choice point
    past its decisions, or it broke on a runtime error. *)
type stop

type 'a step = ('a, stop) result

(** An evaluator of the search experiment. *)
type eval_t = search -> Env.t -> Ast.expr -> Value.t step

(** [open_eval ~self] is one dispatch step of the search evaluator,
    [amb] and [require] included: it handles the node at hand and
    evaluates every subexpression and every closure body through
    [self]. *)
val open_eval : self:eval_t -> eval_t

(** [eval] is the search evaluator, the fixed point of [open_eval]. *)
val eval : eval_t

(** [choose search n] is the alternative taken at a choice point of [n]
    alternatives, an index from [0] to [n - 1] in try order; the
    driver explores every index, depth first, lowest first.  [n] must be
    positive. *)
val choose : search -> int -> int step

(** [fail] fails the current branch. *)
val fail : 'a step

(** [lift r] is [r] as a step: an error breaks the whole search. *)
val lift : (Value.t, Eval_error.t) result -> Value.t step

(** [permanent_assign search name value] writes [value] into the
    reference bound to the top-level name [name], and the write survives
    backtracking: every later attempt starts that reference at the
    value it survived with.  An assignment on a decision prefix an
    earlier attempt already ran is not applied again.  It answers
    [()]. *)
val permanent_assign : search -> string -> Value.t -> Value.t step

(** [run_with ~eval ~forms source] admits [source] under the search
    experiment with the named [forms] as well (see
    [Check.check_experiment_with]), runs it by the search driver with
    [eval], and answers the transcript: the successful branches' output
    and the sub-observations, then [error: ...] when a runtime error
    stops the search, or [rejected: kind] when admission fails. *)
val run_with : eval:eval_t -> forms:(string * string) list -> string -> string
