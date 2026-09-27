(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program repeated in SICP section 1.3
   exercise 1.43 *)

(** Exercise 1.43: [repeated f n] is [f] composed with itself [n]
    times. The stub raises [Sicp_common.Pending.Pending_solution]
    until it is solved. *)

(** [repeated f n x] is [f (f (... (f x)))], [f] applied [n] times;
    [n] must be positive. *)
val repeated : ('a -> 'a) -> int -> 'a -> 'a

(** [ex_1_43 ()] is [(repeated square 2) 5]. *)
val ex_1_43 : unit -> int
