(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sqrt in SICP section 1.1 exercise
   1.7 *)

(** Exercise 1.7: the tolerance test at the extremes. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [ex_1_07_sqrt x] is the section's square root with the fixed
    absolute tolerance [0.001]. *)
val ex_1_07_sqrt : float -> float

(** [ex_1_07_sqrt_improved x] replaces the test with one that watches
    how much the guess changes and stops when the change is a small
    fraction of the guess. *)
val ex_1_07_sqrt_improved : float -> float
