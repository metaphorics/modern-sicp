(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program make-rat in SICP section 2.1
   exercise 2.1 *)

(** Exercise 2.1: sign-normalizing, zero-refusing [make_rat]. *)

type rational_error = Zero_denominator of int

val ex_2_01 : int -> int -> (int * int, rational_error) result
