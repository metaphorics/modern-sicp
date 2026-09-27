(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program smallest-divisor in SICP section 1.2
   exercise 1.21 *)

(** Reference solution of exercise 1.21. *)

val square : int -> int
val find_divisor : int -> int -> int

(** [smallest_divisor n] is [n]'s smallest divisor greater than 1. *)
val smallest_divisor : int -> int

(** [ex_1_21 ()] is [(199, 1999, 7)]. *)
val ex_1_21 : unit -> int * int * int
