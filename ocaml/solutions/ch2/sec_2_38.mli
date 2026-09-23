(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.38: fold-left versus fold-right. *)

val fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a
val ex_2_38 : unit -> float * float * int list * int list
