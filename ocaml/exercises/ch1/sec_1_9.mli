(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program plus in SICP section 1.2 exercise 1.9 *)

(** Exercise 1.9: illustrate the substitution model for the two
    definitions of [plus], and say which is iterative. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

(** [plus_deferred a b] is the book's first [plus]. *)
val plus_deferred : int -> int -> int

(** [plus_tail a b] is the book's second [plus]. *)
val plus_tail : int -> int -> int

(** [ex_1_09 ()] is [(plus_deferred 4 5, plus_tail 4 5)]. *)
val ex_1_09 : unit -> int * int
