(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program pascal in SICP section 1.2 exercise
   1.12 *)

(** Exercise 1.12: elements of Pascal's triangle by a recursive
    process. The stubs raise [Sicp_common.Pending.Pending_solution]
    until they are solved. *)

(** [pascal row col] is the entry at [row], [col], both from 0. *)
val pascal : int -> int -> int

(** [ex_1_12 row] is the whole row, left edge to right edge. *)
val ex_1_12 : int -> int list
