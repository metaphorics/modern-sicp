(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 4.3 *)

(** The nondeterministic evaluator of section 4.3 against the shared
    substrate: the typed [Ast] is the syntax, [Value] the runtime data,
    [Env] the environments, and every object-language failure travels
    through [Eval_error]. The search machinery is OCaml 5 effect
    handlers: execution procedures are continuation-passing steps whose
    success path stays nested in the caller's stack, a dead end performs
    the [Fail] effect and unwinds to the innermost branch handler (an
    assignment's undo trail on the way, then the most recent choice
    point), [amb] performs the [Amb] effect carrying its analyzed
    alternatives to the [choice] handler, and a top-level answer
    performs [Answer] at the driver's sink, which suspends the search on
    the perform's one-shot continuation so [try_again] yields exactly
    one further answer per capture. *)

(** One evaluator: from an expression and an environment to a value or a
    typed error, as in 4.1. *)
type eval_t = Sec_4_1.eval_t

(** [true_ v] holds for every value except the false object. *)
val true_ : Sicp_common.Value.t -> bool

(** [false_ v] holds exactly for the false object. *)
val false_ : Sicp_common.Value.t -> bool

(** {2 The search engine}

    The section's evaluation is a protocol of three effects over
    continuation-passing execution procedures. *)

(** One execution procedure: the environment and the success
    continuation; a failure is performed, not called. The whole future
    use of the answer stays nested inside the success call, which is
    what lets a later [Fail] unwind into the handlers installed for the
    branch. *)
type exec = Sicp_common.Value.env -> (Sicp_common.Value.t -> unit) -> unit

(** One continuation-passing evaluation step, the shape every clause of
    the evaluator recurses through. *)
type eval_k =
  Sicp_common.Ast.expr -> Sicp_common.Value.env -> (Sicp_common.Value.t -> unit) -> unit

(** [Fail] is a dead end: the branch aborts and unwinds to the innermost
    handler installed for it -- an assignment trail interceptor on the
    way, then the most recent choice point, or the driver when no choice
    is pending. *)
type _ Effect.t += Fail : unit Effect.t

(** [Amb alternatives] is one choice point: the [amb] form performs it
    carrying the execution procedures of its alternatives, and the
    [choice] handler runs them in order, advancing whenever the current
    one fails. *)
type _ Effect.t += Amb : exec list -> unit Effect.t

(** [Answer v] is the driver sink: a top-level success performs it, and
    the driver suspends the search on the continuation of that perform
    so [try_again] can demand the next answer. *)
type _ Effect.t += Answer : Sicp_common.Value.t -> unit Effect.t

(** [Raised e] carries the typed object-language error across the
    continuation-passing steps; [drive] reports it as [Error e]. An
    error aborts the whole search: it is a program bug, not a dead end
    to backtrack out of. *)
exception Raised of Sicp_common.Eval_error.t

(** [choice alternatives] is the execution procedure of one choice
    point, the handler of the [Amb] effect: it runs the alternatives in
    order under the success continuation in hand, counting one backtrack
    for every [Fail] it intercepts (4.44a) and performing [Fail] onward
    when the list is exhausted. An exercise that adds a search strategy
    -- exercise 4.50's [ramb] -- builds its execution procedure by
    reordering the alternatives and handing them to [choice]. *)
val choice : exec list -> exec

(** [report v] is the driver's success continuation: it performs
    [Answer v] at the sink. *)
val report : Sicp_common.Value.t -> unit

(** [drive go] runs one search to its first answer or its exhaustion:
    [Ok] of the value that reached the sink, the typed exhaustion error
    when a [Fail] reaches the driver, or the typed [Raised] error. A
    suspended search is kept as the driver state [try_again] resumes. *)
val drive : (unit -> unit) -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** {2 4.1.3: the environment operations}

    The same operations 4.1 publishes, for an exercise that instantiates
    [Core] with its own dispatch. *)

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
    object-language names: the 4.1 entries, then the additions the
    section's programs use -- [>=] and [<=], [abs], [min], [max],
    [even?], [odd?], [member] (suffix from the match, [equal?]
    comparison), [append], and [sqrt] with [integer?], where a perfect
    square answers an exact integer. *)
val primitive_table : (string * Sicp_common.Value.primitive) list

