(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.31: tree-map. *)

type tree =
  | Leaf of int
  | Node of tree list

val ex_2_31_tree_map : (int -> int) -> tree -> tree
val ex_2_31_square_tree : tree -> tree
