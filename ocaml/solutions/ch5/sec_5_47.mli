(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.47: compiled code calling interpreted procedures.

    Every compiled call that is not a saturated primitive goes through
    the runtime's [compiled-apply], which knows primitives and compiled
    procedures only.  The modified runtime adds the third branch: a
    closure made by the evaluator is handed to the evaluator's
    [apply-entry], which saves [continue] and enters [apply-dispatch]
    exactly as an interpreted call would, so the closure's value returns
    to the compiled caller through [continue].  The book routes the
    entry point through a [compapp] register because its compiled code
    is assembled apart from the evaluator; here the runtime is part of
    the evaluator's controller and jumps to the label itself. *)

(** [compound_branch] replaces the runtime's not-applicable signal. *)
val compound_branch : Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** [runtime] is the runtime routines with [compound_branch]. *)
val runtime : Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list

(** [source] defines [apply_twice] and [add6] and applies one to the
    other. *)
val source : string

(** How one top-level item of the session runs. *)
type mode =
  | Compiled
  | Interpreted

(** [session ?runtime modes] runs the items of [source] on the interface
    machine of exercise 5.45 in [modes], item by item, and answers the
    last value. *)
val session
  :  ?runtime:Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction list
  -> mode list
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [ex_5_47 ()] shows the branch and runs the session with [apply_twice]
    compiled and [add6] interpreted, without and with the branch, and
    with every item compiled. *)
val ex_5_47 : unit -> (string list, Sicp_common.Eval_error.t) result
