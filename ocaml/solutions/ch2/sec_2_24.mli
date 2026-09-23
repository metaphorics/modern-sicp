(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.24: the tree reading of 1, (2, (3, 4)). *)

type tree =
  | Leaf of int
  | Node of tree list

val ex_2_24 : unit -> tree
