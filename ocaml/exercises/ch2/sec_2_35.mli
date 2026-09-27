(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.35:
   count-leaves as an accumulation *)

type tree =
  | Leaf of int
  | Node of tree list

(** [ex_2_35 tree] counts the leaves of [tree] in the shape the
    statement's template prescribes: an accumulation over a map. *)
val ex_2_35 : tree -> int
