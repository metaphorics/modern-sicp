(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fib in SICP section 1.2 exercise 1.13 *)

(** Reference solution of exercise 1.13; the induction proof is in
    [ex_1_13.md]. *)

(** [closed_form_fib n] is @math{\varphi^n - \psi^n} over @math{\sqrt5},
    rounded to the nearest integer. *)
val closed_form_fib : int -> int

(** [fib_direct n] is the tree-recursive [fib] of @ref{1.2.2}. *)
val fib_direct : int -> int

(** [ex_1_13 n] is [closed_form_fib n]; [ex_1_13 10] is 55. *)
val ex_1_13 : int -> int
