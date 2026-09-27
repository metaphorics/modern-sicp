(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sqrt in SICP section 1.1 exercise
   1.7 *)

(** Reference solution of exercise 1.7. *)

(** [ex_1_07_sqrt x] is the section's square root with the fixed
    absolute tolerance [0.001]. *)
val ex_1_07_sqrt : float -> float

(** [ex_1_07_sqrt_improved x] replaces the test with one that stops
    when the guess changes by less than one thousandth of the guess;
    it handles small and large radicands the original mishandles,
    with a guard returning [0.0] for [x = 0.0]. *)
val ex_1_07_sqrt_improved : float -> float
