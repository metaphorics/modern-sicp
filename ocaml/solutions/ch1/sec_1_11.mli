(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program f in SICP section 1.2 exercise 1.11 *)

(** Reference solution of exercise 1.11. *)

(** [f_recursive n] computes the book's [f] by a tree-recursive
    process. *)
val f_recursive : int -> int

(** [f_iterative n] computes the same function by an iterative process
    that slides a three-value window forward. *)
val f_iterative : int -> int

(** [ex_1_11 n] is [(f_recursive n, f_iterative n)]; the two agree on
    every [n]. *)
val ex_1_11 : int -> int * int