(** [setup_environment ()] is a fresh global environment with the
    primitives and the bindings of [true] and [false]. *)
val setup_environment : unit -> Sicp_common.Value.env

(** [the_global_environment ()] is a fresh global environment, the
    book's [the-global-environment]. *)
val the_global_environment : unit -> Sicp_common.Value.env

(** [cond_to_if exp] is the derived-expression rewrite of one [cond]. *)
val cond_to_if
  :  Sicp_common.Ast.expr
  -> (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result

(** {2 4.3.3: the evaluator clauses}

    [Core (Eval)] is the section's dispatch in continuation style,
    parameterized by the [eval_k] it recurses through. An exercise that
    adds a clause -- 4.50's [ramb], 4.54's [require] -- instantiates
    [Core] with its own recursive module, exactly as in 4.1. *)
module Core (Eval : sig
    (** The recursive continuation-passing evaluator the clauses call
        back into. *)
    val eval_k : eval_k
  end) : sig
  (** [list_of_values exps env k] evaluates the operands left to right
      (exercise 4.46). *)
  val list_of_values
    :  Sicp_common.Ast.expr list
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t list -> unit)
    -> unit

  (** [eval_sequence exps env k] evaluates a body or [begin] in order
      under the continuation [k]. *)
  val eval_sequence
    :  Sicp_common.Ast.expr list
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t -> unit)
    -> unit

  (** [apply_procedure proc args k] is the book's [apply]. *)
  val apply_procedure
    :  Sicp_common.Value.t
    -> Sicp_common.Value.t list
    -> (Sicp_common.Value.t -> unit)
    -> unit

  (** [eval_if exp env k] evaluates one [if] under the object
      language's truth. *)
  val eval_if
    :  Sicp_common.Ast.expr
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t -> unit)
    -> unit

  (** [eval_assignment name exp env k] is [set!] with the undo trail:
      the old value is restored before a later failure propagates. *)
  val eval_assignment
    :  string
    -> Sicp_common.Ast.expr
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t -> unit)
    -> unit

  (** [eval_permanent name exp env k] is 4.51's [permanent-set!]: the
      assignment survives backtracking. *)
  val eval_permanent
    :  string
    -> Sicp_common.Ast.expr
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t -> unit)
    -> unit

  (** [eval_if_fail e1 e2 env k] is 4.52's [if-fail]: the first failure
      of [e1] is caught once and [e2] succeeds in its place. *)
  val eval_if_fail
    :  Sicp_common.Ast.expr
    -> Sicp_common.Ast.expr
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t -> unit)
    -> unit

  (** [eval_definition d env k] installs one definition and answers
      [ok]. *)
  val eval_definition
    :  Sicp_common.Ast.definition
    -> Sicp_common.Value.env
    -> (Sicp_common.Value.t -> unit)
    -> unit

  (** [eval_k exp env k] is the dispatch: the 4.1 clauses in
      continuation style, [let] as a derived form, and the [amb],
      [permanent-set!], and [if-fail] special forms recognized at
      application heads by their reserved names, because the shared
      grammar gains no node kinds. *)
  val eval_k : eval_k
end
[@@warning "-67"]

(** [eval exp env] evaluates one expression as one problem: the value of
    its first non-failing execution, or the typed exhaustion error when
    every execution fails. *)
val eval : eval_t

(** [backtrack_count ()] is the number of backtracks since the last
    [reset_backtrack_count ()]: deliveries of [Fail] to choice-point
    handlers, the quantity exercise 4.44a asserts. *)
val backtrack_count : unit -> int

(** [reset_backtrack_count ()] zeroes the backtrack counter. *)
val reset_backtrack_count : unit -> unit

(** {2 The driver and the try-again protocol} *)

(** [run env text] reads one object-language form from [text] and
    evaluates it as a new problem in [env]; any pending search is
    discarded, as in the book's driver loop. *)
val run
  :  Sicp_common.Value.env
  -> string
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [run_program env text] reads a whole program of forms and evaluates
    them in order, each as its own problem; the value of the last form
    is the program's value and its search state is what [try_again]
    continues. *)
val run_program
  :  Sicp_common.Value.env
  -> string
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [try_again ()] resumes the suspended search once and answers the
    next value, or the typed exhaustion error when the choices are
    spent, or ``there is no current problem'' when no search is
    pending. *)
val try_again : unit -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result
