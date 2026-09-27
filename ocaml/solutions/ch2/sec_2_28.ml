(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.28: fringe. *)

type tree =
  | Leaf of int
  | Node of tree list

let rec ex_2_28 tree =
  match tree with
  | Leaf n -> [ n ]
  | Node children -> List.concat_map ex_2_28 children
;;
