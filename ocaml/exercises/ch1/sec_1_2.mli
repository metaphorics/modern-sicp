(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program ex_1_02 in SICP section 1.1 *)

(** Exercise 1.2, replaced for this edition: the exercise statement's
    fraction,
    [5 + 4 + (2 - (3 - (6 + 4/5)))] over [3 (6 - 2) (2 - 7)], written as
    pure nested calls of [add], [sub], [mul], and [div] over [float].
    The stub raises [Sicp_common.Pending.Pending_solution] until it is
    solved. *)

(** [ex_1_02 ()] is the fraction's value. *)
val ex_1_02 : unit -> float
