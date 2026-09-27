(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.65 *)

(** Exercise 2.65: [Theta(n)] [union_set] and [intersection_set] for
    sets represented as balanced binary trees, built from Exercise
    2.63's [tree_to_list_2] and Exercise 2.64's [list_to_tree]: convert
    both trees to ordered lists, merge or intersect the lists in
    [Theta(n)] the way the ordered-list representation does, and
    convert the result back to a balanced tree. Every step is
    [Theta(n)], so the whole operation is too. *)

type tree =
  | Empty
  | Node of tree * int * tree

let rec tree_to_list tree =
  let rec copy_to_list tree result_list =
    match tree with
    | Empty -> result_list
    | Node (left, entry, right) ->
      copy_to_list left (entry :: copy_to_list right result_list)
  in
  copy_to_list tree []

and partial_tree elts n =
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

let rec merge_lists list1 list2 =
  match list1, list2 with
  | [], l | l, [] -> l
  | first1 :: rest1, first2 :: rest2 ->
    if first1 = first2
    then first1 :: merge_lists rest1 rest2
    else if first1 < first2
    then first1 :: merge_lists rest1 list2
    else first2 :: merge_lists list1 rest2
;;

let rec intersect_lists list1 list2 =
  match list1, list2 with
  | [], _ | _, [] -> []
  | first1 :: rest1, first2 :: rest2 ->
    if first1 = first2
    then first1 :: intersect_lists rest1 rest2
    else if first1 < first2
    then intersect_lists rest1 list2
    else intersect_lists list1 rest2
;;

let union_set tree1 tree2 =
  list_to_tree (merge_lists (tree_to_list tree1) (tree_to_list tree2))
;;

let intersection_set tree1 tree2 =
  list_to_tree (intersect_lists (tree_to_list tree1) (tree_to_list tree2))
;;

(** [ex_2_65 ()] is the union and the intersection of the tree of
    [\{1; 3; 5; 7\}] and the tree of [\{3; 4; 5\}]. *)
let ex_2_65 () =
  let t1 = list_to_tree [ 1; 3; 5; 7 ] in
  let t2 = list_to_tree [ 3; 4; 5 ] in
  union_set t1 t2, intersection_set t1 t2
;;
