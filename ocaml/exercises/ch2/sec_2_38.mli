(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.38:
   fold-left versus fold-right *)

(** [fold_left op initial sequence] combines elements working from the
    initial value leftwards through the sequence. *)
val fold_left : ('a -> 'b -> 'a) -> 'a -> 'b list -> 'a

(** The four values the statement asks for: the two [ /. ] folds, then
    the two [cons] folds over the integer list. *)
val ex_2_38 : unit -> float * float * int list * int list
