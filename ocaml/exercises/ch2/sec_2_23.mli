(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.23:
   for-each *)

(** [ex_2_23 proc items] applies [proc] to every element of [items],
    left to right, and returns unit. *)
val ex_2_23 : ('a -> unit) -> 'a list -> unit
