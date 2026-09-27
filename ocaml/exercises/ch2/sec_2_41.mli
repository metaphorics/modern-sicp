(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.41:
   ordered triples with a given sum *)

(** [ex_2_41 n s] is the list of the triples of distinct positive
    integers [i], [j], [k], all at most [n] and summing to [s]. *)
val ex_2_41 : int -> int -> (int * int * int) list
