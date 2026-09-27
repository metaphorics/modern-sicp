(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program compose in SICP section 1.3
   exercise 1.42 *)

(** Exercise 1.42: [compose f g] is [f] after [g]. The stub raises
    [Sicp_common.Pending.Pending_solution] until it is solved. *)

(** [compose f g x] is [f (g x)]. *)
val compose : ('b -> 'c) -> ('a -> 'b) -> 'a -> 'c

(** [ex_1_42 ()] is [(compose square inc) 6]. *)
val ex_1_42 : unit -> int
