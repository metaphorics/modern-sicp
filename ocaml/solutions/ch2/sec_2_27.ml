(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.27: deep-reverse. *)

type tree =
  | Leaf of int
  | Node of tree list

let rec ex_2_27 tree =
  match tree with
  | Leaf n -> Leaf n
  | Node children -> Node (List.rev (List.map ex_2_27 children))
;;
