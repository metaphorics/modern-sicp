(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.30: square-tree. *)

type tree =
  | Leaf of int
  | Node of tree list

let rec ex_2_30_direct tree =
  match tree with
  | Leaf n -> Leaf (n * n)
  | Node children ->
    Node
      (List.fold_right (fun sub_tree acc -> ex_2_30_direct sub_tree :: acc) children [])
;;

let rec ex_2_30_map tree =
  match tree with
  | Leaf n -> Leaf (n * n)
  | Node children -> Node (List.map square_subtree children)

and square_subtree sub_tree =
  match sub_tree with
  | Leaf n -> Leaf (n * n)
  | Node _ -> ex_2_30_map sub_tree
;;
