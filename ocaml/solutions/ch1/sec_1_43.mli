(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program repeated in SICP section 1.3
   exercise 1.43 *)

(** Reference solution of exercise 1.43. *)

val repeated : ('a -> 'a) -> int -> 'a -> 'a

(** [ex_1_43 ()] is [625]. *)
val ex_1_43 : unit -> int
