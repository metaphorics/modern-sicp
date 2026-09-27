(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program pascal in SICP section 1.2 exercise
   1.12 *)

(** Reference solution of exercise 1.12. *)

(** [pascal row col] is the entry at [row], [col] (both from 0): 0 off
    either edge, 1 on an edge, otherwise the sum of the two entries
    above it. *)
val pascal : int -> int -> int

(** [ex_1_12 row] is the whole row: [ex_1_12 4] is [[1; 4; 6; 4; 1]]. *)
val ex_1_12 : int -> int list
