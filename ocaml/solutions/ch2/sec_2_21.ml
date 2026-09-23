(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.21: square-list two ways. *)

let rec ex_2_21_direct items =
  match items with
  | [] -> []
  | car :: cdr -> (car * car) :: ex_2_21_direct cdr
;;

let ex_2_21_map items = List.map (fun x -> x * x) items
