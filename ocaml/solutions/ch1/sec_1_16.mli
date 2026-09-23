(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fast-expt in SICP section 1.2
   exercise 1.16 *)

(** Reference solution of exercise 1.16. *)

(** [fast_expt_iter a b n] holds the invariant [a * b^n] and reaches
    [a * b^n] with [n = 0], answering [a], in a logarithmic number of
    steps. *)
val fast_expt_iter : int -> int -> int -> int

(** [ex_1_16 b n] is [b ** n]; [ex_1_16 2 10] is 1024. *)
val ex_1_16 : int -> int -> int
