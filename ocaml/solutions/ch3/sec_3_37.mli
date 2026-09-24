(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.37 *)

(** Exercise 3.37: expression-style constraint combinators. *)

open Sicp_ch3.Sec_3_3

(** [c_add], [c_sub], [c_mul], and [c_div] each return a fresh
    connector constrained to the sum, difference, product, or quotient
    of the arguments; [cv value] is a constant-valued connector. *)
val c_add : Constraints.connector -> Constraints.connector -> Constraints.connector

val c_sub : Constraints.connector -> Constraints.connector -> Constraints.connector
val c_mul : Constraints.connector -> Constraints.connector -> Constraints.connector
val c_div : Constraints.connector -> Constraints.connector -> Constraints.connector
val cv : int -> Constraints.connector

(** [celsius_fahrenheit_converter x] is the one-expression network:
    F = ((9 * C) / 5) + 32 with integer connectors. *)
val celsius_fahrenheit_converter : Constraints.connector -> Constraints.connector

(** [ex_3_37 ()] is [(the Fahrenheit when C = 25, the Celsius when F =
    212, the difference 10 - 4 through c-sub, the x solved when the
    difference is set to 3 with y = 4)]. *)
val ex_3_37 : unit -> int * int * int * int
