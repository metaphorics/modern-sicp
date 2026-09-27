(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.35: count-leaves as an accumulation. *)

type tree =
  | Leaf of int
  | Node of tree list

let rec fringe tree =
  match tree with
  | Leaf n -> [ n ]
  | Node children -> List.concat_map fringe children
;;

let ex_2_35 tree = List.fold_left ( + ) 0 (List.map (fun _ -> 1) (fringe tree))
