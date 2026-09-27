(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.27: deep-reverse. *)

type tree =
  | Leaf of int
  | Node of tree list

val ex_2_27 : tree -> tree
