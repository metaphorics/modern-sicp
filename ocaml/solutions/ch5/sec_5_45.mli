(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.45: stack use of compiled, interpreted, and
    special-purpose factorial code; and the interface of section 5.5.7
    that the exercises 5.45 to 5.49 run on.

    The interface loads compiled code into the explicit-control
    evaluator's machine: the controller is the evaluator's fragments,
    then labelled blocks of compiled code, each ending at [done], then
    the compiler's shared runtime routines.  The evaluator's
    [apply-dispatch] gains a branch that hands a compiled procedure to
    the runtime's [compiled-apply] with [continue] restored, so
    interpreted code calls compiled procedures.  Top-level items run
    either as a compiled block or on the evaluator, threading one global
    environment.

    The three factorials run under the same monitored stack of 5.2.4:
    the evaluator of 5.4, the compiled code of 5.5 in the interface
    machine, and the machine of Figure 5.11.  The interpreter pays about
    twenty pushes per level; each pending level of the compiled code
    holds [continue] for the call's return linkage and [env] to find
    [n] after the call returns, against the special-purpose machine's
    [continue] and [n].  Both compiled measurements include the top-level
    call item's own [env] and [argl] saves, so the compiled pushes are
    [2n] against [2n - 2] (10/10 against 8/8 at [n] = 5) and both ratios
    settle near a tenth.  Open coding of the arithmetic is what brings
    the compiled code this close; calling a known procedure's entry
    directly, as exercise 5.33a does by hand, would remove the rest of
    its call overhead, which shows in steps rather than in pushes. *)

(** {1 The interface} *)

(** [runtime] is the compiler's shared runtime routines. *)
val runtime : Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** [block_statements seq] is the code of one [compile_program] result
    without its shared runtime and final jump to [done]. *)
val block_statements
  :  Sicp_ch5.Sec_5_5.seq
  -> Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** [apply_dispatch] is the evaluator's [apply-dispatch] with the branch
    for compiled procedures. *)
val apply_dispatch : Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** [controller ?runtime blocks] is the interface controller holding the
    labelled [blocks]; [runtime] replaces the shared routines. *)
val controller
  :  ?runtime:Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list
  -> (string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list) list
  -> Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** [compiled_operations] is the compiled-procedure operations the
    evaluator's table lacks. *)
val compiled_operations : (string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.op) list

(** [make_evaluator ?runtime ~emit blocks] is the interface machine
    holding [blocks]; guest printing writes through [emit]. *)
val make_evaluator
  :  ?runtime:Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list
  -> emit:(string -> unit)
  -> (string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list) list
  -> (Sicp_ch5.Sec_5_4.evaluator, Sicp_common.Eval_error.t) result

(** [run_block ev env label] runs the compiled block [label] with [env]
    as the global environment, from a restarted machine, and answers the
    value it leaves and the environment its bindings extend. *)
val run_block
  :  Sicp_ch5.Sec_5_4.evaluator
  -> Sicp_common.Env.t
  -> string
  -> (Sicp_common.Value.t * Sicp_common.Env.t, Sicp_common.Eval_error.t) result

(** [eval_item ev env item] runs the top-level [item] on the evaluator
    in [env] and answers its last value and the extended environment. *)
val eval_item
  :  Sicp_ch5.Sec_5_4.evaluator
  -> Sicp_common.Env.t
  -> Sicp_common.Ast.item
  -> (Sicp_common.Value.t * Sicp_common.Env.t, Sicp_common.Eval_error.t) result

(** [global_environment ~emit] is a fresh global environment. *)
val global_environment : emit:(string -> unit) -> Sicp_common.Env.t

(** {1 The measurements} *)

(** [interpreted source] is the total pushes and maximum depth of the
    last top-level evaluation of [source] on the evaluator. *)
val interpreted : string -> (int * int, Sicp_common.Eval_error.t) result

(** [compiled source] is the same measurement for [source] compiled item
    by item and run on the interface machine. *)
val compiled : string -> (int * int, Sicp_common.Eval_error.t) result

(** [special controller registers n] runs the special-purpose machine
    [controller] with [n] in register [n] and answers its stack
    statistics. *)
val special
  :  Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
  -> string list
  -> int
  -> (int * int, Sicp_common.Eval_error.t) result

(** [factorial_machine] is the recursive factorial machine of Figure
    5.11. *)
val factorial_machine : Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list

(** [comparison ~definition ~call ~machine n] measures the three
    versions of [call n] and renders the pushes, depths, and their ratios
    to the interpreted ones. *)
val comparison
  :  definition:string
  -> call:string
  -> machine:Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
  -> int
  -> (string, Sicp_common.Eval_error.t) result

(** [ex_5_45 ()] is the factorial comparison at [n] = 5 and 10. *)
val ex_5_45 : unit -> (string list, Sicp_common.Eval_error.t) result
