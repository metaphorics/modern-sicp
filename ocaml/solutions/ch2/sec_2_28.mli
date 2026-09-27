(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.28: fringe. *)

type tree =
  | Leaf of int
  | Node of tree list

val ex_2_28 : tree -> int list
