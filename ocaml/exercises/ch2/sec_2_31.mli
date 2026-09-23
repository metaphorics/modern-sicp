(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.31:
   tree-map *)

type tree =
  | Leaf of int
  | Node of tree list

(** [ex_2_31_tree_map f tree] applies [f] to every leaf of [tree]. *)
val ex_2_31_tree_map : (int -> int) -> tree -> tree

(** [square_tree] defined through [ex_2_31_tree_map]. *)
val ex_2_31_square_tree : tree -> tree
