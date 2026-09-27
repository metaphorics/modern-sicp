(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.32: the set of all subsets. *)

let rec ex_2_32 s =
  match s with
  | [] -> [ [] ]
  | car :: cdr ->
    let rest = ex_2_32 cdr in
    rest @ List.map (fun subset -> car :: subset) rest
;;
