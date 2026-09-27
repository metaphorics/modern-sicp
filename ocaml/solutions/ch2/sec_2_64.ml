(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.64 *)

(** Exercise 2.64: [list_to_tree] converts an ordered list into a
    balanced binary tree. [partial_tree elts n] builds a balanced tree
    over the first [n] elements of [elts] and returns it paired with
    the elements after them, a tuple where the book's Scheme returns a
    [cons] pair. It puts roughly half of the [n] elements
    ([(n - 1) / 2]) in the left subtree, takes the next element as the
    entry, and puts the rest in the right subtree, so the tree stays
    balanced by construction; there is no rebalancing step to design.
    Each of the [n] elements is visited once to build one node, so
    [list_to_tree] runs in [Theta(n)]. *)

type tree =
  | Empty
  | Node of tree * int * tree

let rec partial_tree elts n =
  if n = 0
  then Empty, elts
  else (
    let left_size = (n - 1) / 2 in
    let left_tree, non_left_elts = partial_tree elts left_size in
    match non_left_elts with
    | [] -> invalid_arg "partial_tree: fewer than n elements"
    | this_entry :: rest ->
      let right_size = n - (left_size + 1) in
      let right_tree, remaining_elts = partial_tree rest right_size in
      Node (left_tree, this_entry, right_tree), remaining_elts)
;;

let list_to_tree elements = fst (partial_tree elements (List.length elements))

(** [ex_2_64 ()] is [list_to_tree] applied to [\[1; 3; 5; 7; 9; 11\]],
    the book's example list. *)
let ex_2_64 () = list_to_tree [ 1; 3; 5; 7; 9; 11 ]
