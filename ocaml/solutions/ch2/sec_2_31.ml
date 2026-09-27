(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.31: tree-map. *)

type tree =
  | Leaf of int
  | Node of tree list

let rec ex_2_31_tree_map f tree =
  match tree with
  | Leaf n -> Leaf (f n)
  | Node children ->
    Node (List.map (fun sub_tree -> ex_2_31_tree_map f sub_tree) children)
;;

let ex_2_31_square_tree tree = ex_2_31_tree_map (fun n -> n * n) tree
