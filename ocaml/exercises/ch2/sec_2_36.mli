(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.36:
   accumulate-n *)

(** [ex_2_36 op init seqs] combines the first elements of every
    sequence in [seqs] with [op] and [init], then the second elements,
    and so on. All sequences are assumed the same length. *)
val ex_2_36 : ('a -> 'b -> 'b) -> 'b -> 'a list list -> 'b list
