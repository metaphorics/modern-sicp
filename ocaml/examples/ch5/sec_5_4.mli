(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 5.4 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Env = Sicp_common.Env
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(** The section 5.4 explicit-control evaluator: the checked host-subset
    syntax executed by a register machine of [Sec_5_1] whose controller
    is a checked instruction list (grammar section 13, row 5.4).

    The machine has the book's registers -- [exp], [env], [val],
    [continue], [proc], [argl], [unev] -- and the simulator's
    save/restore stack; every evaluation decision is an instruction of
    the controller, and the machine operations only inspect syntax,
    build values, and bind environments.  Evaluation order, tail calls
    (a closure body runs without a save), and stack behavior are
    therefore controller facts that the 5.26-5.30 exercises observe and
    extend.  The operations reuse the chapter 4 operator semantics and
    pattern matcher, so the machine computes what the evaluators of
    [Sicp_ch4.Sec_4_1] compute. *)

(** One machine word. *)
type word =
  | V of Value.t (** A guest value. *)
  | Exp of Ast.expr
  | Exps of Ast.expr list (** The [unev] register's pending operands. *)
  | Args of Value.t list
  (** The [argl] register's evaluated operands, in evaluation order. *)
  | Env of Env.t
  | Lab of string
  | Cases of (Ast.pattern * Ast.expr) list
  | Pat of Ast.pattern (** A compiled [match] case's pattern constant. *)
  | Pending of Env.t * (Ast.binding * Env.cell option) list
  (** A recursive group: the environment binding its cells, and the
      right-hand sides still to run, each with the cell its value
      fills. *)
  | Unassigned

(** [word_to_string w] renders [w] for traces and tests. *)
val word_to_string : word -> string

(** [words] is how the evaluator machine handles its words. *)
val words : word Sec_5_1.words

(** [operation_table ~apply] is the base operation table.  [apply] runs
    a guest procedure for a primitive that calls one (the [List]
    members). *)
val operation_table : apply:Value.apply_fun -> (string * word Sec_5_1.op) list

(** The evaluator's registers. *)
val evaluator_registers : string list

(** The controller fragments in the book's order, each with its block
    name: [driver], [eval-dispatch], [ev-simple], [ev-if], [ev-match],
    [ev-let], [ev-sequence], [ev-logic], [ev-unary], [ev-binary],
    [ev-collect], [ev-application], [apply-dispatch].  An exercise
    composes a variant by keeping, replacing, or adding fragments. *)
val controller_fragments : (string * word Sec_5_1.instruction list) list

(** [base_controller] is every fragment of [controller_fragments]
    concatenated. *)
val base_controller : word Sec_5_1.instruction list

(** [base_operation_names] is every operation the base controller
    names; [make_evaluator] installs them. *)
val base_operation_names : string list

(** One evaluator: its machine and the global environment it runs
    top-level items in. *)
type evaluator

(** [make_evaluator ?operations ~controller ~emit ()] assembles
    [controller] over the base operation table extended by
    [operations], whose entries take precedence, with [registers]
    declared after [evaluator_registers].  Guest printing writes through
    [emit].  The controller must define [eval-dispatch], [apply-dispatch],
    and [done]; the driver enters at [eval-dispatch] with [continue]
    holding [done]. *)
val make_evaluator
  :  ?operations:(string * word Sec_5_1.op) list
  -> ?registers:string list
  -> controller:word Sec_5_1.instruction list
  -> emit:(string -> unit)
  -> unit
  -> (evaluator, Eval_error.t) result

(** [machine ev] is the evaluator's machine. *)
val machine : evaluator -> word Sec_5_1.machine

(** [eval ev env e] runs the controller on [e] in [env]. *)
val eval : evaluator -> Env.t -> Ast.expr -> (Value.t, Eval_error.t) result

(** [run_program ev program] runs every top-level item of [program] in
    source order, answering the last value bound. *)
val run_program : evaluator -> Check.program -> (Value.t, Eval_error.t) result

(** [run ~emit program] executes [program] on the base evaluator. *)
val run : emit:(string -> unit) -> Check.program -> (Value.t, Eval_error.t) result

(** [stack_statistics_after program] runs [program] on the base
    evaluator and answers the total pushes and maximum depth of its last
    top-level evaluation (the 5.4.4 monitored-stack lesson). *)
val stack_statistics_after : Check.program -> (int * int, Eval_error.t) result
