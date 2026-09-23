(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.35: count-leaves as an accumulation. *)

type tree =
  | Leaf of int
  | Node of tree list

val ex_2_35 : tree -> int
