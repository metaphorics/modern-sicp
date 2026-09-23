(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.43:
   the swapped-mapping queens *)

(** The reference queens of Exercise 2.42, kept here so Louis's
    version can be compared against it. *)
val queens : int -> (int * int) list list

(** [louis_queens board_size] solves the puzzle with the nested
    mappings interchanged as the statement shows, computing the same
    solutions far more slowly. *)
val louis_queens : int -> (int * int) list list

(** Whether Louis's version and the reference agree on a small board,
    demonstrating that his version works but is slow. *)
val ex_2_43 : unit -> bool
