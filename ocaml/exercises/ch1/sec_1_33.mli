(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program filtered-accumulate in SICP section
   1.3 exercise 1.33 *)

(** Exercise 1.33: [filtered_accumulate] adds a filter predicate to
    [accumulate]. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [filtered_accumulate combiner null_value term a next b pred] is
    [accumulate], skipping every index [i] for which [pred i] is
    [false]. *)
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

(** [sum_squares_of_primes a b] is the sum of the squares of the
    primes in [[a, b]]. *)
val sum_squares_of_primes : int -> int -> int

(** [product_relatively_prime n] is the product of every positive
    integer less than [n] whose GCD with [n] is 1. *)
val product_relatively_prime : int -> int

(** [ex_1_33 ()] is [(sum_squares_of_primes 2 20,
    product_relatively_prime 10)]. *)
val ex_1_33 : unit -> int * int
