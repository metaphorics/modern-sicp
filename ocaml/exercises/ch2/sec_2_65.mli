(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.65 *)

type tree =
  | Empty
  | Node of tree * int * tree

val tree_to_list : tree -> int list
val list_to_tree : int list -> tree

(** [union_set tree1 tree2] is the balanced tree over the union of the
    elements of [tree1] and [tree2], computed in [Theta(n)]. *)
val union_set : tree -> tree -> tree

(** [intersection_set tree1 tree2] is the balanced tree over the
    intersection of the elements of [tree1] and [tree2], computed in
    [Theta(n)]. *)
val intersection_set : tree -> tree -> tree

(** [ex_2_65 ()] is the union and the intersection of the tree of
    [\{1; 3; 5; 7\}] and the tree of [\{3; 4; 5\}]. *)
val ex_2_65 : unit -> tree * tree
