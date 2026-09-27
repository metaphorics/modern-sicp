(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.64 *)

type tree =
  | Empty
  | Node of tree * int * tree

(** [partial_tree elts n] is a balanced tree over the first [n]
    elements of [elts], paired with the elements after them.
    [invalid_arg] when [elts] holds fewer than [n] elements. *)
val partial_tree : int list -> int -> tree * int list

(** [list_to_tree elements] is the balanced tree over every element of
    the ordered list [elements]. *)
val list_to_tree : int list -> tree

(** [ex_2_64 ()] is [list_to_tree \[1; 3; 5; 7; 9; 11\]]. *)
val ex_2_64 : unit -> tree
