(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program timed-prime-test in SICP section 1.2
   exercise 1.22 *)

(** Reference solution of exercise 1.22; the timing discussion is in
    [ex_1_22.md]. *)

val square : int -> int
val smallest_divisor : int -> int
val is_prime : int -> bool

(** [timed_prime_test n] is [None] for a composite, [Some elapsed] for
    a prime. *)
val timed_prime_test : int -> float option

(** [search_for_primes start count] is the [count] smallest primes at
    or above [start]. *)
val search_for_primes : int -> int -> int list

(** [ex_1_22 ()] maps 1000, 10,000, 100,000, and 1,000,000 to their
    three smallest primes above. *)
val ex_1_22 : unit -> (int * int list) list
