(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program smallest-divisor in SICP section 1.2
   exercise 1.23 *)

(** Reference solution of exercise 1.23; the timing comparison is in
    [ex_1_23.md]. *)

val square : int -> int
val next_test_divisor : int -> int
val find_divisor_fast : int -> int -> int

(** [smallest_divisor_fast n] skips even test divisors above 2. *)
val smallest_divisor_fast : int -> int

val find_divisor_naive : int -> int -> int

(** [smallest_divisor_naive n] is the unskipped search of @ref{1.2.6},
    kept here only to check [smallest_divisor_fast] agrees with it. *)
val smallest_divisor_naive : int -> int

(** [ex_1_23 n] is [smallest_divisor_fast n]. *)
val ex_1_23 : int -> int
