(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.40:
   unique-pairs *)

(** [ex_2_40_unique_pairs n] is the sequence of pairs [i, j] with
    [1 <= j < i <= n]. *)
val ex_2_40_unique_pairs : int -> (int * int) list

(** [prime_sum_pairs] of the text, simplified through
    [ex_2_40_unique_pairs]. *)
val ex_2_40_prime_sum_pairs : int -> (int * int * int) list
