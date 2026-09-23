(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program compose in SICP section 1.3
   exercise 1.42 *)

(** Reference solution of exercise 1.42. *)

val compose : ('b -> 'c) -> ('a -> 'b) -> 'a -> 'c

(** [ex_1_42 ()] is [49]. *)
val ex_1_42 : unit -> int
