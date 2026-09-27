(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.30: square-tree. *)

type tree =
  | Leaf of int
  | Node of tree list

val ex_2_30_direct : tree -> tree
val ex_2_30_map : tree -> tree
