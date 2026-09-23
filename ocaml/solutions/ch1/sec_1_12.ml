(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program pascal in SICP section 1.2 exercise
   1.12 *)

(** Exercise 1.12: an entry off either edge of the triangle is 0; an
    edge entry ([col] = 0 or [col] = [row]) is 1; an interior entry is
    the sum of the two entries above it, a tree-recursive process
    directly off the triangle's own definition. *)

let rec pascal row col =
  if col < 0 || col > row
  then 0
  else if col = 0 || col = row
  then 1
  else pascal (row - 1) (col - 1) + pascal (row - 1) col
;;

let ex_1_12 row = List.init (row + 1) (fun col -> pascal row col)
