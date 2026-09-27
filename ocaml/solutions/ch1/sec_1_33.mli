(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program filtered-accumulate in SICP section
   1.3 exercise 1.33 *)

(** Reference solution of exercise 1.33. *)

val filtered_accumulate
  :  (int -> int -> int)
  -> int
  -> (int -> int)
  -> int
  -> (int -> int)
  -> int
  -> (int -> bool)
  -> int

val is_prime : int -> bool
val gcd : int -> int -> int
val sum_squares_of_primes : int -> int -> int
val product_relatively_prime : int -> int

(** [ex_1_33 ()] is [(1027, 189)]: the primes 2..19 square-summed, and
    [1 * 3 * 7 * 9]. *)
val ex_1_33 : unit -> int * int
