(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.28: forcing the operator.  In [let run_with op = op 2 3]
    the variable [op] reaches the operator position bound to a thunk,
    the delayed operand of the compound call [run_with add].  The
    section's application clause forces the operator with
    [actual_value] precisely so that [apply] receives a procedure it
    can dispatch on.  The demonstration runs the program under the
    section evaluator and under a variant whose application clause
    evaluates the operator without forcing it: the variant hands the
    thunk itself to [apply], which answers the typed error naming the
    value it was asked to apply. *)

(** [ex_4_28 ()] answers the program's transcript under the section
    evaluator, then under the unforced-operator variant. *)
val ex_4_28 : unit -> string list
