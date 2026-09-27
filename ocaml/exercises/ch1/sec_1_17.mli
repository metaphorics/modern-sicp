(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program * in SICP section 1.2 exercise 1.17 *)

(** Exercise 1.17: a logarithmic-step multiplication by doubling and
    halving. The stubs raise [Sicp_common.Pending.Pending_solution]
    until they are solved. *)

(** [times a b] is the book's linear multiplication by repeated
    addition, named so as not to shadow [( * )]. *)
val times : int -> int -> int

val double : int -> int
val halve : int -> int

(** [fast_mult a b] is [a * b] using [double] and [halve], a
    logarithmic number of steps. *)
val fast_mult : int -> int -> int

(** [ex_1_17 a b] is [fast_mult a b]. *)
val ex_1_17 : int -> int -> int
