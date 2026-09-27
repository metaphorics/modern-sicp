(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.63 *)

(** A binary tree of numbers, as in [Sicp_ch2.Sec_2_3.Tree_set]. *)
type tree =
  | Empty
  | Node of tree * int * tree

val tree_to_list_1 : tree -> int list
val tree_to_list_2 : tree -> int list

(** [leaf entry] is a tree with no subtrees, holding [entry]. *)
val leaf : int -> tree

(** The three trees of Figure 2.16, each representing
    [\{1, 3, 5, 7, 9, 11\}] with a different shape. *)
val figure_2_16_a : tree

val figure_2_16_b : tree
val figure_2_16_c : tree

(** [ex_2_63 ()] is [(tree_to_list_1 t, tree_to_list_2 t)] for each of
    the three Figure 2.16 trees, in order. *)
val ex_2_63 : unit -> (int list * int list) list
