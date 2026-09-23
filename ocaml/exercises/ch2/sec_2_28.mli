(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.28:
   fringe *)

type tree =
  | Leaf of int
  | Node of tree list

(** [ex_2_28 tree] is the list of the leaves of [tree], left to
    right. *)
val ex_2_28 : tree -> int list
