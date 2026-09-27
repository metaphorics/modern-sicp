(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.35 *)

(** Exercise 3.35: the squarer as a primitive constraint. *)

open Sicp_ch3.Sec_3_3

(** [isqrt v] is the integer square root of [v]. *)
val isqrt : int -> int

(** [squarer a b] is a primitive constraint with a.in * a.in = b:
    setting b computes a as the integer square root, setting a computes
    b, and a negative b is a programming error. *)
val squarer : Constraints.connector -> Constraints.connector -> Constraints.constraint_

(** [ex_3_35 ()] is [(the a computed when b is set to 49, the b
    computed when, after a forget, a is set to 6)]. *)
val ex_3_35 : unit -> int * int

(** [negative_raises ()] is whether setting b to a negative value
    raises [Invalid_argument]. *)
val negative_raises : unit -> bool
