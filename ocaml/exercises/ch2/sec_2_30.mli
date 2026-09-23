(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.30:
   square-tree *)

type tree =
  | Leaf of int
  | Node of tree list

(** Squares every leaf, written without higher-order procedures. *)
val ex_2_30_direct : tree -> tree

(** The same, defined with [List.map] and recursion. *)
val ex_2_30_map : tree -> tree
