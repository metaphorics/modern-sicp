(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program timed-prime-test in SICP section 1.2
   exercise 1.22 *)

(** Exercise 1.22: search for the three smallest primes above each of
    four thresholds, and time each test. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

val square : int -> int
val smallest_divisor : int -> int
val is_prime : int -> bool

(** [timed_prime_test n] is [None] when [n] is not prime, or
    [Some elapsed] with the [Sys.time] cost of confirming it is. *)
val timed_prime_test : int -> float option

(** [search_for_primes start count] is the [count] smallest primes at
    or above [start]. *)
val search_for_primes : int -> int -> int list

(** [ex_1_22 ()] maps each of 1000, 10,000, 100,000, and 1,000,000 to
    the three smallest primes above it. *)
val ex_1_22 : unit -> (int * int list) list
