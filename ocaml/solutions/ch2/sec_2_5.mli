(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cons in SICP section 2.1
   exercise 2.5 *)

(** Exercise 2.5: pairs represented as [2^a * 3^b]. *)

val cons : int -> int -> int
val car : int -> int
val cdr : int -> int
val ex_2_05 : int -> int -> bool
