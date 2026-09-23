(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program ex_1_03 in SICP section 1.1 *)

(** Reference solution of exercise 1.3. *)

(** [ex_1_03 a b c] is the sum of the squares of the two larger of
    [a], [b], and [c]. Ties for smallest are handled by dropping one
    of the smallest values. *)
val ex_1_03 : int -> int -> int -> int
