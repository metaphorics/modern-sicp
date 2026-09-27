(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.63 *)

(** Exercise 2.63: two procedures that flatten a binary tree into a
    list. [tree_to_list_1] appends the flattened left subtree, the
    entry, and the flattened right subtree; [tree_to_list_2] instead
    carries an accumulator, consing the entry on the way down so no
    intermediate list is ever built by [@]. Both produce the sorted
    element list for every tree, but only the second is [Theta(n)]:
    [tree_to_list_1]'s [@] costs the size of its left argument at every
    node, which sums to [Theta(n log n)] for a balanced tree. *)

type tree =
  | Empty
  | Node of tree * int * tree

let rec tree_to_list_1 = function
  | Empty -> []
  | Node (left, entry, right) -> tree_to_list_1 left @ (entry :: tree_to_list_1 right)
;;

let tree_to_list_2 tree =
  let rec copy_to_list tree result_list =
    match tree with
    | Empty -> result_list
    | Node (left, entry, right) ->
      copy_to_list left (entry :: copy_to_list right result_list)
  in
  copy_to_list tree []
;;

(* The three trees of Figure 2.16, all representing \{1, 3, 5, 7, 9, 11\}. *)
let leaf entry = Node (Empty, entry, Empty)
let figure_2_16_a = Node (Node (leaf 1, 3, leaf 5), 7, Node (Empty, 9, leaf 11))
let figure_2_16_b = Node (leaf 1, 3, Node (leaf 5, 7, Node (Empty, 9, leaf 11)))
let figure_2_16_c = Node (Node (leaf 1, 3, Empty), 5, Node (leaf 7, 9, leaf 11))

(** [ex_2_63 ()] is [tree_to_list_1] and [tree_to_list_2] applied to
    each of the three Figure 2.16 trees. *)
let ex_2_63 () =
  List.map
    (fun t -> tree_to_list_1 t, tree_to_list_2 t)
    [ figure_2_16_a; figure_2_16_b; figure_2_16_c ]
;;
