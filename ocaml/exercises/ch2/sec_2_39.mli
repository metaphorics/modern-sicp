(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.39:
   reverse via folds *)

(** [reverse] defined through the right-to-left fold. *)
val ex_2_39_right : 'a list -> 'a list

(** [reverse] defined through the left-to-right fold. *)
val ex_2_39_left : 'a list -> 'a list
