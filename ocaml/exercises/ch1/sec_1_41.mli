(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program double in SICP section 1.3
   exercise 1.41 *)

(** Exercise 1.41: [double f] applies [f] twice. The stub raises
    [Sicp_common.Pending.Pending_solution] until it is solved. *)

(** [double f x] is [f (f x)]. *)
val double : ('a -> 'a) -> 'a -> 'a

(** [ex_1_41 ()] is [(((double (double double)) inc) 5)]. *)
val ex_1_41 : unit -> int
