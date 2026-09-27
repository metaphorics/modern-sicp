(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program expmod in SICP section 1.2 exercise
   1.25 *)

(** Reference solution of exercise 1.25; the argument is in
    [ex_1_25.md]. *)

(** [expmod_correct base exp m] reduces modulo [m] at every step. *)
val expmod_correct : int -> int -> int -> int

(** [expmod_naive base exp m] is Alyssa's [fast_expt base exp mod m],
    which wraps for [base] and [exp] this large. *)
val expmod_naive : int -> int -> int -> int

(** [ex_1_25 ()] is [(3, 0)]: the correct remainder of [7 ** 200] modulo
    13 next to Alyssa's wrapped, wrong one. *)
val ex_1_25 : unit -> int * int
