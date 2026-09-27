(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The recursive form mirrors the book's sigma notation: add the cube of
   [a] and recurse on the rest of the range. The fold form builds the
   range with [List.init] and lets the fold do the adding. The empty
   range [a > b] is [0] in both, and the recursion depth is [b - a], so
   very wide ranges should prefer the fold, whose helper is tail
   recursive. *)

let rec sum_cubes_rec a b = if a > b then 0 else (a * a * a) + sum_cubes_rec (a + 1) b

let sum_cubes_fold a b =
  List.fold_left
    (fun total k -> total + (k * k * k))
    0
    (List.init (max 0 (b - a + 1)) (fun i -> a + i))
;;
