(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.18: reversing a list. *)

let ex_2_18 items =
  let rec rev acc items =
    match items with
    | [] -> acc
    | car :: cdr -> rev (car :: acc) cdr
  in
  rev [] items
;;
